# Roteiro do interpretador de TAC

Este arquivo é o contrato do projeto. Você escreve o código. O instrutor explica o fundamento quando ele aparece, revisa a forma e ajuda a ler o erro do compilador. O instrutor não implementa o interpretador, a menos que você peça uma função específica.

A referência do que o TAC contém é `10-tac-essencial.md`. Esse texto descreve a tradução de MiniC para TAC. Não descreve MiniC como entrada deste programa. A entrada do interpretador é o TAC já gerado.

Não há, neste repositório, uma gramática oficial da cadeira nem uma instrução de escrita. Onde este roteiro lê uma forma de linha a partir dos exemplos, ela está marcada como **leitura dos exemplos**. Se o formato oficial divergir, a leitura perde e o formato oficial fica.

## O que este programa é

Um executável só. Duas entradas. A mesma máquina.

- Com caminho: `cargo run -- programa.txt` lê o arquivo inteiro, executa e escreve no terminal só o que o TAC mandar imprimir.
- Sem caminho: modo interativo. O texto acumula num buffer. `run` trata o buffer como se fosse o arquivo. `reset` limpa o buffer.

O executor não abre arquivo e não sabe se a frase veio do `.txt` ou do teclado.

## O que este programa não é

- Não é um compilador de MiniC. `if`, `while`, `&&` e `||` da linguagem de origem já viraram rótulo e salto no texto que recebemos.
- Não faz checagem de tipos. Outro compilador já fez isso. O interpretador confia no TAC.
- Não tem analisador de precedência. Cada linha de TAC já tem no máximo um operador.
- Não otimiza e não gera código de novo.

## Léxico, sintaxe e semântica

Vale a pena ter analisador léxico. Não vale a pena ter análise sintática de expressão, nem análise semântica.

O léxico transforma o texto em fichas: identificador, número, `=`, `+`, `:`, vírgula, e palavras como `goto`, `if`, `ifFalse`, `param`, `call`, `return`, `true`, `false`. Os exemplos indentam e deixam linhas em branco. Espaço em branco não carrega significado. `ifFalse` é uma ficha só, não `if` seguido de `False`.

O que vem depois do léxico é um reconhecedor de linha, não um parser de linguagem. A linha já é plana. O reconhecedor olha as primeiras fichas e escolhe uma variante de `Instrucao`. Uma gramática com recursão e precedência resolveria um problema que o gerador de TAC já resolveu. O resultado desse reconhecedor é o próprio programa: um `Vec` de instruções. Não há uma árvore sintática intermediária.

Análise semântica, no sentido da cadeira (declaração, tipo, escopo da linguagem de origem), não entra. Três coisas parecem semântica e não são:

| Tarefa | Onde mora | Por quê |
| :---- | :---- | :---- |
| Rótulo `L` existe? | Montagem do programa | É o trabalho de um montador: o mapa nome → índice. Um salto para rótulo ausente é erro de execução. |
| Variável foi atribuída antes do uso? | Máquina, na hora de ler o nome | Erro de execução, não uma passagem anterior. |
| `true` somado com `5`? | Não se verifica | O TAC recebido já foi checado. |

## Conjunto de instruções

Lido dos exemplos de `10-tac-essencial.md`. Provisório até o formato oficial.

| Forma no texto | Papel |
| :---- | :---- |
| `soma:` | Rótulo. Função e `LFim` são a mesma coisa: um nome colado a um índice. |
| `x = temp2` | Cópia. |
| `a = 5` | Cópia de constante. |
| `x = true` e `x = false` | Cópia de constante booleana. Ver a decisão abaixo. |
| `x = @parameter0` | Cópia do argumento número 0 da chamada atual. |
| `temp1 = b * c` | Uma operação, três endereços. Nos exemplos aparecem `+` e `*`. |
| `goto LFim` | Salto. |
| `if t1 goto LTrue` | Salta se o valor for verdadeiro. |
| `ifFalse t1 goto LElse` | Salta se o valor for falso. |
| `param a` | Enfileira um argumento para o próximo `call`. |
| `temp2 = call soma, 2` | Chama `soma` com 2 argumentos e guarda o retorno em `temp2`. |
| `call soma, 2` | Chamada de procedimento. O documento descreve essa forma; o exemplo concreto sempre guarda o retorno. |
| `return temp1` | Devolve o valor e volta para quem chamou. |

`&&`, `||`, `if/else` e `while` não são instruções. O documento mostra o TAC que sobra depois deles.

Fora dos exemplos, e portanto fora da primeira versão:

