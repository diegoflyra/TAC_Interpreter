

#[derive(Debug, PartialEq, Clone)]
pub enum Variante {

    Identificador(String),
    Numero(i64),
    Parametro(usize),
    Igual,
    Mais,
    Menos,
    Vezes,
    DivididoPor,
    DoisPontos,
    Virgula,
    Goto,
    If,
    IfFalse,
    Param,
    Call,
    Return,
    True,
    False

}

#[derive(Debug, PartialEq, Clone)]
pub struct Ficha {
    pub palavra: Variante,  pub linha:usize,
}

pub fn lexar(texto: &str) -> Vec<Ficha>{

    let mut fichas: Vec<Ficha> = vec![];

    

    for (i, l) in texto.lines().enumerate(){

        let linha = i + 1;

        let mut chars = l.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_whitespace() { 

            } else if c == '=' {
                fichas.push(Ficha{palavra:Variante::Igual, linha:linha});
            } else if c == '+' {
                fichas.push(Ficha {palavra:Variante::Mais, linha:linha});
            } else if c == '-' {
                fichas.push(Ficha{palavra:Variante::Menos, linha:linha});
            } else if c == '*' {
                fichas.push(Ficha{palavra:Variante::Vezes, linha:linha});
            } else if c == '/' {
                fichas.push(Ficha{palavra:Variante::DivididoPor, linha:linha});
            } else if c == ':' {
                fichas.push(Ficha{palavra:Variante::DoisPontos, linha:linha});
            } else if c == ',' {
                fichas.push(Ficha{palavra:Variante::Virgula, linha:linha});
            } else if c.is_alphabetic() {
                let mut palavra: String = String::new();
                palavra.push(c);
                while let Some(c2) = chars.peek(){
                    if !c2.is_alphanumeric() {
                        break;
                    }

                    palavra.push(*c2);
                    chars.next();
                }

                match palavra.as_str() {
                    "goto" => fichas.push(Ficha{palavra:Variante::Goto, linha:linha}),
                    "if" => fichas.push(Ficha{palavra:Variante::If, linha:linha}),
                    "ifFalse" => fichas.push(Ficha{palavra:Variante::IfFalse, linha:linha}),
                    "param" => fichas.push(Ficha{palavra:Variante::Param, linha:linha}),
                    "call" => fichas.push(Ficha{palavra:Variante::Call, linha:linha}),
                    "return" => fichas.push(Ficha{palavra:Variante::Return, linha:linha}),
                    "true" => fichas.push(Ficha{palavra:Variante::True, linha:linha}),
                    "false" => fichas.push(Ficha{palavra:Variante::False, linha:linha}),
                    _ => fichas.push(Ficha{palavra:Variante::Identificador(palavra), linha:linha})

                }
            } else if c.is_ascii_digit(){
                let mut palavra: String = String::new();
                palavra.push(c);
                while let Some(c2) = chars.peek(){
                    if !c2.is_numeric() {
                        break;
                    }

                    palavra.push(*c2);
                    chars.next();
                }

                let v = palavra.parse().unwrap();
                fichas.push(Ficha { palavra: Variante::Numero(v), linha:linha })

            } else if c == '@'{

                let mut palavra: String = String::new();
                while let Some(c2) = chars.peek(){
                    if !c2.is_alphanumeric() {
                        break;
                    }

                    palavra.push(*c2);
                    chars.next();
                }

                match palavra.strip_prefix("parameter") {
                    Some(s) => {
                        
                        let n: usize = match s.parse() {
                            Ok(n) => n,
                            Err(_) => panic!("parâmetro sem número válido na linha {}", linha),
                        };

                        fichas.push(Ficha { palavra: Variante::Parametro(n), linha:linha });


                    },
                    None => panic!("parâmetro inválido")
                }


            } else {
                panic!("caractere inválido '{}' na linha {}", c, linha)
            }
            
        }


        
    }
    return fichas;
}