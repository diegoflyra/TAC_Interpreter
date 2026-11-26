// Testes automáticos. Rodam com: cargo test
// Este arquivo só existe na compilação de teste (ver `#[cfg(test)] mod testes;` no main.rs).

use crate::instrucao::{Instrucao, Op, Operando};
use crate::maquina::Maquina;
use crate::programa::Programa;

// ---------- atalhos para montar instruções ----------

fn rot(nome: &str) -> Instrucao {
    Instrucao::Rotulo { nome: nome.to_string() }
}

fn copia(destino: &str, valor: i64) -> Instrucao {
    Instrucao::Copia {
        destino: destino.to_string(),
        fonte: Operando::Constante(valor),
    }
}

fn bin(destino: &str, esq: &str, op: Op, dir: &str) -> Instrucao {
    Instrucao::Binaria {
        destino: destino.to_string(),
        esquerda: Operando::Nome(esq.to_string()),
        op,
        direita: Operando::Nome(dir.to_string()),
    }
}

fn goto(rotulo: &str) -> Instrucao {
    Instrucao::Goto { rotulo: rotulo.to_string() }
}

fn se(cond: &str, rotulo: &str) -> Instrucao {
    Instrucao::If {
        condicao: Operando::Nome(cond.to_string()),
        rotulo: rotulo.to_string(),
    }
}

fn se_falso(cond: &str, rotulo: &str) -> Instrucao {
    Instrucao::IfFalse {
        condicao: Operando::Nome(cond.to_string()),
        rotulo: rotulo.to_string(),
    }
}

fn param_nome(nome: &str) -> Instrucao {
    Instrucao::Param {
        fonte: Operando::Nome(nome.to_string()),
    }
}

fn param_const(valor: i64) -> Instrucao {
    Instrucao::Param {
        fonte: Operando::Constante(valor),
    }
}

fn chama(destino: Option<&str>, funcao: &str, n: usize) -> Instrucao {
    Instrucao::Call {
        destino: destino.map(|d| d.to_string()),
        funcao: funcao.to_string(),
        n,
    }
}

fn ret_nome(nome: &str) -> Instrucao {
    Instrucao::Return {
        valor: Operando::Nome(nome.to_string()),
    }
}

fn ret_const(valor: i64) -> Instrucao {
    Instrucao::Return {
        valor: Operando::Constante(valor),
    }
}

// ---------- executar e consultar ----------

fn executar(instrucoes: Vec<Instrucao>) -> Maquina {
    let p = Programa::montar(instrucoes);
    let mut m = Maquina::montar(p.inicio);
    m.rodar(&p);
    m
}

fn var(m: &Maquina, nome: &str) -> i64 {
    *m.pilha[0]
        .variaveis
        .get(nome)
        .unwrap_or_else(|| panic!("variável {} não existe no quadro", nome))
}

fn tem(m: &Maquina, nome: &str) -> bool {
    m.pilha[0].variaveis.contains_key(nome)
}

// ---------- Etapa 2: Programa ----------

#[test]
fn montar_acha_indices() {
    let p = Programa::montar(vec![rot("soma"), copia("x", 1), rot("main")]);
    assert_eq!(p.tabela["soma"], 0);
    assert_eq!(p.tabela["main"], 2);
    assert_eq!(p.inicio, 2);
}

#[test]
#[should_panic(expected = "rotulo repetido")]
fn montar_rejeita_rotulo_repetido() {
    Programa::montar(vec![rot("main"), rot("main")]);
}

#[test]
#[should_panic(expected = "main ausente")]
fn montar_exige_main() {
    Programa::montar(vec![rot("soma")]);
}

// ---------- Etapa 3: máquina reta ----------

#[test]
fn copia_e_binaria() {
    let m = executar(vec![
        rot("main"),
        copia("a", 5),
        copia("b", 10),
        bin("c", "a", Op::Add, "b"),
    ]);
    assert_eq!(var(&m, "a"), 5);
    assert_eq!(var(&m, "b"), 10);
    assert_eq!(var(&m, "c"), 15);
}

#[test]
fn as_quatro_operacoes() {
    assert_eq!(Maquina::efetuar(&Op::Add, 3, 4), 7);
    assert_eq!(Maquina::efetuar(&Op::Sub, 3, 10), -7);
    assert_eq!(Maquina::efetuar(&Op::Mul, 3, 4), 12);
    assert_eq!(Maquina::efetuar(&Op::Div, 8, 2), 4);
}

#[test]
fn zero_so_e_erro_na_divisao() {
    // somar ou multiplicar por zero é válido
    assert_eq!(Maquina::efetuar(&Op::Add, 5, 0), 5);
    assert_eq!(Maquina::efetuar(&Op::Mul, 5, 0), 0);
}

#[test]
#[should_panic(expected = "divisão por 0")]
fn divisao_por_zero_derruba() {
    Maquina::efetuar(&Op::Div, 8, 0);
}

#[test]
#[should_panic]
fn variavel_ausente_derruba() {
    executar(vec![rot("main"), bin("c", "a", Op::Add, "b")]);
}

// ---------- Etapa 4: saltos ----------

#[test]
fn goto_pula_instrucoes() {
    // main: a = 1; goto LFim; a = 2; LFim: b = 3
    let m = executar(vec![
        rot("main"),
        copia("a", 1),
        goto("LFim"),
        copia("a", 2),
        rot("LFim"),
        copia("b", 3),
    ]);
    assert_eq!(var(&m, "a"), 1, "o goto não pulou 'a = 2'");
    assert_eq!(var(&m, "b"), 3, "a execução não continuou depois do rótulo");
}

#[test]
fn goto_para_rotulo_logo_a_frente() {
    // salto para a instrução imediatamente seguinte ao rótulo
    let m = executar(vec![rot("main"), goto("L"), rot("L"), copia("a", 7)]);
    assert_eq!(var(&m, "a"), 7);
}

#[test]
fn goto_nao_executa_o_que_foi_pulado() {
    let m = executar(vec![
        rot("main"),
        goto("L"),
        copia("pulada", 1),
        rot("L"),
        copia("b", 3),
    ]);
    assert!(!tem(&m, "pulada"));
    assert_eq!(var(&m, "b"), 3);
}

#[test]
#[should_panic(expected = "rotulo inexistente")]
fn goto_para_rotulo_ausente_derruba() {
    executar(vec![rot("main"), goto("LXX")]);
}

// ---------- Etapa 4: if / ifFalse ----------

#[test]
fn if_salta_com_valor_diferente_de_zero() {
    for c in [1, 7, -5] {
        let m = executar(vec![
            rot("main"),
            copia("c", c),
            se("c", "L"),
            copia("pulada", 1),
            rot("L"),
            copia("fim", 1),
        ]);
        assert!(!tem(&m, "pulada"), "c = {} deveria saltar", c);
        assert_eq!(var(&m, "fim"), 1);
    }
}

#[test]
fn if_nao_salta_com_zero() {
    let m = executar(vec![
        rot("main"),
        copia("c", 0),
        se("c", "L"),
        copia("seguiu", 1),
        rot("L"),
        copia("fim", 1),
    ]);
    assert_eq!(var(&m, "seguiu"), 1, "o if com 0 não deveria saltar");
    assert_eq!(var(&m, "fim"), 1);
}

#[test]
fn if_false_salta_com_zero() {
    let m = executar(vec![
        rot("main"),
        copia("c", 0),
        se_falso("c", "L"),
        copia("pulada", 1),
        rot("L"),
        copia("fim", 1),
    ]);
    assert!(!tem(&m, "pulada"), "ifFalse com 0 deveria saltar");
    assert_eq!(var(&m, "fim"), 1);
}

#[test]
fn if_false_nao_salta_com_diferente_de_zero() {
    for c in [1, -3] {
        let m = executar(vec![
            rot("main"),
            copia("c", c),
            se_falso("c", "L"),
            copia("seguiu", 1),
            rot("L"),
        ]);
        assert_eq!(var(&m, "seguiu"), 1, "c = {} não deveria saltar", c);
    }
}

#[test]
fn if_para_rotulo_ausente_derruba_so_quando_salta() {
    // com 0 o if não salta, então o rótulo ausente nunca é consultado
    let m = executar(vec![rot("main"), copia("c", 0), se("c", "LXX"), copia("a", 1)]);
    assert_eq!(var(&m, "a"), 1);
}