- escrita (`print` ou equivalente): o documento nunca imprime nada;
- operador unário, inclusive `!`, que o próprio documento deixa como pergunta;
- comparação (`<`, `==`, …) como operador de TAC: o que aparece é `if` / `ifFalse` sobre um valor já calculado;
- `-` e `/`: entram no enum quando um exemplo os usar. Divisão por zero já está na lista de erros de execução, então `Div` tem lugar reservado no desenho, sem precisar aparecer no primeiro programa.

### Decisão: um só tipo de valor, `i64`

`true` vale 1 e `false` vale 0. `if` salta quando o valor é diferente de zero. `ifFalse` salta quando o valor é zero. O texto continua escrevendo `true` e `false`; quem traduz essas palavras em 1 e 0 é o reconhecedor de linha.

Um enum de valor só entra se o formato oficial tiver um dado que não caiba em `i64`. Até lá, `Operando` carrega três formas de *endereço escrito na instrução*, e a máquina só guarda `i64`:

- nome de variável;
- constante inteira;
- `@parameterN`.

### Decisão: a execução começa em `main`

No exemplo, `soma:` vem antes de `main:`. Começar no índice 0 executaria o corpo de `soma` como se fosse o programa. A montagem procura o rótulo `main` e o contador de programa nasce aí. `main` ausente é erro de execução.

## A máquina

Uma função, no TAC deste documento, é um rótulo mais as cópias `x = @parameterN` mais o corpo. Dois funções podem usar o mesmo nome de parâmetro. Uma função pode chamar a si mesma. Um mapa único de nomes quebra os dois casos. A máquina usa uma pilha de quadros.

Cada quadro tem:

- o mapa nome → `i64` daquela ativação;
- o índice para onde `return` volta;
- o nome que recebe o valor devolvido, se a chamada era `temp = call …`. Na chamada de procedimento esse nome está vazio.

A máquina, além da pilha, tem:

- o contador de programa;
- a fila de `param`, esvaziada por `call`.

`param a` seguido de `param b` significa `@parameter0 = a` e `@parameter1 = b`. A fila preserva essa ordem. `call f, n` confere se há `n` argumentos. Número errado é erro de execução.

O ciclo, com o programa já montado:

1. Empilha um quadro para `main` e põe o contador no índice de `main`.
2. Lê a instrução nesse índice.
3. Executa.
4. Se a instrução não saltou e não era `return`, o contador avança um.
5. Para quando o quadro de `main` faz `return`, ou quando o contador sai do vetor. O exemplo de `main` não tem `return`; cair fora do vetor no quadro inicial encerra em silêncio.

Efeito de cada instrução:

| Instrução | Efeito |
| :---- | :---- |
| Rótulo | Nenhum. O contador avança. |
| Cópia | Calcula o operando e grava no nome, no quadro atual. |
| Binária | Lê os dois operandos, aplica o operador, grava no destino. |
| `goto` | Contador recebe o índice do rótulo. |
| `if` / `ifFalse` | Decide com zero ou não zero e, se for o caso, faz o mesmo que `goto`. |
| `param` | Calcula o operando agora e põe o `i64` na fila. |
| `call` | Confere `n`, empilha um quadro novo com o endereço de volta, salta para o rótulo da função. |
| `return` | Calcula o valor, desempilha, grava no destino de quem chamou, contador volta. |

Ler um nome ausente no quadro atual é erro. `@parameterN` com `N` fora da lista desta chamada é erro. `goto` ou `call` para rótulo ausente é erro. Divisão por zero é erro. A primeira versão pode usar `panic!` com uma frase curta. `Result` é a ferramenta rústica para o mesmo problema sem derrubar o processo; ela aparece quando a primeira versão já executar um exemplo, não antes.

Não há variáveis globais neste TAC. Nome se lê e se escreve no quadro do topo.

## As duas frentes

O modo interativo não executa linha a linha. No `while` do documento, `ifFalse t1 goto LFim` cita um rótulo que ainda não apareceu. Um `goto LLoop` cita um rótulo que já passou. Os dois têm de existir antes de qualquer salto. Por isso o buffer inteiro é que vira programa.

Comandos do modo interativo, reconhecidos pela frente, antes do léxico. Não são TAC.

| Linha | Efeito |
| :---- | :---- |
| `run` | Lê o buffer, monta, executa. |
| `reset` | Esvazia o buffer. |
| `lista` | Mostra o buffer, para você ver o que será executado. |
| `ambiente` | Mostra o quadro de `main` depois do último `run`. |
| qualquer outra | Acrescenta ao buffer, inclusive linha em branco. |

