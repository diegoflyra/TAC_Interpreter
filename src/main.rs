mod instrucao;
mod maquina;
mod programa;
use crate::instrucao::Instrucao;
use crate::instrucao::Op;
use crate::instrucao::Operando;
use crate::maquina::Maquina;
use crate::maquina::Quadro;
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

fn mostrar_indices(p: &Programa) {
    for (nome, pos) in &p.tabela {
        println!("{}: {}", nome, pos);
    }
}

fn main() {
    let programa = programa_soma();
    println!("{}", programa.len());

    let p = Programa::montar(programa_soma());

    mostrar_indices(&p);

    let m = Maquina::montar(p.inicio);
    println!("{}", m.pc); // deve imprimir 5
    println!("{}", m.pilha.len()); // deve imprimir 1

    println!("{}", m.get_valor(&Operando::Constante(7))); // 7

    let mut m = Maquina::montar(p.inicio);
    m.pilha[0].variaveis.insert("x".to_string(), 42);
    println!("{}", m.get_valor(&Operando::Nome("x".to_string()))); // 42

    println!("{}", Maquina::efetuar(&Op::Add, 3, 4)); // 7
    println!("{}", Maquina::efetuar(&Op::Mul, 3, 4)); // 12
    println!("{}", Maquina::efetuar(&Op::Sub, 3, 10)); // -7
    println!("{}", Maquina::efetuar(&Op::Div, 8, 2)); // 4
}