#[test]
#[should_panic(expected = "rotulo inexistente")]
fn if_que_salta_para_rotulo_ausente_derruba() {
    executar(vec![rot("main"), copia("c", 1), se("c", "LXX")]);
}

// ---------- Etapa 4: os esquemas do documento (10-tac-essencial.md) ----------

// if / else:
//     ifFalse c goto LElse ; r = 10 ; goto LFim ; LElse: r = 20 ; LFim:
fn programa_if_else(c: i64) -> Vec<Instrucao> {
    vec![
        rot("main"),
        copia("c", c),
        se_falso("c", "LElse"),
        copia("r", 10),
        goto("LFim"),
        rot("LElse"),
        copia("r", 20),
        rot("LFim"),
    ]
}

#[test]
fn if_else_escolhe_o_ramo_verdadeiro() {
    assert_eq!(var(&executar(programa_if_else(1)), "r"), 10);
}

#[test]
fn if_else_escolhe_o_ramo_falso() {
    assert_eq!(var(&executar(programa_if_else(0)), "r"), 20);
}

// if sem else:
//     ifFalse c goto LFim ; r = 10 ; LFim:
fn programa_if_sem_else(c: i64) -> Vec<Instrucao> {
    vec![
        rot("main"),
        copia("c", c),
        copia("r", 0),
        se_falso("c", "LFim"),
        copia("r", 10),
        rot("LFim"),
    ]
}

#[test]
fn if_sem_else_executa_o_bloco() {
    assert_eq!(var(&executar(programa_if_sem_else(1)), "r"), 10);
}

#[test]
fn if_sem_else_pula_o_bloco() {
    assert_eq!(var(&executar(programa_if_sem_else(0)), "r"), 0);
}

// while (a != 0) { s = s + a; a = a - 1 }
//     LLoop: ifFalse a goto LFim ; s = s + a ; a = a - um ; goto LLoop ; LFim:
fn programa_while(n: i64) -> Vec<Instrucao> {
    vec![
        rot("main"),
        copia("a", n),
        copia("s", 0),
        copia("um", 1),
        rot("LLoop"),
        se_falso("a", "LFim"),
        bin("s", "s", Op::Add, "a"),
        bin("a", "a", Op::Sub, "um"),
        goto("LLoop"),
        rot("LFim"),
    ]
}

#[test]
fn while_soma_de_3_ate_1() {
    let m = executar(programa_while(3));
    assert_eq!(var(&m, "s"), 6); // 3 + 2 + 1
    assert_eq!(var(&m, "a"), 0);
}

#[test]
fn while_soma_de_10_ate_1() {
    assert_eq!(var(&executar(programa_while(10)), "s"), 55);
}

#[test]
fn while_com_zero_voltas() {
    let m = executar(programa_while(0));
    assert_eq!(var(&m, "s"), 0);
}

// e1 && e2 com curto-circuito:
//     ifFalse t1 goto LFalse ; efeito = 1 ; ifFalse t2 goto LFalse ;
//     x = 1 ; goto LFim ; LFalse: x = 0 ; LFim:
fn programa_e(t1: i64, t2: i64) -> Vec<Instrucao> {
    vec![
        rot("main"),
        copia("t1", t1),
        copia("t2", t2),
        se_falso("t1", "LFalse"),
        copia("avaliou_e2", 1),
        se_falso("t2", "LFalse"),
        copia("x", 1),
        goto("LFim"),
        rot("LFalse"),
        copia("x", 0),
        rot("LFim"),
    ]
}

#[test]
fn e_logico_tabela_verdade() {
    assert_eq!(var(&executar(programa_e(1, 1)), "x"), 1);
    assert_eq!(var(&executar(programa_e(1, 0)), "x"), 0);
    assert_eq!(var(&executar(programa_e(0, 1)), "x"), 0);
    assert_eq!(var(&executar(programa_e(0, 0)), "x"), 0);
}

#[test]
fn e_logico_curto_circuito() {
    // e1 falso: e2 nem é avaliado
    assert!(!tem(&executar(programa_e(0, 1)), "avaliou_e2"));
    // e1 verdadeiro: e2 é avaliado
    assert!(tem(&executar(programa_e(1, 1)), "avaliou_e2"));
}

// e1 || e2 com curto-circuito:
//     if t1 goto LTrue ; efeito = 1 ; if t2 goto LTrue ;
//     x = 0 ; goto LFim ; LTrue: x = 1 ; LFim:
fn programa_ou(t1: i64, t2: i64) -> Vec<Instrucao> {
    vec![
        rot("main"),
        copia("t1", t1),
        copia("t2", t2),
        se("t1", "LTrue"),
        copia("avaliou_e2", 1),
        se("t2", "LTrue"),
        copia("x", 0),
        goto("LFim"),
        rot("LTrue"),
        copia("x", 1),
        rot("LFim"),
    ]
}

#[test]
fn ou_logico_tabela_verdade() {
    assert_eq!(var(&executar(programa_ou(1, 1)), "x"), 1);
    assert_eq!(var(&executar(programa_ou(1, 0)), "x"), 1);
    assert_eq!(var(&executar(programa_ou(0, 1)), "x"), 1);
    assert_eq!(var(&executar(programa_ou(0, 0)), "x"), 0);
}

#[test]
fn ou_logico_curto_circuito() {
    assert!(!tem(&executar(programa_ou(1, 0)), "avaliou_e2"));
    assert!(tem(&executar(programa_ou(0, 0)), "avaliou_e2"));
}

// ---------- Etapa 5: param ----------

#[test]
fn param_enfileira_na_ordem() {
    let m = executar(vec![
        rot("main"),
        copia("a", 5),
        copia("b", 10),
        param_nome("a"),
        param_nome("b"),
    ]);
    assert_eq!(m.fila_parametros, vec![5, 10]);
}

#[test]
fn param_guarda_o_valor_e_nao_o_nome() {
    // a muda depois do param; a fila precisa continuar com o valor antigo
    let m = executar(vec![
        rot("main"),
        copia("a", 5),
        param_nome("a"),
        copia("a", 99),
    ]);
    assert_eq!(m.fila_parametros, vec![5]);
    assert_eq!(var(&m, "a"), 99);
}

#[test]
fn param_aceita_constante() {
    let m = executar(vec![rot("main"), param_const(7), param_const(-2)]);
    assert_eq!(m.fila_parametros, vec![7, -2]);
}

#[test]
fn param_nao_cria_variavel() {
    let m = executar(vec![rot("main"), copia("a", 1), param_nome("a")]);
    assert_eq!(m.pilha.len(), 1, "param não pode empilhar quadro");
    assert!(m.pilha[0].argumentos.is_empty());
}

#[test]
fn param_repetido_acumula() {
    // o mesmo operando duas vezes gera duas entradas
    let m = executar(vec![rot("main"), copia("a", 4), param_nome("a"), param_nome("a")]);
    assert_eq!(m.fila_parametros, vec![4, 4]);
}

#[test]
#[should_panic(expected = "variável")]
fn param_de_variavel_ausente_derruba() {
    executar(vec![rot("main"), param_nome("naoexiste")]);
}

#[test]
fn fila_nasce_vazia() {
    let p = Programa::montar(vec![rot("main")]);
    let m = Maquina::montar(p.inicio);
    assert!(m.fila_parametros.is_empty());
}

// ---------- Etapa 5: call (ainda sem return) ----------
//
// A função fica no FIM do programa, para a execução cair fora do vetor
// depois do salto. Assim dá para inspecionar o estado logo após o call.
//
//   0: main:
//   1:   a = 5
//   2:   b = 10
//   3:   param a
//   4:   param b
//   5:   temp2 = call soma, 2     ← o quadro novo volta para 6
//   6:   r = temp2
//   7: soma:

fn programa_so_call(destino: Option<&str>) -> Vec<Instrucao> {
    vec![
        rot("main"),
        copia("a", 5),
        copia("b", 10),
        param_nome("a"),
        param_nome("b"),
        chama(destino, "soma", 2),
        copia("r", 0),
        rot("soma"),
    ]
}

