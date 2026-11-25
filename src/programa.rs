use crate::instrucao::Instrucao;
use std::{collections::HashMap, hash::Hash};
pub struct Programa {
    pub instrucoes: Vec<Instrucao>,
    pub tabela: HashMap<String, usize>,
    pub inicio: usize,
}

impl Programa {
    pub fn montar(v: Vec<Instrucao>) -> Programa {
        let mut t: HashMap<String, usize> = HashMap::new();

        for (pos, i) in v.iter().enumerate() {
            match i {
                Instrucao::Rotulo { nome } => {
                    match t.get(nome) {
                        Some(pos) => panic!("rotulo repetido"),
                        None => t.insert(nome.clone(), pos),
                    };
                }
                _ => {}
            }
        }

        let inicio: usize = match t.get("main") {
            Some(i) => *i,
            None => panic! {"main ausente"},
        };

        return Programa {
            instrucoes: v,
            tabela: t,
            inicio: inicio,
        };
    }
}
