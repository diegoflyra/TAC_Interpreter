use crate::instrucao::Instrucao::Copia;
use crate::instrucao::Op;
use crate::instrucao::Op::Mul;
use crate::instrucao::Op::Sub;
use crate::instrucao::Operando;
use crate::instrucao::Instrucao;
use crate::lexer::Variante::Identificador;
use crate::lexer::lexar;
use crate::programa::Programa;
use crate::lexer::Ficha;
use crate::lexer::Variante;

pub fn operando(v: &Variante) -> Operando{

    match v {
        Variante::Identificador(Nome) => Operando::Nome(Nome.clone()),
        Variante::Numero(n) => Operando::Constante(*n),
        Variante::Parametro(n) => Operando::Parametro(*n),
        Variante::True => Operando::Constante(1),
        Variante::False => Operando::Constante(0),
        _ => panic!("Não é um operando")
    }

}

pub fn operador(v: &Variante) -> Op {

    match v {

        Variante::Mais => Op::Add,
        Variante::Menos => Op::Sub,
        Variante::Vezes => Op::Mul,
        Variante::DivididoPor => Op::Div,
        _ => panic!("Não é um operador")

    }
}
pub const TEMP_NEG: &str = "string desnecessariamente grande que serve de destino aleatorio e que eu imagino que ninguem vai usar esse nome pra guardar nada";
pub fn ler_linha (fichas: &[Variante]) -> Vec<Instrucao> {

    match fichas {
        [Variante::Identificador(nome), Variante::DoisPontos] =>
            vec![Instrucao::Rotulo { nome: nome.clone() }],
    
        [Variante::Goto, Variante::Identificador(rotulo)] =>
            vec![Instrucao::Goto { rotulo: rotulo.clone() }],
    

        [Variante::Identificador(destino), Variante::Igual, fonte] =>
            vec![Instrucao::Copia { destino: destino.clone(), fonte: operando(fonte) }],

        [Variante::Identificador(destino), Variante::Igual, Variante::Menos, fonte] =>
            vec![Instrucao::Binaria { destino: (destino.clone()), esquerda: (Operando::Constante(0)), op: (Op::Sub), direita: (operando(fonte)) }],

        [Variante::If, condicao, Variante::Goto, Variante::Identificador(rotulo)] =>
            vec![Instrucao::If { condicao: operando(condicao), rotulo: (rotulo.clone()) }],

        [Variante::IfFalse, condicao, Variante::Goto, Variante::Identificador(rotulo)] =>
            vec![Instrucao::IfFalse { condicao: operando(condicao), rotulo: (rotulo.clone()) }],
        [Variante::Param, n] => vec![Instrucao::Param { fonte: operando(n) }],
        [Variante::Param, Variante::Menos, n] => 
            vec![Instrucao::Binaria { destino: TEMP_NEG.to_string(), esquerda: (Operando::Constante(0)), op: (Op::Sub), direita: (operando(n)) }, Instrucao::Param { fonte: Operando::Nome(TEMP_NEG.to_string()) }],
        [Variante::Return, x] => vec![Instrucao::Return { valor: operando(x) }],
        [Variante::Return, Variante::Menos, x] => 
            vec![Instrucao::Binaria { destino: (TEMP_NEG.to_string()), esquerda: (Operando::Constante(0)), op: (Op::Sub), direita: (operando(x)) }, Instrucao::Return { valor: Operando::Nome(TEMP_NEG.to_string()) }],
        [Variante::Call, Variante::Identificador(f), Variante::Virgula, Variante::Numero(n)] =>
            vec![Instrucao::Call { destino: None, funcao: f.clone(), n: (*n as usize) }],
        [Variante::Identificador(t), Variante::Igual, x, op, y] =>
            vec![Instrucao::Binaria { destino: t.clone(), esquerda: operando(x), op: operador(op), direita: operando(y)}],
        [Variante::Identificador(t), Variante::Igual, Variante::Menos, x, op, y] =>
            vec![Instrucao::Binaria { destino: (TEMP_NEG.to_string()), esquerda: (Operando::Constante(0)), op: (Op::Sub), direita: (operando(x)) }, Instrucao::Binaria { destino: t.clone(), esquerda: Operando::Nome(TEMP_NEG.to_string()), op: operador(op), direita: operando(y)}],
        [Variante::Identificador(t), Variante::Igual, x, op, Variante::Menos, y] =>
            vec![Instrucao::Binaria { destino: (TEMP_NEG.to_string()), esquerda: (Operando::Constante(0)), op: (Op::Sub), direita: (operando(y)) }, Instrucao::Binaria { destino: t.clone(), esquerda: operando(x), op: operador(op), direita: Operando::Nome(TEMP_NEG.to_string())}],

        [Variante::Identificador(t), Variante::Igual, Variante::Call, Variante::Identificador(f), Variante::Virgula, Variante::Numero(n)] => 
            vec![Instrucao::Call { destino: Some(t.clone()), funcao: (f.clone()), n: (*n as usize) }],
            


    

        _ => panic!("linha não reconhecida"),
    }
}

pub fn agrupar(fichas: Vec<Ficha>) -> Vec<Vec<Variante>> {

    let mut v: Vec<Vec<Variante>> = vec![vec![]];

    let mut linha: usize = 1;

    let mut c: usize = 0;

    while c < fichas.len() {
        
        if linha == fichas[c].linha {
            match v.last_mut() {
                Some(vi) => {
                    vi.push(fichas[c].palavra.clone());
                    c += 1;
                },
                None => {}
            }
        } else {
            linha += 1;
            
            match v.last() {
                Some(vi) => {
                    if !vi.is_empty() {
                        v.push(vec![]);
                    }
                },
                None => {}
            }



        }

    }

    match v.last() {
        Some(vi) => {
            if vi.is_empty() {
                v.pop();
            }
        },
        None => {}
    }

    return v;
}

pub fn ler_programa(texto: &str) -> Vec<Instrucao> {

    let fichas = lexar(texto);
    let grupos = agrupar(fichas);
    let mut instrucoes: Vec<Instrucao> = vec![];

    for grupo in grupos {
        instrucoes.extend(ler_linha(&grupo));
    }

    return instrucoes;



}