#[test]
fn call_empilha_um_quadro() {
    let m = executar(programa_so_call(Some("temp2")));
    assert_eq!(m.pilha.len(), 2);
}

#[test]
fn call_passa_os_argumentos_na_ordem() {
    let m = executar(programa_so_call(Some("temp2")));
    assert_eq!(m.pilha[1].argumentos, vec![5, 10]);
}

#[test]
fn call_guarda_o_ponto_de_retorno() {
    // o call está na posição 5, então o retorno é 6
    let m = executar(programa_so_call(Some("temp2")));
    assert_eq!(m.pilha[1].retorno, 6);
}

#[test]
fn call_guarda_o_destino_do_retorno() {
    let m = executar(programa_so_call(Some("temp2")));
    assert_eq!(m.pilha[1].destino, Some("temp2".to_string()));
}

#[test]
fn call_de_procedimento_nao_tem_destino() {
    let m = executar(programa_so_call(None));
    assert_eq!(m.pilha[1].destino, None);
}

#[test]
fn call_esvazia_a_fila() {
    let m = executar(programa_so_call(Some("temp2")));
    assert!(m.fila_parametros.is_empty());
}

#[test]
fn call_comeca_com_variaveis_vazias() {
    // as variáveis de main não vazam para o quadro da função
    let m = executar(programa_so_call(Some("temp2")));
    assert!(m.pilha[1].variaveis.is_empty());
}

#[test]
fn call_nao_mexe_nas_variaveis_do_chamador() {
    let m = executar(programa_so_call(Some("temp2")));
    assert_eq!(m.pilha[0].variaveis["a"], 5);
    assert_eq!(m.pilha[0].variaveis["b"], 10);
}

#[test]
fn call_salta_para_a_funcao_e_nao_executa_o_resto_de_main() {
    // 'r = 0' (posição 6) vem depois do call e não deve rodar antes do return
    let m = executar(programa_so_call(Some("temp2")));
    assert!(!tem(&m, "r"));
}

#[test]
fn call_sem_argumentos() {
    let m = executar(vec![rot("main"), chama(None, "f", 0), rot("f")]);
    assert_eq!(m.pilha.len(), 2);
    assert!(m.pilha[1].argumentos.is_empty());
}

#[test]
fn dois_calls_nao_herdam_argumentos_um_do_outro() {
    let m = executar(vec![
        rot("main"),
        param_const(1),
        chama(None, "f", 1),
        rot("f"),
    ]);
    // quem chamou f deixou a fila vazia
    assert!(m.fila_parametros.is_empty());
    assert_eq!(m.pilha[1].argumentos, vec![1]);
}

#[test]
#[should_panic(expected = "argumentos")]
fn call_com_menos_argumentos_que_o_declarado_derruba() {
    executar(vec![rot("main"), param_const(1), chama(None, "f", 2), rot("f")]);
}

#[test]
#[should_panic(expected = "argumentos")]
fn call_com_mais_argumentos_que_o_declarado_derruba() {
    executar(vec![
        rot("main"),
        param_const(1),
        param_const(2),
        chama(None, "f", 1),
        rot("f"),
    ]);
}

#[test]
#[should_panic]
fn call_para_funcao_inexistente_derruba() {
    executar(vec![rot("main"), chama(None, "naoexiste", 0)]);
}

// ---------- Etapa 5: return ----------
//
// As funções ficam antes de main (como no documento). Nenhum teste aqui usa
// @parameter: isso é o passo 5.6.

#[test]
fn return_grava_o_valor_no_destino_do_chamador() {
    // f: v = 7 ; return v
    // main: temp = call f, 0 ; r = temp
    let m = executar(vec![
        rot("f"),
        copia("v", 7),
        ret_nome("v"),
        rot("main"),
        chama(Some("temp"), "f", 0),
        bin("r", "temp", Op::Add, "temp"), // r = temp + temp, prova que temp existe e o pc voltou
    ]);
    assert_eq!(var(&m, "temp"), 7);
    assert_eq!(var(&m, "r"), 14);
}

#[test]
fn return_desempilha_o_quadro_da_funcao() {
    let m = executar(vec![
        rot("f"),
        ret_const(1),
        rot("main"),
        chama(Some("t"), "f", 0),
    ]);
    assert_eq!(m.pilha.len(), 1, "o quadro de f deveria ter sumido");
}

#[test]
fn return_volta_para_a_instrucao_depois_do_call() {
    // se o pc não voltasse, 'depois' nunca seria gravado
    let m = executar(vec![
        rot("f"),
        ret_const(1),
        rot("main"),
        chama(Some("t"), "f", 0),
        copia("depois", 1),
    ]);
    assert_eq!(var(&m, "depois"), 1);
}

#[test]
fn return_nao_executa_o_que_vem_depois_dele_na_funcao() {
    let m = executar(vec![
        rot("f"),
        ret_const(1),
        copia("nunca", 1),
        rot("main"),
        chama(Some("t"), "f", 0),
    ]);
    assert!(!tem(&m, "nunca"));
}

#[test]
fn return_de_constante() {
    let m = executar(vec![
        rot("f"),
        ret_const(42),
        rot("main"),
        chama(Some("r"), "f", 0),
    ]);
    assert_eq!(var(&m, "r"), 42);
}

#[test]
fn return_negativo() {
    let m = executar(vec![
        rot("f"),
        ret_const(-9),
        rot("main"),
        chama(Some("r"), "f", 0),
    ]);
    assert_eq!(var(&m, "r"), -9);
}

#[test]
fn procedimento_nao_grava_nada_mas_volta() {
    // call sem destino: o valor é descartado, e a execução continua
    let m = executar(vec![
        rot("f"),
        copia("g", 1),
        ret_nome("g"),
        rot("main"),
        chama(None, "f", 0),
        copia("depois", 5),
    ]);
    assert_eq!(var(&m, "depois"), 5, "procedimento não voltou ao chamador");
    assert!(!tem(&m, "g"), "variável local da função vazou para main");
    assert_eq!(m.pilha.len(), 1);
}

#[test]
fn variaveis_locais_da_funcao_nao_vazam() {
    let m = executar(vec![
        rot("f"),
        copia("local", 3),
        ret_nome("local"),
        rot("main"),
        chama(Some("r"), "f", 0),
    ]);
    assert!(!tem(&m, "local"));
    assert_eq!(var(&m, "r"), 3);
}

#[test]
fn funcao_nao_altera_variavel_de_mesmo_nome_do_chamador() {
    // main tem a = 1; f define a = 99 no próprio quadro
    let m = executar(vec![
        rot("f"),
        copia("a", 99),
        ret_nome("a"),
        rot("main"),
        copia("a", 1),
        chama(Some("r"), "f", 0),
    ]);
    assert_eq!(var(&m, "a"), 1, "f sobrescreveu a variável de main");
    assert_eq!(var(&m, "r"), 99);
}

#[test]
fn duas_chamadas_da_mesma_funcao() {
    let m = executar(vec![
        rot("f"),
        copia("v", 7),
        ret_nome("v"),
        rot("main"),
        chama(Some("t1"), "f", 0),
        chama(Some("t2"), "f", 0),
        bin("r", "t1", Op::Add, "t2"),
    ]);
    assert_eq!(var(&m, "r"), 14);
    assert_eq!(m.pilha.len(), 1);
}

#[test]
fn chamada_dentro_de_chamada() {
    // g devolve 3; f chama g e devolve o que g devolveu; main chama f
    let m = executar(vec![
        rot("g"),
        ret_const(3),
        rot("f"),
        chama(Some("x"), "g", 0),
        ret_nome("x"),
        rot("main"),
        chama(Some("r"), "f", 0),
    ]);
    assert_eq!(var(&m, "r"), 3);
    assert_eq!(m.pilha.len(), 1);
}

#[test]
fn chamadas_aninhadas_em_tres_niveis() {
    // h -> 1 ; g = h + 1 ; f = g + 1 ; main chama f  =>  3
    let m = executar(vec![
        rot("h"),
        ret_const(1),
        rot("g"),
        chama(Some("a"), "h", 0),
        copia("um", 1),
        bin("b", "a", Op::Add, "um"),
        ret_nome("b"),
        rot("f"),
        chama(Some("a"), "g", 0),
        copia("um", 1),
        bin("b", "a", Op::Add, "um"),
        ret_nome("b"),
        rot("main"),
        chama(Some("r"), "f", 0),
    ]);
    assert_eq!(var(&m, "r"), 3);
    assert_eq!(m.pilha.len(), 1);
}

