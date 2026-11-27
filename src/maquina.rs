use crate::instrucao::Op;
use crate::instrucao::Operando;
use crate::instrucao::Instrucao;
use crate::programa::Programa;
use crate::leitor::TEMP_NEG;

use std::{collections::HashMap, hash::Hash};
pub struct Quadro {
    pub variaveis: HashMap<String, i64>,
    pub argumentos: Vec<i64>,
    pub retorno: usize,
    pub destino: Option<String>
}

impl Quadro {
    pub fn novo(v: HashMap<String, i64>, a: Vec<i64>, r: usize, d: Option<String>) -> Quadro {
        return Quadro { variaveis: v , argumentos: a, retorno:r, destino:d};
    }
}

pub struct Maquina {
    pub pc: usize,
    pub pilha: Vec<Quadro>,
    pub fila_parametros: Vec<i64>,
}

impl Maquina {
    pub fn montar(pc: usize) -> Maquina {
        let v: Vec<Quadro> = vec![Quadro::novo(HashMap::new(), vec![], 0, None)];

        return Maquina { pc: pc, pilha: v , fila_parametros: vec![]};
    }

    pub fn get_valor(&self, op: &Operando) -> i64 {
        match op {
            Operando::Constante(n) => return *n,
            Operando::Parametro(n) => {
                let o = self.pilha.last();
                match o {
                    Some(q) => {match q.argumentos.get(*n) {
                        Some (p) => {return *p},
                        None => panic!("parametro não existe")
                    }},
                    None => panic!("Pilha vazia!")
                }
            },
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


    pub fn rodar (&mut self, p: &Programa) {

        while self.pc < p.instrucoes.len() {
            
            match &p.instrucoes[self.pc] {
                Instrucao::Rotulo { .. } => {},
                Instrucao::Copia { destino, fonte } => {

                    let valor = self.get_valor(fonte);

                    match self.pilha.last_mut(){

                        Some(q) => {q.variaveis.insert(destino.clone(), valor);},
                        None => panic!("Pilha vazia"),
                    }

                },
                Instrucao::Binaria { destino, esquerda, op, direita } => {
                    let valor = Maquina::efetuar(op, self.get_valor(esquerda), self.get_valor(direita));

                    match self.pilha.last_mut(){

                    Some(q) => {q.variaveis.insert(destino.clone(), valor);},
                    None => panic!("Pilha vazia"),
                }
                },
                Instrucao::Goto { rotulo } => {match p.tabela.get(rotulo) {
                    Some(i) => {self.pc = *i; continue; },
                    None => panic!("rotulo inexistente")
                }},
                Instrucao::If { condicao, rotulo } => {
                    
                    if self.get_valor(condicao) != 0 {
                        match p.tabela.get(rotulo) {
                            Some(i) => {self.pc = *i; continue; },
                            None => panic!("rotulo inexistente")
                        }
                    }
                },
                Instrucao::IfFalse { condicao, rotulo } => {
                    if self.get_valor(condicao) == 0 {
                        match p.tabela.get(rotulo) {
                            Some(i) => {self.pc = *i; continue; },
                            None => panic!("rotulo inexistente")
                        }
                    }
                }, 
                Instrucao::Param { fonte } => {
                    let valor = self.get_valor(fonte);
                    self.fila_parametros.push(valor);

                },
                Instrucao::Call { destino, funcao, n } => {

                    if self.fila_parametros.len() != *n{
                        panic!("número incorreto de argumentos")
                    }

                    let args = self.fila_parametros.clone();
                    self.fila_parametros.clear();

                    let e = match p.tabela.get(funcao) {
                                        Some(i) => *i,
                                        None => panic!("função inexistente"),

                                        };
                    
                    let q = Quadro::novo(HashMap::new(), args, self.pc + 1, destino.clone());
                    
                    self.pilha.push(q);
                    self.pc = e;
                    continue;

                }, 
                Instrucao::Return { valor } => {

                    let v = self.get_valor(valor);

                    if self.pilha.len() == 1 {
                        self.pc = p.instrucoes.len(); 
                        continue;
                    }

                    let o = self.pilha.pop();

                    if self.pilha.is_empty() {
                        self.pc = p.instrucoes.len();
                        continue;
                    } else {
                        match o {
                            Some(n) => {match self.pilha.last_mut(){
                                                Some(q) => {if let Some(n) = &n.destino{
                                                    q.variaveis.insert(n.to_string(), v);}
                                                                         self.pc = n.retorno;
                                                                         continue;},  
                                                None => {},
                                                
                                                }
                        },
                            None => {}
                        }
                    }
                    
                },
                

                
            }


            self.pc += 1;
        }
    }

    // uma linha "nome: valor" por variável do quadro atual, em ordem alfabética,
    // sem o temporário que o leitor cria para o menos unário
    pub fn ambiente(&self) -> Vec<String> {
        let quadro = self.pilha.last().expect("pilha vazia!");

        let mut variaveis: Vec<(&String, &i64)> = quadro
            .variaveis
            .iter()
            .filter(|(nome, _)| nome.as_str() != TEMP_NEG)
            .collect();
        variaveis.sort();

        variaveis
            .iter()
            .map(|(nome, valor)| format!("{}: {}", nome, valor))
            .collect()
    }

    pub fn mostrar(&self) {
        for linha in self.ambiente() {
            println!("{}", linha);
        }
    }
}
