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
- [>] 3.4 `rodar`: ciclo com `while` e `match` das instruções
- [ ] 3.5 `mostrar`: imprime as variáveis do quadro
- [ ] 3.6 teste final: `programa_reto` termina com `a=5`, `b=10`, `c=15`

## Etapa 4 · Saltos — falta

- [ ] 4.1 `Goto`
- [ ] 4.2 `If` e `IfFalse`
- [ ] 4.3 teste: `if` sem `else` e `while` montados à mão
- [ ] 4.4 teste: rótulo inventado derruba com a frase de rótulo ausente

## Etapa 5 · Chamadas — falta

- [ ] 5.1 `Quadro` ganha argumentos, retorno e destino
- [ ] 5.2 fila de `param` na `Maquina`
- [ ] 5.3 `Param`
- [ ] 5.4 `Call` (confere `n`, empilha quadro, salta)
- [ ] 5.5 `Return` (desempilha, grava no destino, volta)
- [ ] 5.6 `Parametro(n)` em `get_valor`
- [ ] 5.7 teste: `programa_soma` termina com `r = 15` no quadro de `main`

## Etapa 6 · Léxico e leitor (`lexer.rs`, `leitor.rs`) — falta

- [ ] 6.1 fichas com linha de origem
- [ ] 6.2 leitor: fichas viram `Vec<Instrucao>`
- [ ] 6.3 teste: o texto do `soma`/`main` gera o mesmo `Vec` feito à mão

## Etapa 7 · Arquivo (`main.rs`) — falta

- [ ] 7.1 `std::env::args` e `std::fs::read_to_string`
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