#[test]
fn return_de_main_encerra_a_execucao() {
    let m = executar(vec![
        rot("main"),
        copia("a", 1),
        ret_nome("a"),
        copia("nunca", 1),
    ]);
    // o quadro de main saiu da pilha, então não há mais variáveis para consultar;
    // a prova de que 'nunca = 1' não rodou é o pc ter ido direto para o fim (4)
    assert!(m.pilha.is_empty(), "o quadro de main deveria ter saído");
    assert_eq!(m.pc, 4, "a execução deveria ter parado no return de main");
}

#[test]
fn return_le_a_variavel_antes_de_remover_o_quadro() {
    // se o quadro fosse removido antes do cálculo, 'local' não existiria
    let m = executar(vec![
        rot("f"),
        copia("local", 8),
        ret_nome("local"),
        rot("main"),
        chama(Some("r"), "f", 0),
    ]);
    assert_eq!(var(&m, "r"), 8);
}

#[test]
#[should_panic(expected = "variável")]
fn return_de_variavel_ausente_derruba() {
    executar(vec![
        rot("f"),
        ret_nome("naoexiste"),
        rot("main"),
        chama(Some("r"), "f", 0),
    ]);
}

#[test]
fn call_dentro_de_while() {
    // soma 1 + 1 + 1 chamando f (que devolve 1) três vezes
    let m = executar(vec![
        rot("f"),
        ret_const(1),
        rot("main"),
        copia("i", 3),
        copia("s", 0),
        copia("um", 1),
        rot("LLoop"),
        se_falso("i", "LFim"),
        chama(Some("t"), "f", 0),
        bin("s", "s", Op::Add, "t"),
        bin("i", "i", Op::Sub, "um"),
        goto("LLoop"),
        rot("LFim"),
    ]);
    assert_eq!(var(&m, "s"), 3);
    assert_eq!(m.pilha.len(), 1);
}

// ---------- Etapa 5: parâmetros e o programa do documento ----------

#[test]
fn programa_soma_do_documento() {
    let m = executar(crate::programa_soma());
    assert_eq!(m.pilha.len(), 1, "o quadro de soma deveria ter sumido");
    assert_eq!(var(&m, "r"), 15);
    assert_eq!(var(&m, "temp2"), 15);
    assert_eq!(var(&m, "a"), 5);
    assert_eq!(var(&m, "b"), 10);
    assert!(!tem(&m, "x"), "x é local de soma");
    assert!(!tem(&m, "temp1"), "temp1 é local de soma");
    assert!(m.fila_parametros.is_empty());
}

#[test]
#[should_panic(expected = "parametro")]
fn parametro_fora_da_lista_derruba() {
    // f lê @parameter1, mas main passou só 1 argumento
    executar(vec![
        rot("f"),
        Instrucao::Copia {
            destino: "y".to_string(),
            fonte: Operando::Parametro(1),
        },
        ret_nome("y"),
        rot("main"),
        param_const(1),
        chama(Some("r"), "f", 1),
    ]);
}

#[test]
fn recursao_fatorial_de_4() {
    // fat(n): se n == 0 retorna 1; senão retorna n * fat(n - 1)
    // n é testado como "zero ou não zero", sem operador de comparação
    let m = executar(vec![
        rot("fat"),
        Instrucao::Copia {
            destino: "n".to_string(),
            fonte: Operando::Parametro(0),
        },
        se("n", "Lrec"),
        ret_const(1),
        rot("Lrec"),
        copia("um", 1),
        bin("m", "n", Op::Sub, "um"),
        param_nome("m"),
        chama(Some("f"), "fat", 1),
        bin("r", "n", Op::Mul, "f"),
        ret_nome("r"),
        rot("main"),
        copia("quatro", 4),
        param_nome("quatro"),
        chama(Some("res"), "fat", 1),
    ]);
    assert_eq!(var(&m, "res"), 24);
    assert_eq!(m.pilha.len(), 1);
}

// ---------- Etapa 6: léxico, símbolos ----------

use crate::lexer::{lexar, Ficha, Variante};

fn f(v: Variante, linha: usize) -> Ficha {
    Ficha { palavra: v, linha }
}

#[test]
fn lexar_os_sete_simbolos() {
    let fichas = lexar("= + - * / : ,");
    assert_eq!(
        fichas,
        vec![
            f(Variante::Igual, 1),
            f(Variante::Mais, 1),
            f(Variante::Menos, 1),
            f(Variante::Vezes, 1),
            f(Variante::DivididoPor, 1),
            f(Variante::DoisPontos, 1),
            f(Variante::Virgula, 1),
        ]
    );
}

#[test]
fn lexar_simbolos_colados() {
    assert_eq!(lexar("=+-").len(), 3);
}

#[test]
fn lexar_ignora_espacos() {
    let fichas = lexar("     =      +   ");
    assert_eq!(fichas, vec![f(Variante::Igual, 1), f(Variante::Mais, 1)]);
}

#[test]
fn lexar_texto_vazio() {
    assert!(lexar("").is_empty());
}

#[test]
fn lexar_so_espacos() {
    assert!(lexar("     ").is_empty());
}

#[test]
fn lexar_conta_as_linhas() {
    // linhas em branco contam, mas não geram ficha
    let fichas = lexar("=\n+\n\n-");
    assert_eq!(
        fichas,
        vec![
            f(Variante::Igual, 1),
            f(Variante::Mais, 2),
            f(Variante::Menos, 4),
        ]
    );
}

#[test]
fn lexar_linhas_comecam_em_1() {
    assert_eq!(lexar(":")[0].linha, 1);
}

#[test]
fn lexar_fim_de_linha_do_windows() {
    // \r\n não pode virar caractere inválido
    let fichas = lexar("=\r\n+");
    assert_eq!(fichas, vec![f(Variante::Igual, 1), f(Variante::Mais, 2)]);
}

#[test]
fn lexar_tab_e_ignorado() {
    let fichas = lexar("\t=\t+");
    assert_eq!(fichas, vec![f(Variante::Igual, 1), f(Variante::Mais, 1)]);
}

#[test]
fn lexar_indentacao_com_espacos() {
    // os exemplos do documento indentam com 4 espaços
    let fichas = lexar("    = ,");
    assert_eq!(fichas.len(), 2);
}

#[test]
#[should_panic(expected = "caractere inválido")]
fn lexar_caractere_invalido_derruba() {
    lexar("= # +");
}

#[test]
#[should_panic(expected = "linha 3")]
fn lexar_erro_diz_a_linha() {
    lexar("=\n+\n#");
}

#[test]
#[should_panic(expected = "'#'")]
fn lexar_erro_diz_o_caractere() {
    lexar("#");
}

// ---------- Etapa 6: léxico, identificadores e palavras reservadas ----------

fn id(nome: &str, linha: usize) -> Ficha {
    Ficha {
        palavra: Variante::Identificador(nome.to_string()),
        linha,
    }
}

fn so_variantes(texto: &str) -> Vec<Variante> {
    lexar(texto).into_iter().map(|x| x.palavra).collect()
}

#[test]
fn lexar_identificador_simples() {
    assert_eq!(lexar("temp"), vec![id("temp", 1)]);
}

#[test]
fn lexar_identificador_de_uma_letra() {
    assert_eq!(lexar("a"), vec![id("a", 1)]);
}

#[test]
fn lexar_identificador_com_digitos() {
    // temp1 é UMA ficha, não 'temp' seguido de '1'
    assert_eq!(lexar("temp1"), vec![id("temp1", 1)]);
    assert_eq!(lexar("x2y3"), vec![id("x2y3", 1)]);
}

#[test]
fn lexar_identificador_no_fim_do_texto() {
    // sem espaço nem quebra de linha depois: o peek devolve None
    assert_eq!(lexar("abc"), vec![id("abc", 1)]);
}

