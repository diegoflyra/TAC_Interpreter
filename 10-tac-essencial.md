# 10 — Geração de TAC: a essência

Este documento resume, em alto nível, como as construções de MiniC são traduzidas para código de três endereços (TAC). A ideia não é mostrar o código Rust do gerador, mas o **esquema** de cada tradução: o que é gerado, em que ordem, e por quê.

---

## A ideia central

Toda a geração de TAC se apoia em uma única função, `traduzir`, com dois "formatos" de resultado:

```py
# Expressões produzem um VALOR: onde o resultado ficou + como calculá-lo
(t, ins) = traduzir(expr)     # t   : endereço (variável, constante ou temporário)
                              # ins : instruções que deixam o valor em t

# Comandos produzem apenas EFEITOS: uma sequência de instruções
ins = traduzir(cmd)
```

A tradução é **composicional**: o código de uma construção é obtido colando o código de suas partes, usando apenas três "colas":

| Cola | Papel |
| :---- | :---- |
| concatenação | executar uma parte depois da outra |
| rótulos (`L1:`) | marcar pontos para onde se pode saltar |
| saltos (`goto`, `ifFalse ... goto`) | decidir qual parte executa a seguir |

Cada construção abaixo é só uma receita diferente de combinar essas três colas.

---

## Expressões aritméticas

Em MiniC:

```c
x = a + b * c;
```

Esquema:

```py
(t1, ins1) = traduzir(e1)
(t2, ins2) = traduzir(e2)
t          = novo_temporario()
# código: ins1 ; ins2 ; t = t1 op t2     → resultado em t
```

Em TAC:

```nasm
temp1 = b * c
temp2 = a + temp1
x = temp2
```

&nbsp;

**Essência:** primeiro calculamos os operandos (recursivamente), depois combinamos seus endereços em **um** novo temporário. Cada instrução tem no máximo um operador — daí "três endereços".

---

## `&&` com curto-circuito

Em MiniC:

```c
x = e1 && e2;
```

Esquema:

```py
(t1, ins1) = traduzir(e1)
(t2, ins2) = traduzir(e2)
```

Em TAC:

```nasm
    ins1
    ifFalse t1 goto LFalse    ; e1 falso? nem avalia e2
    ins2
    ifFalse t2 goto LFalse
    x = true
    goto LFim
LFalse:
    x = false
LFim:
```

&nbsp;

**Essência:**

- `ins2` só aparece **depois** do primeiro teste — é isso que implementa o curto-circuito. Se `e2` tiver efeitos colaterais (ex.: uma chamada), eles não acontecem quando `e1` é falso.  
- Os dois testes saltam para o **mesmo** rótulo `LFalse`: basta um falso para o resultado ser falso.  
- O `goto LFim` impede que o caminho verdadeiro "caia" no bloco falso.

O `||` é o espelho: troque `ifFalse` por `if` e `LFalse` por `LTrue`.

```nasm
    ins1
    if t1 goto LTrue          ; e1 verdadeiro? nem avalia e2
    ins2
    if t2 goto LTrue
    x = false
    goto LFim
LTrue:
    x = true
LFim:
```

&nbsp;

---

## `if` / `else`

Em MiniC:

```c
if (cond) { bloco1 } else { bloco2 }
```

Esquema:

```py
(t1, ins1) = traduzir(cond)
```

Em TAC:

```nasm
    ins1
    ifFalse t1 goto LElse
    traduzir(bloco1)
    goto LFim                 ; pula o else
LElse:
    traduzir(bloco2)
LFim:
```

&nbsp;

**Essência:** a condição vira **um** salto condicional que escolhe entre dois trechos lineares de código; o `goto LFim` garante que apenas um deles execute.

Sem `else`, o esquema encolhe — `LElse` e `LFim` passam a ser o mesmo ponto:

```nasm
    ins1
    ifFalse t1 goto LFim
    traduzir(bloco1)
LFim:
```

&nbsp;

---

## `while`

Em MiniC:

```c
while (cond) { bloco1 }
```

Esquema:

```py
(t1, ins1) = traduzir(cond)
```

Em TAC:

```nasm
LLoop:
    ins1                      ; a condição é recalculada a cada volta
    ifFalse t1 goto LFim
    traduzir(bloco1)
    goto LLoop                ; o "laço" é só um salto para trás
LFim:
```

&nbsp;

**Essência:** é um `if` sem `else` com um salto de volta ao início. Note que `ins1` fica **dentro** do laço (depois de `LLoop`): se ficasse antes, a condição seria avaliada uma única vez.

---

## Funções e chamadas

Em MiniC:

```c
int soma(int x, int y) {
    return x + y;
}

void main() {
    int a = 5;
    int b = 10;
    int r = soma(a, b);
}
```

Em TAC:

```nasm
soma:
    x = @parameter0           ; parâmetros formais recebem os reais
    y = @parameter1
    temp1 = x + y
    return temp1

main:
    a = 5
    b = 10
    param a                   ; empilha os argumentos, na ordem
    param b
    temp2 = call soma, 2      ; chama soma com 2 argumentos
    r = temp2
```

&nbsp;

**Essência:**

- **Definição:** uma função é um rótulo com seu nome, seguido de uma cópia por parâmetro (`x = @parameter0`, no estilo do Jimple) e do código do corpo. A partir daí, o corpo usa `x` e `y` como variáveis comuns.  
- **Chamada:** cada argumento é traduzido como uma expressão qualquer; seus endereços são passados com `param`, e `call f, n` transfere o controle, informando quantos argumentos foram passados.  
- **Retorno:** `return t` devolve o endereço calculado pela expressão; na chamada, `temp = call ...` captura esse valor. Uma chamada usada como comando (procedimento) simplesmente omite o `temp =`.  
- **Programa:** é a concatenação do código de todas as funções.

---

## Padrões que se repetem

1. **Primeiro calcule, depois use.** Antes de qualquer teste ou atribuição, aparece o `ins` da subexpressão. O endereço `t` só é válido depois dele.  
2. **Rótulos sempre novos.** Cada tradução de `if`, `while` ou `&&` pede rótulos frescos ao ambiente (`L1`, `L2`, ...). Se dois `while` aninhados usassem o mesmo `LFim`, o `goto` do laço interno sairia do laço errado.  
3. **Temporários sempre novos.** Pelo mesmo motivo: cada valor intermediário ganha seu próprio nome, e nada é sobrescrito por acidente.  
4. **Fluxo estruturado → saltos.** `if`, `while`, `&&` e `||` deixam de existir em TAC; sobram apenas sequências lineares de instruções, costuradas por rótulos e saltos.  
5. **A recursão faz o trabalho pesado.** Cada receita só sabe lidar com **sua** construção; o restante (`traduzir(bloco1)`, `traduzir(e1)`) fica a cargo das chamadas recursivas.

---

## Para pensar

- Como ficaria o esquema de `x = !e`? Precisa de saltos?  
- Como traduzir `if (e1 && e2) { ... }` **sem** materializar o booleano em `x`, saltando diretamente para `LElse`?  
- No `while`, é possível testar a condição no **fim** do laço e economizar um `goto` por iteração? O que muda no esquema?