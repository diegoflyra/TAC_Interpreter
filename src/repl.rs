// Modo interativo: as linhas se acumulam num buffer, e `run` executa o buffer inteiro,
// como se fosse um arquivo. Assim um `goto` pode citar um rótulo digitado depois.

use std::io::{self, BufRead, Write};
use std::panic;

use crate::rodar_texto;

// recebe a entrada e a saída como parâmetros para os testes poderem simular o terminal
pub fn rodar<E: BufRead, S: Write>(entrada: E, mut saida: S) -> io::Result<()> {
    let mut buffer: Vec<String> = vec![];
    // quadro de main do último `run`; vazio se ele falhou ou se ainda não houve `run`
    let mut ambiente: Option<Vec<String>> = None;

    writeln!(saida, "Interpretador de TAC. Comandos: run, reset, lista, ambiente. Ctrl+D sai.")?;
    write!(saida, "> ")?;
    saida.flush()?;

    // `?` repassa para quem chamou um erro de leitura ou de escrita
    for linha in entrada.lines() {
        let linha = linha?;

        match linha.trim() {
            "run" => {
                // um erro de TAC é um panic; capturado aqui, a sessão e o buffer continuam.
                // a mensagem já foi impressa pelo gancho instalado no main
                let texto = buffer.join("\n");
                ambiente = panic::catch_unwind(|| rodar_texto(&texto).ambiente()).ok();
                if let Some(linhas) = &ambiente {
                    escrever(&mut saida, linhas)?;
                }
            }
            "reset" => buffer.clear(),
            "lista" => escrever(&mut saida, &buffer)?,
            "ambiente" => match &ambiente {
                Some(linhas) => escrever(&mut saida, linhas)?,
                None => writeln!(saida, "nenhum ambiente: use run sem erros primeiro")?,
            },
            _ => buffer.push(linha),
        }

        write!(saida, "> ")?;
        saida.flush()?;
    }

    writeln!(saida)?;
    Ok(())
}

fn escrever<S: Write>(saida: &mut S, linhas: &[String]) -> io::Result<()> {
    for linha in linhas {
        writeln!(saida, "{}", linha)?;
    }
    Ok(())
}