`lista` e `ambiente` existem porque o documento não tem instrução de escrita. Sem elas, `run` de um arquivo correto não mostra nada. Quando a cadeira definir o `print`, a saída oficial do modo arquivo continua sendo só o que essa instrução escrever. `ambiente` permanece comando da frente, não instrução de TAC.

O modo arquivo não aceita `run`, `reset`, `lista` nem `ambiente`. Um `.txt` é só TAC.

## Arquivos

Tudo continua num binário, sem dependências. Cada arquivo nasce na etapa em que o conceito aparece. Até lá, não crie o arquivo vazio.

```text
TAC_Interpreter/
  Cargo.toml
  ROADMAP.md
  10-tac-essencial.md
  exemplos/
    soma.txt
  src/
    main.rs        frente: arquivo ou buffer
    instrucao.rs   enums da instrução e do operando
    programa.rs    vetor, mapa de rótulos, índice de main
    maquina.rs     contador, quadros, fila de param, ciclo
    lexer.rs       texto para fichas
    leitor.rs      fichas para Vec de instruções
    repl.rs        buffer e comandos
```

`main.rs` declara os módulos. Em Rust, `mod instrucao;` procura `src/instrucao.rs`. O que outro arquivo usa precisa ser `pub`. O nome do módulo é o nome do arquivo.

Os exercícios `q1`…`q6` saem do `main` na etapa 1. Enquanto ficarem no arquivo sem serem chamados, o compilador avisa `dead_code`. Esse aviso é a hora de apagá-los.

## Etapas

Cada etapa termina quando o critério de pronto vale e o programa compila com `cargo run`. Não antecipe a etapa seguinte no mesmo arquivo.

### 1. Instrução

Arquivo: `src/instrucao.rs`.

Você escreve `Operando`, `Op` e `Instrucao`. `Op` começa com `Soma` e `Mul`. `Div` e `Sub` podem existir no enum desde já, sem nenhum `match` que os use.

`Instrucao` cobre as formas da tabela: rótulo, cópia, binária, `goto`, `if`, `ifFalse`, `param`, `call`, `return`. Em `call`, o destino é opcional. Em `return`, o valor é obrigatório na primeira versão, porque é o que o exemplo escreve.

No `main`, monte um `vec!` com o programa `soma` / `main` do documento, à mão, sem ler texto. Ainda sem executar.

Fundamento que aparece: enum com dados, `String` possuída, `Option` para o destino ausente de um `call` de procedimento.

Pronto quando: `cargo run` compila e não imprime nada de TAC.

### 2. Programa

Arquivo: `src/programa.rs`.

Uma struct com o vetor, o mapa rótulo → índice e o índice de `main`. Uma função recebe o `Vec<Instrucao>`, percorre uma vez, e preenche o mapa só com as variantes de rótulo. Rótulo repetido é erro. `main` ausente é erro.

Fundamento que aparece: `HashMap<String, usize>`, `use std::collections::HashMap`, inserir e consultar.

Pronto quando: uma função de apoio, chamada pelo `main`, imprime o índice de `main` e o índice de `soma` para o `vec!` da etapa 1. Essa impressão é temporária e sai na etapa 3.

### 3. Máquina reta

Arquivo: `src/maquina.rs`.

Quadro, pilha com um quadro só, contador, ciclo. Implemente rótulo, cópia e binária. Nas outras variantes, `panic!("ainda não")` é honesto: o `match` de Rust exige todas as variantes, e as outras chegam nas etapas 4 e 5.

Comece no índice de `main`. Para o exemplo `soma` / `main`, `main` não chama ninguém ainda: ignore mentalmente as linhas `param` e `call` até a etapa 5, usando um `vec!` reduzido só com `a = 5`, `b = 10` e as cópias que não dependem de chamada. O programa completo volta na etapa 5.

Fundamento que aparece: empréstimo do programa (`&Programa`) enquanto a máquina muda o seu próprio estado; `panic!`.

Pronto quando: o `vec!` reduzido de `main` termina com `a` valendo 5 e `b` valendo 10 no quadro, impressos pela função temporária da etapa 2. Aí essa impressão temporária sai do `main` e fica atrás do comando `ambiente`, mesmo que o REPL ainda não exista: uma função `mostrar` em `maquina.rs`, chamada uma vez pelo `main`, basta até a etapa 8.

### 4. Saltos

Ainda em `maquina.rs`.

`goto`, `if`, `ifFalse`. Rótulo continua sem efeito. Monte à mão o esquema do `if` sem `else` e o esquema do `while`, com operandos concretos no lugar de `ins1` e `traduzir(bloco1)`.

