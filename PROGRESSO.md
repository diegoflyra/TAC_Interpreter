# Onde estou

Legenda: `[x]` feito · `[>]` em andamento · `[ ]` falta.

As **etapas** (1 a 9) são as do [ROADMAP.md](ROADMAP.md). Os **passos** são as tarefas pequenas dentro de cada etapa. A conversa sempre começa com `Etapa X · Passo Y`.

## Etapa 1 · Instrução (`src/instrucao.rs`) — pronta

- [x] 1.1 `Operando`
- [x] 1.2 `Op`
- [x] 1.3 `Instrucao`
- [x] 1.4 `programa_soma()` montado à mão no `main` (12 instruções)

## Etapa 2 · Programa (`src/programa.rs`) — pronta

- [x] 2.1 `struct Programa` (instrucoes, tabela, inicio)
- [x] 2.2 `Programa::montar` com o laço que preenche a tabela
- [x] 2.3 rótulo repetido é erro; `main` ausente é erro; `inicio` de verdade
- [x] 2.4 `mostrar_indices` imprime `main: 5` e `soma: 0` (temporária)

## Etapa 3 · Máquina reta (`src/maquina.rs`) — em andamento

Só `Rotulo`, `Copia` e `Binaria`. As outras variantes dão `panic!("ainda não")`.

- [x] 3.1 `Quadro`, `Maquina` e `Maquina::montar` (pilha nasce com um quadro)
- [x] 3.2 `get_valor`: `Operando` vira `i64`
- [x] 3.3 `efetuar`: aplica um `Op` (divisão por zero só dentro do `Div`)
- [x] 3.4 `rodar`: ciclo com `while` e `match` das instruções
- [x] 3.5 `mostrar`: imprime as variáveis do quadro
- [x] 3.7 limpeza: `main` fica só com montar, criar máquina, `rodar` e `mostrar` (apagar `mostrar_indices` e os testes temporários)
- [x] 3.6 teste final: `programa_reto` termina com `a=5`, `b=10`, `c=15`

## Etapa 4 · Saltos — pronta

- [x] 4.1 `Goto` (forma A: `self.pc = *i; continue;`)
- [x] 4.2 `If` e `IfFalse`
- [x] 4.3 teste: `if` com e sem `else`, `while`, `&&` e `||` com curto-circuito (escritos por Claude em `src/testes.rs`)
- [x] 4.4 teste: rótulo inventado derruba com a frase de rótulo ausente

Os testes ficam em `src/testes.rs` e rodam com `cargo test` (29 passando ao fim da etapa 4; 36 ao fim do 5.3; 66 ao fim do 5.5; 69 ao fim da etapa 5; 82 com os símbolos; 105 com as palavras; 122 com os números; 139 com o léxico completo; 147 com operando; 154 com operador; 184 com ler_linha; 193 com agrupar; 207 ao fim da etapa 6).

Melhoria opcional: o trecho "procure o rótulo e defina o `pc`" aparece 3 vezes (`Goto`, `If`, `IfFalse`); dá para extrair um método `saltar`.

## Etapa 5 · Chamadas — pronta

- [x] 5.1 `Quadro` ganha argumentos, retorno e destino
- [x] 5.2 fila de `param` na `Maquina`
- [x] 5.3 `Param`
- [x] 5.4 `Call` (confere `n`, empilha quadro, salta)
- [x] 5.5 `Return` (desempilha, grava no destino, volta)
- [x] 5.6 `Parametro(n)` em `get_valor`
- [x] 5.7 teste: `programa_soma` termina com `r = 15` no quadro de `main`

## Etapa 6 · Léxico e leitor (`lexer.rs`, `leitor.rs`) — pronta

- [x] 6.1 fichas com linha de origem (símbolos, palavras, números, @parameterN, linha e erros)
- [x] 6.2 leitor: fichas viram `Vec<Instrucao>` (operando, operador, ler_linha, agrupar, ler_programa)
- [x] 6.3 teste: o texto do `soma`/`main` gera o mesmo `Vec` feito à mão

## Etapa 7 · Arquivo (`main.rs`) — falta

- [>] 7.1 `std::env::args` e `std::fs::read_to_string`
- [ ] 7.2 `exemplos/soma.txt` e teste com `cargo run -- exemplos/soma.txt`

## Etapa 8 · Modo interativo (`repl.rs`) — falta

- [ ] 8.1 laço de leitura do terminal
- [ ] 8.2 comandos `run`, `reset`, `lista`, `ambiente`

## Etapa 9 · Corpus (`exemplos/`) — falta

- [ ] 9.1 cinco exemplos de TAC mais `soma.txt`, executados pelo modo arquivo

## Pendências de limpeza (sem pressa)

- `hash::Hash` sobrando nos `use` de `programa.rs` e `maquina.rs`
- `Some(pos)` no laço de `montar` vira `Some(_)`
- `Parametro(n)` com `n` sem uso vira `Parametro(_)`
- quando a etapa 3 fechar: apagar `mostrar_indices` e as linhas de teste do `main`