#[test]
fn lexar_cada_palavra_reservada() {
    assert_eq!(so_variantes("goto"), vec![Variante::Goto]);
    assert_eq!(so_variantes("if"), vec![Variante::If]);
    assert_eq!(so_variantes("ifFalse"), vec![Variante::IfFalse]);
    assert_eq!(so_variantes("param"), vec![Variante::Param]);
    assert_eq!(so_variantes("call"), vec![Variante::Call]);
    assert_eq!(so_variantes("return"), vec![Variante::Return]);
    assert_eq!(so_variantes("true"), vec![Variante::True]);
    assert_eq!(so_variantes("false"), vec![Variante::False]);
}

#[test]
fn lexar_todas_as_palavras_reservadas_numa_linha() {
    assert_eq!(
        so_variantes("goto if ifFalse param call return true false"),
        vec![
            Variante::Goto,
            Variante::If,
            Variante::IfFalse,
            Variante::Param,
            Variante::Call,
            Variante::Return,
            Variante::True,
            Variante::False,
        ]
    );
}

#[test]
fn lexar_if_false_e_uma_ficha_so() {
    // 'ifFalse' não é 'if' seguido de 'False'
    assert_eq!(lexar("ifFalse").len(), 1);
    assert_eq!(so_variantes("ifFalse"), vec![Variante::IfFalse]);
}

#[test]
fn lexar_if_separado_de_false_sao_duas_fichas() {
    assert_eq!(
        so_variantes("if False"),
        vec![Variante::If, Variante::Identificador("False".to_string())]
    );
}

#[test]
fn lexar_maiusculas_nao_sao_palavras_reservadas() {
    assert_eq!(lexar("Goto"), vec![id("Goto", 1)]);
    assert_eq!(lexar("IF"), vec![id("IF", 1)]);
    assert_eq!(lexar("Return"), vec![id("Return", 1)]);
    assert_eq!(lexar("True"), vec![id("True", 1)]);
}

#[test]
fn lexar_palavra_que_comeca_com_reservada_e_identificador() {
    assert_eq!(lexar("gotoX"), vec![id("gotoX", 1)]);
    assert_eq!(lexar("iff"), vec![id("iff", 1)]);
    assert_eq!(lexar("ifFalse2"), vec![id("ifFalse2", 1)]);
    assert_eq!(lexar("returns"), vec![id("returns", 1)]);
    assert_eq!(lexar("truex"), vec![id("truex", 1)]);
    assert_eq!(lexar("param1"), vec![id("param1", 1)]);
}

#[test]
fn lexar_palavra_reservada_so_e_reservada_inteira() {
    // 'main' e 'soma' não são palavras reservadas
    assert_eq!(lexar("main"), vec![id("main", 1)]);
    assert_eq!(lexar("soma"), vec![id("soma", 1)]);
}

#[test]
fn lexar_atribuicao_com_operacao() {
    // temp1 = x + y
    assert_eq!(
        lexar("temp1 = x + y"),
        vec![
            id("temp1", 1),
            f(Variante::Igual, 1),
            id("x", 1),
            f(Variante::Mais, 1),
            id("y", 1),
        ]
    );
}

#[test]
fn lexar_simbolos_colados_nas_palavras() {
    // sem espaços: a palavra para no símbolo
    assert_eq!(
        lexar("x=y+z"),
        vec![
            id("x", 1),
            f(Variante::Igual, 1),
            id("y", 1),
            f(Variante::Mais, 1),
            id("z", 1),
        ]
    );
}

#[test]
fn lexar_rotulo() {
    assert_eq!(lexar("soma:"), vec![id("soma", 1), f(Variante::DoisPontos, 1)]);
    assert_eq!(lexar("LFim :"), vec![id("LFim", 1), f(Variante::DoisPontos, 1)]);
}

#[test]
fn lexar_atribuicao_de_booleano() {
    assert_eq!(
        lexar("x = true"),
        vec![id("x", 1), f(Variante::Igual, 1), f(Variante::True, 1)]
    );
    assert_eq!(
        lexar("x = false"),
        vec![id("x", 1), f(Variante::Igual, 1), f(Variante::False, 1)]
    );
}

#[test]
fn lexar_goto() {
    assert_eq!(
        lexar("goto LFim"),
        vec![f(Variante::Goto, 1), id("LFim", 1)]
    );
}

#[test]
fn lexar_if_e_if_false_com_goto() {
    assert_eq!(
        lexar("if t1 goto LTrue"),
        vec![
            f(Variante::If, 1),
            id("t1", 1),
            f(Variante::Goto, 1),
            id("LTrue", 1),
        ]
    );
    assert_eq!(
        lexar("ifFalse t1 goto LFalse"),
        vec![
            f(Variante::IfFalse, 1),
            id("t1", 1),
            f(Variante::Goto, 1),
            id("LFalse", 1),
        ]
    );
}

#[test]
fn lexar_param_e_return() {
    assert_eq!(lexar("param a"), vec![f(Variante::Param, 1), id("a", 1)]);
    assert_eq!(
        lexar("return temp1"),
        vec![f(Variante::Return, 1), id("temp1", 1)]
    );
}

#[test]
fn lexar_call_sem_numero() {
    // a forma com número (call soma, 2) só vale depois do passo 4
    assert_eq!(
        lexar("call soma, a"),
        vec![
            f(Variante::Call, 1),
            id("soma", 1),
            f(Variante::Virgula, 1),
            id("a", 1),
        ]
    );
}

#[test]
fn lexar_varias_linhas_com_indentacao() {
    let fichas = lexar("soma:\n    x = y\n    return x");
    assert_eq!(
        fichas,
        vec![
            id("soma", 1),
            f(Variante::DoisPontos, 1),
            id("x", 2),
            f(Variante::Igual, 2),
            id("y", 2),
            f(Variante::Return, 3),
            id("x", 3),
        ]
    );
}

#[test]
fn lexar_palavras_com_linhas_em_branco_no_meio() {
    let fichas = lexar("a\n\nb");
    assert_eq!(fichas, vec![id("a", 1), id("b", 3)]);
}

#[test]
fn lexar_tabs_e_espacos_repetidos_entre_palavras() {
    assert_eq!(
        so_variantes("goto \t  L"),
        vec![Variante::Goto, Variante::Identificador("L".to_string())]
    );
}

#[test]
#[should_panic(expected = "caractere inválido")]
fn lexar_palavra_seguida_de_caractere_invalido_derruba() {
    lexar("abc#");
}

// ---------- Etapa 6: léxico, números ----------

fn num(n: i64, linha: usize) -> Ficha {
    Ficha {
        palavra: Variante::Numero(n),
        linha,
    }
}

#[test]
fn lexar_numero_de_um_digito() {
    assert_eq!(lexar("5"), vec![num(5, 1)]);
    assert_eq!(lexar("0"), vec![num(0, 1)]);
}

#[test]
fn lexar_numero_de_varios_digitos_e_uma_ficha_so() {
    assert_eq!(lexar("15"), vec![num(15, 1)]);
    assert_eq!(lexar("1234567890"), vec![num(1234567890, 1)]);
}

#[test]
fn lexar_numero_com_zeros_a_esquerda() {
    assert_eq!(lexar("007"), vec![num(7, 1)]);
}

#[test]
fn lexar_o_maior_i64() {
    assert_eq!(lexar("9223372036854775807"), vec![num(i64::MAX, 1)]);
}

#[test]
fn lexar_atribuicao_de_constante() {
    assert_eq!(
        lexar("a = 5"),
        vec![id("a", 1), f(Variante::Igual, 1), num(5, 1)]
    );
}

#[test]
fn lexar_atribuicao_de_constante_colada() {
    assert_eq!(
        lexar("b=10"),
        vec![id("b", 1), f(Variante::Igual, 1), num(10, 1)]
    );
}

#[test]
fn lexar_dois_numeros_separados_por_espaco() {
    assert_eq!(lexar("12 34"), vec![num(12, 1), num(34, 1)]);
}

#[test]
fn lexar_numeros_separados_por_virgula() {
    assert_eq!(
        lexar("1,2"),
        vec![num(1, 1), f(Variante::Virgula, 1), num(2, 1)]
    );
}

#[test]
fn lexar_numero_seguido_de_letra_sao_duas_fichas() {
    // 5abc: o número para na primeira letra
    assert_eq!(lexar("5abc"), vec![num(5, 1), id("abc", 1)]);
    assert_eq!(lexar("1x"), vec![num(1, 1), id("x", 1)]);
}

