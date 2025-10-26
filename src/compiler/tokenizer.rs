use crate::compiler::lexizer::Lexeme;
const FORMAT_PREFIXES: [&'static str; 4] = ["0x", "0b", "0o", "0d"];
#[derive(PartialEq, Debug)]
pub enum Token {
    Variable(String),
    Number(isize),
    OpenP,
    ClosedP,
    Plus,
    Dash,
    Star,
    Slash,
    Eq,
}
#[derive(PartialEq, Debug)]
pub struct TokenContainer {
    token: Token,
    lexeme: Lexeme,
}
impl TokenContainer {
    fn _test_new(token: Token, lexeme: Lexeme) -> Self {
        Self { token, lexeme }
    }
    fn new(lexeme: Lexeme) -> Result<Self, String> {
        println!("Processing {lexeme:?}");
        let token = match lexeme.inner.as_str() {
            "+" => Token::Plus,
            "-" => Token::Dash,
            "*" => Token::Star,
            "/" => Token::Slash,
            "=" => Token::Eq,
            "(" => Token::OpenP,
            ")" => Token::ClosedP,

            // unwrap should be safe as it checks beforehand if all chars are digits
            x if x.chars().all(|c| c.is_digit(10)) => Token::Number(x.parse().unwrap()),

            x if FORMAT_PREFIXES.iter().any(|s| x.starts_with(*s)) => {
                let n: String = x.chars().skip(2).collect();
                // nth is safe because starts_with ensures
                let format = x.chars().nth(1).unwrap();
                match format {
                    'd' => Token::Number(
                        isize::from_str_radix(&n, 10)
                            .map_err(|e| format!("parsing decimal failed: {e}"))?,
                    ),
                    'x' => Token::Number(
                        isize::from_str_radix(&n, 16)
                            .map_err(|e| format!("parsing hexadecimal failed: {e}"))?,
                    ),
                    'b' => Token::Number(
                        isize::from_str_radix(&n, 2)
                            .map_err(|e| format!("parsing binary failed: {e}"))?,
                    ),
                    'o' => Token::Number(
                        isize::from_str_radix(&n, 8)
                            .map_err(|e| format!("parsing octal failed: {e}"))?,
                    ),
                    _ => Err(format!("{format} is not a valid numerical format"))?,
                }
            }
            x if x.chars().rev().skip(1).all(|c| c.is_digit(10)) => {
                let total: String = x.chars().take(x.len() - 1).collect();
                let format = x.chars().last().unwrap(); // safe because empty lexemes are never created

                match format {
                    'd' => Token::Number(
                        isize::from_str_radix(&total, 10)
                            .map_err(|e| format!("parsing decimal failed: {e}"))?,
                    ),
                    'x' => Token::Number(
                        isize::from_str_radix(&total, 16)
                            .map_err(|e| format!("parsing hexadecimal failed: {e}"))?,
                    ),
                    'b' => Token::Number(
                        isize::from_str_radix(&total, 2)
                            .map_err(|e| format!("parsing binary failed: {e}"))?,
                    ),
                    'o' => Token::Number(
                        isize::from_str_radix(&total, 8)
                            .map_err(|e| format!("parsing octal failed: {e}"))?,
                    ),
                    _ => Err(format!("{format} is not a valid numerical format"))?,
                }
            }
            // x if x.chars().next().unwrap().is_alphabetic() => Token::Variable(x.to_string()),
            _ => Token::Variable(lexeme.inner.clone()),
        };
        println!("built Token: {token:?}");
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
            Token::Number(2),
            Token::OpenP,
            Token::Number(10),
            Token::Plus,
            Token::Number(3),
            Token::ClosedP,
            Token::Plus,
            Token::Variable("x".to_string()),
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
