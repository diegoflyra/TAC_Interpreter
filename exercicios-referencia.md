# Exercícios q1–q8: referência

Soluções dos oito exercícios de Rust feitos antes do interpretador. Servem para consultar, não para copiar: tente escrever a sua e compare depois.

Saída esperada do `main` completo:

```
11  24  5  5  10  9  negativo  zero  positivo  8  7  20
```

## Código completo (compila com `rustc --edition 2024`)

```rust
use std::collections::HashMap;

// ---------- Q1: soma e produto ----------
fn soma(a: i64, b: i64) -> i64 {
    a + b
}

fn mul(a: i64, b: i64) -> i64 {
    a * b
}

fn q1() {
    let x = 8;
    let y = 3;
    println!("{}", soma(x, y)); // 11
    println!("{}", mul(x, y)); // 24
}

// ---------- Q2: valor absoluto ----------
fn abs(a: i64) -> i64 {
    if a < 0 {
        return -a;
    }
    a
}

fn q2() {
    println!("{}", abs(5)); // 5
    println!("{}", abs(-5)); // 5
}

// ---------- Q3: soma de 1 até n ----------
fn soma_ate(n: i64) -> i64 {
    let mut a = 0;
    for i in 1..=n {
        a += i;
    }
    a
}

fn q3() {
    println!("{}", soma_ate(4)); // 10
}

// ---------- Q4: struct Par ----------
struct Par {
    esquerda: i64,
    direita: i64,
}

fn maior(par: Par) -> i64 {
    if par.esquerda > par.direita {
        par.esquerda
    } else {
        par.direita
    }
}

fn q4(a: i64, b: i64) {
    let par = Par { esquerda: a, direita: b };
    println!("{}", maior(par)); // com (4, 9): 9
}

// ---------- Q5: enum sem dados + match ----------
enum Sinal {
    Negativo,
    Zero,
    Positivo,
}

fn classificar(n: i64) -> Sinal {
    if n < 0 {
        Sinal::Negativo
    } else if n == 0 {
        Sinal::Zero
    } else {
        Sinal::Positivo
    }
}

fn imprimir_sinal(s: Sinal) {
    match s {
        Sinal::Negativo => println!("negativo"),
        Sinal::Zero => println!("zero"),
        Sinal::Positivo => println!("positivo"),
    }
}

fn q5(a: i64, b: i64, c: i64) {
    imprimir_sinal(classificar(a));
    imprimir_sinal(classificar(b));
    imprimir_sinal(classificar(c));
}

// ---------- Q6: &Vec<i64> ----------
// &Vec empresta o vetor: depois da chamada, ele continua existindo.
fn soma_v(numeros: &Vec<i64>) -> i64 {
    let mut a = 0;
    for n in numeros {
        a += n; // n é &i64; += aceita a referência
    }
    a
}

fn q6() {
    let v: Vec<i64> = vec![3, 1, 4];
    println!("{}", soma_v(&v)); // 8
}

// ---------- Q7: HashMap ----------
fn ler(mapa: &HashMap<String, i64>, chave: &str) -> i64 {
    // .get devolve Option<&i64>: Some(&valor) ou None
    match mapa.get(chave) {
        Some(v) => *v, // *v pega o i64 de dentro da referência
        None => panic!("ausente"),
    }
}

fn q7() {
    let mut mapa: HashMap<String, i64> = HashMap::new();
    mapa.insert("a".to_string(), 2);
    mapa.insert("b".to_string(), 5);
    println!("{}", ler(&mapa, "a") + ler(&mapa, "b")); // 7
}

// ---------- Q8: enum com dados (miniatura do interpretador) ----------
enum Instrucao {
    Carrega(i64),
    Soma(i64),
    Imprime,
}

fn q8() {
    let mut acc: i64 = 0;
    let v: Vec<Instrucao> = vec![
        Instrucao::Carrega(10),
        Instrucao::Soma(7),
        Instrucao::Soma(3),
        Instrucao::Imprime,
    ];

    for i in v {
        match i {
            Instrucao::Carrega(n) => acc = n,
            Instrucao::Soma(n) => acc += n,
            Instrucao::Imprime => println!("{}", acc), // 20
        }
    }
}

fn main() {
    q1();
    q2();
    q3();
    q4(4, 9);
    q5(-2, 0, 6);
    q6();
    q7();
    q8();
}
```

## O que cada exercício ensina

| Exercício | Ideia | Onde reaparece no interpretador |
| :---- | :---- | :---- |
| Q1 | funções, parâmetros, retorno | tudo |
| Q2 | `if` e a última expressão sem `;` | decisão de `if`/`ifFalse` na máquina |
| Q3 | `for i in 1..=n`, variável `mut` | o ciclo da máquina |
| Q4 | `struct` com campos nomeados | `Programa`, campos de `Instrucao` |
| Q5 | `enum` sem dados + `match` | `Op` (Add, Sub, Mul, Div) |
| Q6 | `&Vec`: emprestar sem consumir | `&Programa` na máquina |
| Q7 | `HashMap`, `Option`, `panic!` | tabela de rótulos, ambiente de variáveis |
| Q8 | `enum` com dados + `match` | `Instrucao` e `Operando` |

## Diferenças em relação ao que você escreveu

- **`return` explícito vs. última expressão.** Você usava `return a + b;`. Aqui aparece `a + b` sem ponto e vírgula. Os dois funcionam; a forma sem `return` é a mais comum em Rust.
- **`else if`** em vez de `else { if ... }` aninhado na `classificar`.
- **`soma_v` recebe `&Vec<i64>`**, como pedia o enunciado do exercício 6, e a chamada passa `&v`. Assim `v` continua válido depois.
- **Q7:** o enunciado pedia a saída `7` (soma de `a` e `b`). O seu imprimia `2` e `5` em linhas separadas, o que também mostra que `ler` funciona.
- **`println!("{}", x)`** com parênteses. `println!{...}` com chaves (que você usou na Q3) também compila, mas ninguém escreve assim.

## Dicas que vieram das dúvidas

- `*v` em `Some(v) => *v`: o `.get` devolve uma **referência** (`&i64`); o `*` pega o valor de dentro.
- `for n in numeros` com `numeros: &Vec<i64>` entrega `&i64`; `a += n` funciona porque `i64` sabe somar com `&i64`.
- `for n in v` (sem `&`) **consome** o vetor; depois do laço, `v` não existe mais.
