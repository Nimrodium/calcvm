use core::fmt;

use crate::compiler::lexizer::Lexeme;
const FORMAT_PREFIXES: [&'static str; 4] = ["0x", "0b", "0o", "0d"];

#[derive(Debug,PartialEq,Clone)]
pub enum Atomic{
    Integer(isize),
    Float(f64),
    Ident(String),
}
impl Atomic {
    // later change to f64 ? 
    pub fn evaluate(&self) -> isize{
        match self {
            Atomic::Integer(i) => *i,
            Atomic::Float(f) => *f as isize,
            Atomic::Ident(i) => todo!("symbol table unimplemented"),
        }
    }
}
impl fmt::Display for Atomic{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", 
            match self {
                Atomic::Integer(a) => a.to_string(),
                Atomic::Float(a) => a.to_string(),
                Atomic::Ident(a) => a.to_string(),
            }
        )
    }
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Atomic(Atomic),
    OpenP,
    ClosedP,
    Plus,
    Dash,
    Star,
    Slash,
    Carot,
    Eq,
}
impl Token {
    fn int(n:isize) ->Self {
        Self::Atomic(Atomic::Integer(n))
    }
    fn float(f:f64) -> Self {
        Self::Atomic(Atomic::Float(f))
    }
    fn ident(s:&str) -> Self {
        Self::Atomic(Atomic::Ident(s.to_owned()))
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",
            match self { // all this to_string is gross and id prefer if i could just convert Atomic to_string().as_str()
                Token::Atomic(atomic) => atomic.to_string(),
                Token::OpenP => "(".to_string(),
                Token::ClosedP => ")".to_string(),
                Token::Plus => "+".to_string(),
                Token::Dash => "-".to_string(),
                Token::Star => "*".to_string(),
                Token::Slash => "/".to_string(),
                Token::Carot => "^".to_string(),
                Token::Eq => "=".to_string(),
            }
        )
    }
}
#[derive(PartialEq, Debug)]
pub struct TokenContainer {
    pub token: Token,
    pub lexeme: Lexeme,
}
impl TokenContainer {
    fn _test_new(token: Token, lexeme: Lexeme) -> Self {
        Self { token, lexeme }
    }
    fn new(lexeme: Lexeme) -> Result<Self, String> {
        // println!("Processing {lexeme:?}");
        let token = match lexeme.inner.as_str() {
            "+" => Token::Plus,
            "-" => Token::Dash,
            "*" => Token::Star,
            "/" => Token::Slash,
            "^" => Token::Carot,
            "=" => Token::Eq,
            "(" => Token::OpenP,
            ")" => Token::ClosedP,

            // unwrap should be safe as it checks beforehand if all chars are digits
            x if x.chars().all(|c| c.is_digit(10)) => Token::int(x.parse().unwrap()),

            x if FORMAT_PREFIXES.iter().any(|s| x.starts_with(*s)) => {
                let n: String = x.chars().skip(2).collect();
                // nth is safe because starts_with ensures
                let format = x.chars().nth(1).unwrap();
                match format {
                    'd' => Token::int(
                        isize::from_str_radix(&n, 10)
                            .map_err(|e| format!("parsing decimal failed: {e}"))?,
                    ),
                    'x' => Token::int(
                        isize::from_str_radix(&n, 16)
                            .map_err(|e| format!("parsing hexadecimal failed: {e}"))?,
                    ),
                    'b' => Token::int(
                        isize::from_str_radix(&n, 2)
                            .map_err(|e| format!("parsing binary failed: {e}"))?,
                    ),
                    'o' => Token::int(
                        isize::from_str_radix(&n, 8)
                            .map_err(|e| format!("parsing octal failed: {e}"))?,
                    ),
                    _ => Err(format!("{format} is not a valid numerical format"))?,
                }
            }
            // x if x.chars().next().unwrap().is_alphabetic() => Token::Variable(x.to_string()),
            _ => Token::ident(&lexeme.inner),
        };
        // println!("built Token: {token:?}");
        Ok(Self { token, lexeme })
    }
}
pub fn tokenize(lexemes: Vec<Lexeme>) -> Result<Vec<TokenContainer>, String> {
    lexemes
        .into_iter()
        .map(|l| TokenContainer::new(l))
        .collect::<Result<Vec<_>, _>>()
}
#[cfg(test)]
mod tests {
    use crate::compiler::lexizer::lexize;
    use super::*;
    #[test]
    fn test_tokenizer() {
        let lexemes = lexize("2(0xa+0b11)+x");
        let tokens = tokenize(lexemes.clone()).unwrap();
        let expect_tokens = vec![
            Token::int(2),
            Token::OpenP,
            Token::int(10),
            Token::Plus,
            Token::int(3),
            Token::ClosedP,
            Token::Plus,
            Token::ident("x"),
        ];
        let expect = expect_tokens
            .into_iter()
            .zip(lexemes)
            .map(|(t, l)| TokenContainer {
                token: t,
                lexeme: l,
            })
            .collect::<Vec<TokenContainer>>();
        assert_eq!(tokens, expect);
    }
}