Pronto quando: os dois esquemas terminam no quadro esperado e um rótulo inventado derruba a execução com a frase de rótulo ausente.

### 5. Chamadas

Ainda em `maquina.rs`.

Fila de `param`, empilhar e desempilhar quadro, `@parameterN`, `call` com destino, `return`. Use o programa completo do documento:

```text
soma:
    x = @parameter0
    y = @parameter1
    temp1 = x + y
    return temp1

main:
    a = 5
    b = 10
    param a
    param b
    temp2 = call soma, 2
    r = temp2
```

Pronto quando: depois da execução, no quadro de `main`, `r` vale 15. O quadro de `soma` não existe mais.

### 6. Léxico e leitor

Arquivos: `src/lexer.rs`, `src/leitor.rs`.

O léxico devolve fichas com a linha de origem, para o erro poder dizer onde. O leitor consome fichas e devolve `Vec<Instrucao>`. Uma instrução por linha. Linha em branco desaparece.

Ordem de decisão, em cada linha:

1. identificador seguido de `:` → rótulo;
2. `goto`;
3. `if` ou `ifFalse`;
4. `param`;
5. `return`;
6. `call` → chamada sem destino;
7. identificador seguido de `=` e depois `call` → chamada com destino;
8. identificador seguido de `=` e de um operador no meio → binária;
9. identificador seguido de `=` → cópia.

`true` e `false` viram constante 1 e 0. `@parameter` seguido de um número vira operando de parâmetro. O número do exemplo está colado à palavra (`@parameter0`); trate isso como uma ficha só, ou como `@parameter` e um número, e use a mesma escolha em todos os exemplos.

Pronto quando: o texto do programa `soma` / `main`, escrito numa `String` dentro do `main`, produz o mesmo `Vec` que você montava à mão. Confira pelo mapa de rótulos e por `r = 15` ao executar.

### 7. Arquivo

Em `main.rs`.

Se `std::env::args` trouxer um caminho, leia o arquivo inteiro com a biblioteca padrão, passe a string ao leitor, monte, execute. Sem caminho, caia no comportamento da etapa 6 até a etapa 8 existir.

Fundamento que aparece: `std::env::args`, `std::fs::read_to_string`.

Pronto quando: `cargo run -- exemplos/soma.txt` termina com `r = 15` visível pela função `mostrar`. O arquivo mora em `exemplos/soma.txt` e você o escreve copiando o exemplo do documento.

### 8. Modo interativo

Arquivo: `src/repl.rs`.

Laço que lê uma linha do terminal. Comandos da tabela de frentes. Qualquer outra linha entra no buffer. `run` chama o mesmo leitor, a mesma montagem e a mesma máquina do modo arquivo.

Fundamento que aparece: `std::io::stdin`, laço enquanto houver linha.

Pronto quando: subir sem argumentos, colar o programa `soma` / `main`, `run`, e ver `r = 15` via `ambiente`. `reset` seguido de `lista` mostra buffer vazio. Um `goto` para rótulo que está mais abaixo no buffer funciona, porque o `run` vê o texto todo.

### 9. Corpus

Escreva em `exemplos/`, à mão, versões concretas dos esquemas do documento: `if` com `else`, `if` sem `else`, `while`, `&&`, `||`. Cada arquivo é TAC, não MiniC. Rode pelo modo arquivo.

Pronto quando: os cinco arquivos, mais `soma.txt`, executam e o quadro final é o que o esquema descreve.

## Onde paramos

O projeto fecha na etapa 9, com os dois modos e o corpus tirado de `10-tac-essencial.md`.

Não entra neste corte:

- parser ou interpretador de MiniC;
- checagem de tipos;
- `print`, até você colar o formato;
- unário e comparação como operadores de TAC, até um exemplo os mostrar;
- editor dentro do buffer (apagar linha, reescrever trecho). O buffer é uma lista de linhas. Corrigir é `reset` e colar de novo.

## Perguntas abertas

Estas não travam a etapa 1. Travam só o pedaço nomeado.

1. Qual é a instrução de escrita da cadeira, se houver? Trava a frase "a saída é só o que o TAC imprimir". Até lá, `mostrar` / `ambiente` é o visor.
2. O formato oficial escreve `@parameter0` colado? Trava um detalhe do léxico, na etapa 6.
3. Existe `return` sem valor para `void`? O `main` do exemplo não retorna; ele acaba caindo fora do vetor. Trava uma variante extra de `return`, se um exemplo aparecer.
4. Quais operadores além de `+` e `*` o gerador emite? Trava o crescimento de `Op`, não a forma da instrução binária.
