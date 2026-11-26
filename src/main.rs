mod instrucao;
mod maquina;
mod programa;
mod lexer;
mod leitor;
#[cfg(test)]
mod testes;
use crate::instrucao::Instrucao;
use crate::instrucao::Op;
use crate::instrucao::Operando;
use crate::maquina::Maquina;
use crate::programa::Programa;

fn programa_soma() -> Vec<Instrucao> {
    return vec![
        Instrucao::Rotulo {
            nome: "soma".to_string(),
        },
        Instrucao::Copia {
            destino: "x".to_string(),
            fonte: Operando::Parametro(0),
        },
        Instrucao::Copia {
            destino: "y".to_string(),
            fonte: Operando::Parametro(1),
        },
        Instrucao::Binaria {
            destino: "temp1".to_string(),
            esquerda: Operando::Nome("x".to_string()),
            op: Op::Add,
            direita: Operando::Nome("y".to_string()),
        },
        Instrucao::Return {
            valor: Operando::Nome("temp1".to_string()),
        },
        Instrucao::Rotulo {
            nome: "main".to_string(),
        },
        Instrucao::Copia {
            destino: "a".to_string(),
            fonte: Operando::Constante(5),
        },
        Instrucao::Copia {
            destino: "b".to_string(),
            fonte: Operando::Constante(10),
        },
        Instrucao::Param {
            fonte: Operando::Nome("a".to_string()),
        },
        Instrucao::Param {
            fonte: Operando::Nome("b".to_string()),
        },
        Instrucao::Call {
            destino: Some("temp2".to_string()),
            funcao: "soma".to_string(),
            n: 2,
        },
        Instrucao::Copia {
            destino: "r".to_string(),
            fonte: Operando::Nome("temp2".to_string()),
        },
    ];
}

// main:
//     a = 5
//     b = 10
//     c = a + b
fn programa_reto() -> Vec<Instrucao> {
    vec![
        Instrucao::Rotulo {
            nome: "main".to_string(),
        },
        Instrucao::Copia {
            destino: "a".to_string(),
            fonte: Operando::Constante(5),
        },
        Instrucao::Copia {
            destino: "b".to_string(),
            fonte: Operando::Constante(10),
        },
        Instrucao::Binaria {
            destino: "c".to_string(),
            esquerda: Operando::Nome("a".to_string()),
            op: Op::Add,
            direita: Operando::Nome("b".to_string()),
        },
    ]
}

fn main() {
    let p = Programa::montar(programa_reto());
    let mut m = Maquina::montar(p.inicio);
    m.rodar(&p);
    m.mostrar();
}
