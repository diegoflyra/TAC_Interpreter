pub enum Operando {
    Nome(String),
    Constante(i64),
    Parametro(usize),
}

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Instrucao {
    Rotulo {
        nome: String,
    },
    Copia {
        destino: String,
        fonte: Operando,
    },
    Binaria {
        destino: String,
        esquerda: Operando,
        op: Op,
        direita: Operando,
    },
    Goto {
        rotulo: String,
    },
    If {
        condicao: Operando,
        rotulo: String,
    },
    IfFalse {
        condicao: Operando,
        rotulo: String,
    },
    Param {
        fonte: Operando,
    },
    Call {
        destino: Option<String>,
        funcao: String,
        n: usize,
    },
    Return {
        valor: Operando,
    },
}