#[test]
fn lexar_letra_seguida_de_digito_e_identificador() {
    // x1 NÃO é 'x' e '1': identificador pode ter dígitos depois da letra
    assert_eq!(lexar("x1"), vec![id("x1", 1)]);
}

#[test]
fn lexar_o_menos_nao_gruda_no_numero() {
    // o léxico não decide sobre números negativos: isso fica para o leitor
    assert_eq!(lexar("-5"), vec![f(Variante::Menos, 1), num(5, 1)]);
    assert_eq!(
        lexar("3-2"),
        vec![num(3, 1), f(Variante::Menos, 1), num(2, 1)]
    );
}

#[test]
fn lexar_operacao_com_constantes() {
    assert_eq!(
        lexar("a = 5 + 10"),
        vec![
            id("a", 1),
            f(Variante::Igual, 1),
            num(5, 1),
            f(Variante::Mais, 1),
            num(10, 1),
        ]
    );
}

#[test]
fn lexar_call_com_numero_de_argumentos() {
    // a linha do documento: temp2 = call soma, 2
    assert_eq!(
        lexar("temp2 = call soma, 2"),
        vec![
            id("temp2", 1),
            f(Variante::Igual, 1),
            f(Variante::Call, 1),
            id("soma", 1),
            f(Variante::Virgula, 1),
            num(2, 1),
        ]
    );
}

#[test]
fn lexar_numero_no_fim_do_texto() {
    // sem quebra de linha depois: o peek devolve None
    assert_eq!(lexar("x = 42"), vec![id("x", 1), f(Variante::Igual, 1), num(42, 1)]);
}

#[test]
fn lexar_numeros_em_varias_linhas() {
    let fichas = lexar("a = 5\nb = 10");
    assert_eq!(
        fichas,
        vec![
            id("a", 1),
            f(Variante::Igual, 1),
            num(5, 1),
            id("b", 2),
            f(Variante::Igual, 2),
            num(10, 2),
        ]
    );
}

#[test]
#[should_panic(expected = "caractere inválido")]
fn lexar_numero_seguido_de_caractere_invalido_derruba() {
    lexar("5#");
}

#[test]
#[should_panic]
fn lexar_numero_grande_demais_para_i64_derruba() {
    // 20 dígitos não cabem em i64
    lexar("99999999999999999999");
}

// ---------- Etapa 6: léxico, @parameterN ----------

fn par(n: usize, linha: usize) -> Ficha {
    Ficha {
        palavra: Variante::Parametro(n),
        linha,
    }
}

#[test]
fn lexar_parametro_zero() {
    assert_eq!(lexar("@parameter0"), vec![par(0, 1)]);
}

#[test]
fn lexar_parametro_um() {
    assert_eq!(lexar("@parameter1"), vec![par(1, 1)]);
}

#[test]
fn lexar_parametro_com_dois_digitos() {
    assert_eq!(lexar("@parameter12"), vec![par(12, 1)]);
}

#[test]
fn lexar_parametro_e_uma_ficha_so() {
    assert_eq!(lexar("@parameter0").len(), 1);
}

#[test]
fn lexar_copia_de_parametro() {
    // x = @parameter0   (a linha do documento)
    assert_eq!(
        lexar("x = @parameter0"),
        vec![id("x", 1), f(Variante::Igual, 1), par(0, 1)]
    );
}

#[test]
fn lexar_copia_de_parametro_colada() {
    assert_eq!(
        lexar("y=@parameter1"),
        vec![id("y", 1), f(Variante::Igual, 1), par(1, 1)]
    );
}

#[test]
fn lexar_parametro_seguido_de_outra_ficha() {
    // o parâmetro consome exatamente "@parameter0"; o resto continua normal
    assert_eq!(lexar("@parameter0 y"), vec![par(0, 1), id("y", 1)]);
}

#[test]
fn lexar_parametro_seguido_de_simbolo() {
    assert_eq!(
        lexar("@parameter0+z"),
        vec![par(0, 1), f(Variante::Mais, 1), id("z", 1)]
    );
}

#[test]
fn lexar_parametro_com_indentacao() {
    assert_eq!(
        lexar("    x = @parameter0"),
        vec![id("x", 1), f(Variante::Igual, 1), par(0, 1)]
    );
}

#[test]
fn lexar_os_dois_parametros_do_documento() {
    let fichas = lexar("soma:\n    x = @parameter0\n    y = @parameter1");
    assert_eq!(
        fichas,
        vec![
            id("soma", 1),
            f(Variante::DoisPontos, 1),
            id("x", 2),
            f(Variante::Igual, 2),
            par(0, 2),
            id("y", 3),
            f(Variante::Igual, 3),
            par(1, 3),
        ]
    );
}

#[test]
fn lexar_parametro_na_expressao() {
    assert_eq!(
        lexar("t = @parameter0 + @parameter1"),
        vec![
            id("t", 1),
            f(Variante::Igual, 1),
            par(0, 1),
            f(Variante::Mais, 1),
            par(1, 1),
        ]
    );
}

#[test]
fn lexar_sem_arroba_e_identificador() {
    // 'parameter0' sem o @ não é um parâmetro
    assert_eq!(lexar("parameter0"), vec![id("parameter0", 1)]);
}

#[test]
#[should_panic(expected = "parâmetro")]
fn lexar_arroba_sem_numero_derruba() {
    lexar("@parameter");
}

#[test]
#[should_panic(expected = "parâmetro")]
fn lexar_arroba_com_palavra_errada_derruba() {
    lexar("@foo");
}

#[test]
#[should_panic(expected = "parâmetro")]
fn lexar_arroba_sozinho_derruba() {
    lexar("@");
}

#[test]
#[should_panic(expected = "parâmetro")]
fn lexar_parametro_com_maiuscula_derruba() {
    lexar("@Parameter0");
}

#[test]
#[should_panic(expected = "linha 2")]
fn lexar_erro_de_parametro_diz_a_linha() {
    lexar("x = 1\ny = @parameter");
}

// ---------- Etapa 6: leitor, operando ----------

use crate::leitor::operando;

#[test]
fn operando_de_identificador_e_nome() {
    let o = operando(&Variante::Identificador("x".to_string()));
    assert!(matches!(o, Operando::Nome(ref n) if n == "x"));
}

#[test]
fn operando_de_nome_com_digitos() {
    let o = operando(&Variante::Identificador("temp12".to_string()));
    assert!(matches!(o, Operando::Nome(ref n) if n == "temp12"));
}

#[test]
fn operando_de_numero_e_constante() {
    assert!(matches!(operando(&Variante::Numero(5)), Operando::Constante(5)));
    assert!(matches!(operando(&Variante::Numero(0)), Operando::Constante(0)));
    assert!(matches!(
        operando(&Variante::Numero(1234567890)),
        Operando::Constante(1234567890)
    ));
}

#[test]
fn operando_de_parametro() {
    assert!(matches!(operando(&Variante::Parametro(0)), Operando::Parametro(0)));
    assert!(matches!(operando(&Variante::Parametro(3)), Operando::Parametro(3)));
}

#[test]
fn operando_true_vale_1() {
    assert!(matches!(operando(&Variante::True), Operando::Constante(1)));
}

#[test]
fn operando_false_vale_0() {
    assert!(matches!(operando(&Variante::False), Operando::Constante(0)));
}

#[test]
fn operando_nao_consome_a_ficha() {
    // a função empresta: a mesma ficha pode ser convertida duas vezes
    let v = Variante::Identificador("a".to_string());
    let _ = operando(&v);
    let o = operando(&v);
    assert!(matches!(o, Operando::Nome(ref n) if n == "a"));
}

#[test]
fn operando_recusa_tudo_que_nao_e_valor() {
    let proibidas = vec![
        Variante::Igual,
        Variante::Mais,
        Variante::Menos,
        Variante::Vezes,
        Variante::DivididoPor,
        Variante::DoisPontos,
        Variante::Virgula,
        Variante::Goto,
        Variante::If,
        Variante::IfFalse,
        Variante::Param,
        Variante::Call,
        Variante::Return,
    ];
    for v in proibidas {
        let resultado = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| operando(&v)));
        assert!(resultado.is_err(), "{:?} não deveria virar operando", v);
    }
}

