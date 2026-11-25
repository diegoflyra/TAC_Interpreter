use crate::instrucao::Op;
use crate::instrucao::Operando;

use std::{collections::HashMap, hash::Hash};
pub struct Quadro {
    pub variaveis: HashMap<String, i64>,
}

impl Quadro {
    pub fn novo(v: HashMap<String, i64>) -> Quadro {
        return Quadro { variaveis: v };
    }
}

pub struct Maquina {
    pub pc: usize,
    pub pilha: Vec<Quadro>,
}

impl Maquina {
    pub fn montar(pc: usize) -> Maquina {
        let v: Vec<Quadro> = vec![Quadro::novo(HashMap::new())];

        return Maquina { pc: pc, pilha: v };
    }

    pub fn get_valor(&self, op: &Operando) -> i64 {
        match op {
            Operando::Constante(n) => return *n,
            Operando::Parametro(n) => panic!("ainda não"),
            Operando::Nome(n) => match self.pilha.last() {
                Some(q) => match q.variaveis.get(n) {
                    Some(k) => return *k,
                    None => panic!("variável vazia!"),
                },
                None => panic!("pilha vazia!"),
            },
        }
    }

    pub fn efetuar(op: &Op, a: i64, b: i64) -> i64 {
        if b == 0 {
            panic!("Divisão por 0!")
        }

        match op {
            Op::Add => return a + b,
            Op::Sub => return a - b,
            Op::Mul => return a * b,
            Op::Div => {
                if b == 0 {
                    panic!("divisão por 0!")
                } else {
                    return a / b;
                }
            }
        }
    }
}
