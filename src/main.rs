mod instrucao;
mod maquina;
mod programa;
mod lexer;
mod leitor;
mod repl;
#[cfg(test)]
mod testes;

#[cfg(test)]
use crate::instrucao::{Instrucao, Op, Operando};
use crate::leitor::ler_programa;
use crate::maquina::Maquina;
use crate::programa::Programa;

// programa `soma` / `main` do documento, montado à mão: os testes comparam o leitor com ele
#[cfg(test)]
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

// o caminho comum às duas frentes: texto de TAC -> máquina que já terminou de rodar
fn rodar_texto(texto: &str) -> Maquina {
    let p = Programa::montar(ler_programa(texto));
    let mut m = Maquina::montar(p.inicio);
    m.rodar(&p);
    m
}

fn modo_arquivo(caminho: &str) {
    match std::fs::read_to_string(caminho) {
        Ok(texto) => rodar_texto(&texto).mostrar(),
        Err(erro) => {
            eprintln!("erro: não consegui ler '{}': {}", caminho, erro);
            std::process::exit(1);
        }
    }
}

// troca a mensagem padrão do panic (com arquivo e linha do Rust) por "erro: ..."
fn instalar_mensagem_de_erro() {
    std::panic::set_hook(Box::new(|info| {
        let dados = info.payload();
        let mensagem = dados
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| dados.downcast_ref::<String>().map(|s| s.as_str()))
            .unwrap_or("falha na execução");
        eprintln!("erro: {}", mensagem);
    }));
}

fn main() {
    instalar_mensagem_de_erro();

    let argumentos: Vec<String> = std::env::args().collect();

    match argumentos.as_slice() {
        [_] => {
            let entrada = std::io::stdin().lock();
            let saida = std::io::stdout();
            if let Err(erro) = repl::rodar(entrada, saida) {
                eprintln!("erro no terminal: {}", erro);
                std::process::exit(1);
            }
        }
        [_, caminho] => modo_arquivo(caminho),
        _ => {
            eprintln!("uso: cargo run [-- programa.txt]");
            std::process::exit(1);
        }
    }
}