// ---------- Etapa 6: leitor, operador ----------

use crate::instrucao::Op as OpInstr;
use crate::leitor::operador;

#[test]
fn operador_mais_e_soma() {
    assert!(matches!(operador(&Variante::Mais), OpInstr::Add));
}

#[test]
fn operador_menos_e_subtracao() {
    assert!(matches!(operador(&Variante::Menos), OpInstr::Sub));
}

#[test]
fn operador_vezes_e_multiplicacao() {
    assert!(matches!(operador(&Variante::Vezes), OpInstr::Mul));
}

#[test]
fn operador_dividido_e_divisao() {
    assert!(matches!(operador(&Variante::DivididoPor), OpInstr::Div));
}

#[test]
fn operador_aplicado_pela_maquina() {
    // do símbolo até o resultado: o leitor e a máquina combinam
    let r = Maquina::efetuar(&operador(&Variante::Mais), 3, 4);
    assert_eq!(r, 7);
    assert_eq!(Maquina::efetuar(&operador(&Variante::Menos), 3, 10), -7);
    assert_eq!(Maquina::efetuar(&operador(&Variante::Vezes), 3, 4), 12);
    assert_eq!(Maquina::efetuar(&operador(&Variante::DivididoPor), 8, 2), 4);
}

#[test]
fn operador_recusa_o_que_nao_e_simbolo_aritmetico() {
    let proibidas = vec![
        Variante::Igual,
        Variante::DoisPontos,
        Variante::Virgula,
        Variante::Goto,
        Variante::If,
        Variante::IfFalse,
        Variante::Param,
        Variante::Call,
        Variante::Return,
        Variante::True,
        Variante::False,
        Variante::Numero(1),
        Variante::Parametro(0),
        Variante::Identificador("x".to_string()),
    ];
    for v in proibidas {
        let resultado = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| operador(&v)));
        assert!(resultado.is_err(), "{:?} não deveria virar operador", v);
    }
}

#[test]
fn operador_nao_consome_a_ficha() {
    let v = Variante::Mais;
    let _ = operador(&v);
    assert!(matches!(operador(&v), OpInstr::Add));
}

// ---------- Etapa 6: leitor, ler_linha ----------

use crate::instrucao::Instrucao as I;
use crate::leitor::ler_linha;

fn linha(texto: &str) -> I {
    let v: Vec<Variante> = lexar(texto).into_iter().map(|x| x.palavra).collect();
    ler_linha(&v)
}

fn nome(s: &str) -> Operando {
    Operando::Nome(s.to_string())
}

#[test]
fn ler_rotulo() {
    assert_eq!(linha("soma:"), I::Rotulo { nome: "soma".to_string() });
    assert_eq!(linha("LFim:"), I::Rotulo { nome: "LFim".to_string() });
}

#[test]
fn ler_goto() {
    assert_eq!(linha("goto LFim"), I::Goto { rotulo: "LFim".to_string() });
}

#[test]
fn ler_copia_de_constante() {
    assert_eq!(
        linha("a = 5"),
        I::Copia { destino: "a".to_string(), fonte: Operando::Constante(5) }
    );
}

#[test]
fn ler_copia_de_nome() {
    assert_eq!(
        linha("r = temp2"),
        I::Copia { destino: "r".to_string(), fonte: nome("temp2") }
    );
}

#[test]
fn ler_copia_de_parametro() {
    assert_eq!(
        linha("x = @parameter0"),
        I::Copia { destino: "x".to_string(), fonte: Operando::Parametro(0) }
    );
    assert_eq!(
        linha("y = @parameter1"),
        I::Copia { destino: "y".to_string(), fonte: Operando::Parametro(1) }
    );
}

#[test]
fn ler_copia_de_true_e_false() {
    assert_eq!(
        linha("x = true"),
        I::Copia { destino: "x".to_string(), fonte: Operando::Constante(1) }
    );
    assert_eq!(
        linha("x = false"),
        I::Copia { destino: "x".to_string(), fonte: Operando::Constante(0) }
    );
}

#[test]
fn ler_binaria_soma() {
    assert_eq!(
        linha("temp1 = x + y"),
        I::Binaria {
            destino: "temp1".to_string(),
            esquerda: nome("x"),
            op: Op::Add,
            direita: nome("y"),
        }
    );
}

#[test]
fn ler_binaria_com_os_quatro_operadores() {
    for (texto, op) in [("+", Op::Add), ("-", Op::Sub), ("*", Op::Mul), ("/", Op::Div)] {
        let l = linha(&format!("t = a {} b", texto));
        assert_eq!(
            l,
            I::Binaria {
                destino: "t".to_string(),
                esquerda: nome("a"),
                op,
                direita: nome("b"),
            }
        );
    }
}

#[test]
fn ler_binaria_com_constantes_e_parametros() {
    assert_eq!(
        linha("c = 1 + 2"),
        I::Binaria {
            destino: "c".to_string(),
            esquerda: Operando::Constante(1),
            op: Op::Add,
            direita: Operando::Constante(2),
        }
    );
    assert_eq!(
        linha("t = @parameter0 * @parameter1"),
        I::Binaria {
            destino: "t".to_string(),
            esquerda: Operando::Parametro(0),
            op: Op::Mul,
            direita: Operando::Parametro(1),
        }
    );
    assert_eq!(
        linha("t = x - 5"),
        I::Binaria {
            destino: "t".to_string(),
            esquerda: nome("x"),
            op: Op::Sub,
            direita: Operando::Constante(5),
        }
    );
}

#[test]
fn ler_if() {
    assert_eq!(
        linha("if t1 goto LTrue"),
        I::If { condicao: nome("t1"), rotulo: "LTrue".to_string() }
    );
}

#[test]
fn ler_if_false() {
    assert_eq!(
        linha("ifFalse t1 goto LFalse"),
        I::IfFalse { condicao: nome("t1"), rotulo: "LFalse".to_string() }
    );
}

#[test]
fn ler_if_com_constante_e_booleano() {
    assert_eq!(
        linha("if 1 goto L"),
        I::If { condicao: Operando::Constante(1), rotulo: "L".to_string() }
    );
    assert_eq!(
        linha("ifFalse false goto L"),
        I::IfFalse { condicao: Operando::Constante(0), rotulo: "L".to_string() }
    );
}

#[test]
fn ler_param() {
    assert_eq!(linha("param a"), I::Param { fonte: nome("a") });
    assert_eq!(linha("param 5"), I::Param { fonte: Operando::Constante(5) });
}

#[test]
fn ler_return() {
    assert_eq!(linha("return temp1"), I::Return { valor: nome("temp1") });
    assert_eq!(linha("return 7"), I::Return { valor: Operando::Constante(7) });
}

#[test]
fn ler_call_sem_destino() {
    assert_eq!(
        linha("call soma, 2"),
        I::Call { destino: None, funcao: "soma".to_string(), n: 2 }
    );
}

#[test]
fn ler_call_com_destino() {
    assert_eq!(
        linha("temp2 = call soma, 2"),
        I::Call {
            destino: Some("temp2".to_string()),
            funcao: "soma".to_string(),
            n: 2,
        }
    );
}

#[test]
fn ler_call_com_zero_argumentos() {
    assert_eq!(
        linha("call f, 0"),
        I::Call { destino: None, funcao: "f".to_string(), n: 0 }
    );
    assert_eq!(
        linha("x = call f, 0"),
        I::Call { destino: Some("x".to_string()), funcao: "f".to_string(), n: 0 }
    );
}

#[test]
fn ler_linha_ignora_indentacao() {
    assert_eq!(
        linha("      temp1 = x + y"),
        linha("temp1 = x + y")
    );
}

#[test]
fn ler_linhas_do_documento_batem_com_o_vec_montado_a_mao() {
    let texto = [
        "soma:",
        "x = @parameter0",
        "y = @parameter1",
        "temp1 = x + y",
        "return temp1",
        "main:",
        "a = 5",
        "b = 10",
        "param a",
        "param b",
        "temp2 = call soma, 2",
        "r = temp2",
    ];
    let esperado = crate::programa_soma();
    assert_eq!(texto.len(), esperado.len());
    for (i, t) in texto.iter().enumerate() {
        assert_eq!(linha(t), esperado[i], "linha {} ({})", i, t);
    }
}

// ---------- erros ----------

#[test]
#[should_panic(expected = "linha não reconhecida")]
fn ler_linha_vazia_derruba() {
    ler_linha(&[]);
}

#[test]
#[should_panic(expected = "linha não reconhecida")]
fn ler_tres_identificadores_derruba() {
    linha("a b c");
}

#[test]
#[should_panic(expected = "linha não reconhecida")]
fn ler_goto_sem_rotulo_derruba() {
    linha("goto");
}

#[test]
#[should_panic(expected = "linha não reconhecida")]
fn ler_goto_para_numero_derruba() {
    linha("goto 5");
}

#[test]
#[should_panic(expected = "linha não reconhecida")]
fn ler_atribuicao_sem_valor_derruba() {
    linha("x =");
}

#[test]
#[should_panic(expected = "linha não reconhecida")]
fn ler_operacao_incompleta_derruba() {
    linha("x = y +");
}

#[test]
#[should_panic(expected = "linha não reconhecida")]
fn ler_return_sem_valor_derruba() {
    linha("return");
}

#[test]
#[should_panic(expected = "linha não reconhecida")]
fn ler_if_sem_goto_derruba() {
    linha("if x L");
}

#[test]
#[should_panic(expected = "linha não reconhecida")]
fn ler_call_sem_virgula_derruba() {
    linha("call soma 2");
}

#[test]
#[should_panic]
fn ler_atribuicao_com_operador_invalido_derruba() {
    // x = a = b: o "operador" é um Igual
    linha("x = a = b");
}

#[test]
#[should_panic]
fn ler_atribuicao_de_palavra_reservada_derruba() {
    // x = call: 'call' não é um operando
    linha("x = call");
}

// ---------- Etapa 6: leitor, agrupar ----------

use crate::leitor::agrupar;

fn vid(s: &str) -> Variante {
    Variante::Identificador(s.to_string())
}

#[test]
fn agrupar_uma_linha() {
    assert_eq!(
        agrupar(lexar("x = 1")),
        vec![vec![vid("x"), Variante::Igual, Variante::Numero(1)]]
    );
}

#[test]
fn agrupar_duas_linhas_nao_perde_ficha() {
    assert_eq!(
        agrupar(lexar("a b\nc d")),
        vec![vec![vid("a"), vid("b")], vec![vid("c"), vid("d")]]
    );
}

#[test]
fn agrupar_nao_esquece_a_ultima_linha() {
    assert_eq!(
        agrupar(lexar("a\nb\nc")),
        vec![vec![vid("a")], vec![vid("b")], vec![vid("c")]]
    );
}

#[test]
fn agrupar_com_linha_em_branco_no_meio() {
    assert_eq!(
        agrupar(lexar("x\n\ny z")),
        vec![vec![vid("x")], vec![vid("y"), vid("z")]]
    );
}

#[test]
fn agrupar_com_varias_linhas_em_branco() {
    assert_eq!(
        agrupar(lexar("a\n\n\n\nb\n\n\nc")),
        vec![vec![vid("a")], vec![vid("b")], vec![vid("c")]]
    );
}

#[test]
fn agrupar_com_linhas_em_branco_no_comeco() {
    assert_eq!(agrupar(lexar("\n\na")), vec![vec![vid("a")]]);
}

#[test]
fn agrupar_com_linhas_em_branco_no_fim() {
    assert_eq!(agrupar(lexar("a\n\n\n")), vec![vec![vid("a")]]);
}

#[test]
fn agrupar_texto_vazio_da_lista_vazia() {
    assert!(agrupar(lexar("")).is_empty());
}

#[test]
fn agrupar_nunca_gera_grupo_vazio() {
    let grupos = agrupar(lexar("\n\nsoma:\n\n    x = y\n\nmain:\n"));
    assert!(grupos.iter().all(|g| !g.is_empty()), "{:?}", grupos);
    assert_eq!(grupos.len(), 3);
}

// ---------- Etapa 6.3: do texto até a execução ----------

use crate::leitor::ler_programa;

// o TAC do documento, com a indentação e a linha em branco originais (sem os comentários)
const SOMA_TXT: &str = "soma:
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
";

fn executar_texto(texto: &str) -> Maquina {
    executar(ler_programa(texto))
}

#[test]
fn texto_do_documento_gera_o_mesmo_vec_feito_a_mao() {
    assert_eq!(ler_programa(SOMA_TXT), crate::programa_soma());
}

#[test]
fn texto_do_documento_executa_com_r_igual_a_15() {
    let m = executar_texto(SOMA_TXT);
    assert_eq!(var(&m, "r"), 15);
    assert_eq!(m.pilha.len(), 1);
}

#[test]
fn texto_do_documento_com_fim_de_linha_do_windows() {
    let windows = SOMA_TXT.replace('\n', "\r\n");
    assert_eq!(ler_programa(&windows), crate::programa_soma());
}

#[test]
fn texto_do_documento_sem_indentacao_e_sem_quebra_no_fim() {
    let seco: Vec<&str> = SOMA_TXT.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    assert_eq!(ler_programa(&seco.join("\n")), crate::programa_soma());
}

#[test]
fn texto_com_tabs_e_muitas_linhas_em_branco() {
    let m = executar_texto("\n\nmain:\n\n\n\ta = 2\n\n\tb = a * a\n\n");
    assert_eq!(var(&m, "b"), 4);
}

#[test]
fn texto_vazio_gera_programa_vazio() {
    assert!(ler_programa("").is_empty());
    assert!(ler_programa("\n\n   \n").is_empty());
}

#[test]
fn texto_if_else() {
    for (c, esperado) in [("true", 10), ("false", 20)] {
        let texto = format!(
            "main:
    c = {}
    ifFalse c goto LElse
    r = 10
    goto LFim
LElse:
    r = 20
LFim:
",
            c
        );
        assert_eq!(var(&executar_texto(&texto), "r"), esperado, "c = {}", c);
    }
}

#[test]
fn texto_while_soma_de_10_ate_1() {
    let m = executar_texto(
        "main:
    a = 10
    s = 0
LLoop:
    ifFalse a goto LFim
    s = s + a
    a = a - 1
    goto LLoop
LFim:
",
    );
    assert_eq!(var(&m, "s"), 55);
}

#[test]
fn texto_e_logico_com_curto_circuito() {
    let m = executar_texto(
        "main:
    t1 = false
    ifFalse t1 goto LFalse
    avaliou = 1
    ifFalse t2 goto LFalse
    x = true
    goto LFim
LFalse:
    x = false
LFim:
",
    );
    // t2 nem existe: se fosse avaliado, daria variável ausente
    assert_eq!(var(&m, "x"), 0);
    assert!(!tem(&m, "avaliou"));
}

#[test]
fn texto_ou_logico() {
    let m = executar_texto(
        "main:
    t1 = true
    if t1 goto LTrue
    if t2 goto LTrue
    x = false
    goto LFim
LTrue:
    x = true
LFim:
",
    );
    assert_eq!(var(&m, "x"), 1);
}

#[test]
fn texto_expressao_aritmetica_do_documento() {
    // x = a + b * c
    let m = executar_texto(
        "main:
    a = 2
    b = 3
    c = 4
    temp1 = b * c
    temp2 = a + temp1
    x = temp2
",
    );
    assert_eq!(var(&m, "x"), 14);
}

#[test]
fn texto_recursao_com_chamada_de_procedimento() {
    let m = executar_texto(
        "ignora:
    return 0

fat:
    n = @parameter0
    if n goto Lrec
    return 1
Lrec:
    m = n - 1
    param m
    f = call fat, 1
    r = n * f
    return r

main:
    call ignora, 0
    param 5
    res = call fat, 1
",
    );
    assert_eq!(var(&m, "res"), 120);
}

#[test]
#[should_panic(expected = "linha não reconhecida")]
fn texto_com_linha_sem_forma_derruba() {
    ler_programa("main:\n    a b c\n");
}

#[test]
#[should_panic(expected = "linha 2")]
fn texto_com_caractere_invalido_diz_a_linha() {
    ler_programa("main:\n    a = #\n");
}
