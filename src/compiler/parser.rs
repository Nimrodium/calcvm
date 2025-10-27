use std::{fmt::Display, iter::Peekable};
use crate::compiler::tokenizer::{Atomic, Token, TokenContainer};
const FACTOR_ERROR : &str = "f: expected one of IDENT|EXPR:";
const TERM_ERROR : &str = "t: expected EXPR: ";
const EXPR_ERROR : &str = "e:";
macro_rules! factor_error {
    ($t:expr) => {
        Err(format!("{FACTOR_ERROR} found `{}`; a valid expression must follow an operator, not `{}`",$t,$t))?
    };
}
#[derive(Debug,PartialEq,Clone)]
pub enum Expression{
    
    Add {
        left:Box<Expression>,
        right:Box<Expression>,
    },
    Subtract{
        left:Box<Expression>,
        right:Box<Expression>,
    },
    Negate{
        right:Box<Expression>,
    },
    Multiply{
        left:Box<Expression>,
        right:Box<Expression>,
    },
    Divide{
        left:Box<Expression>,
        right:Box<Expression>,
    },
    Modulo{
        left:Box<Expression>,
        right:Box<Expression>,
    },
    Exponent{
        left:Box<Expression>,
        right:Box<Expression>,
    },
    Assign{
        left:Box<Expression>,
        right:Box<Expression>,
    },
    Leaf(Atomic),
}
impl Expression{
    fn leaf(a:&Atomic) -> Self {
        Self::Leaf(a.to_owned())
    }
  
    pub fn parse(tokens:Vec<TokenContainer>) -> Result<Self,String> {
        let mut token_stream: Peekable<std::slice::Iter<'_, TokenContainer>> = tokens.iter().peekable();
        Ok(Self::parse_e(&mut token_stream)?)
        
    }// x=2(1+3) 
    fn parse_e<'a>( tokens: &mut Peekable<std::slice::Iter<'_, TokenContainer>>) -> Result<Self,String>{
     
        let mut left = Self::parse_t(tokens)?; // -> a:T
        while let Some(token) = tokens.peek(){
            match token.token{
                Token::ClosedP => {tokens.next();break}
                Token::Plus => {
                    tokens.next();
                    left = Self::Add { left: Box::new(left), right: Box::new(Self::parse_t(tokens)?) };
                },
                Token::Dash => {
                    tokens.next();
                    left = Self::Subtract { left: Box::new(left), right: Box::new(Self::parse_t(tokens)?) };
                },
                _ => break,
            }
        }
        println!("e: {left}");
        Ok(left)
    }
    // law:
    //  return on +-EOF
    //  elaborate on */
    //  impossible for (^
    // let F = parse_f; in
    fn parse_t<'a>( tokens: &mut Peekable<std::slice::Iter<'_, TokenContainer>>) -> Result<Self,String>{

        // left = 2^(1+1)
        // 
        // F F+ F- -> T
        // F* -> F*F
        // F/ -> F
        // get left hand side of Term
        let mut left = Self::parse_f(tokens)?; // -> a:F 
        while let Some(token) = tokens.peek(){
            match token.token{
                // Token::ClosedP => {tokens.next();break}
                Token::Star => {
                    tokens.next();
                    left = Self::Multiply { left: Box::new(left), right: Box::new(Self::parse_f(tokens)?) };
                },
                Token::Slash => {
                    tokens.next();
                    left = Self::Divide { left: Box::new(left), right: Box::new(Self::parse_f(tokens)?) };
                },
                _ => break,
            }
        }
        println!("t: {left}");
        Ok(left)
    }
    // really the stream should consume.
    // law: 
    //  return on */+-
    //  elaborate on (^
    //  legal = [ x x[(^*/+-)]y  ]
    //  illegal = [ = ( ^ * / + - ) ]
    fn parse_f<'a>(tokens: &mut Peekable<std::slice::Iter<'_, TokenContainer>>) -> Result<Self,String>{
        let next = tokens.next().ok_or(format!("{FACTOR_ERROR} found `None`"))?;
        let x = match &next.token {
            Token::Atomic(a) => {
                Ok(
                    match tokens.peek(){
                        Some(t) => match t.token{
                            Token::Atomic(_) => todo!(), // x y, not valid or implicit call ? 
                            Token::OpenP => todo!(), // x() implicit multiplcation or call ?
                            Token::Carot => { // x^ -> (x^y)
                                tokens.next(); // skip ^
                                Self::Exponent {
                                    left: Box:: new(Self::leaf(a)),
                                    right: Box:: new( Self::parse_f(tokens)? )
                                }
                            }, 
                            Token::Eq => Err(format!("{FACTOR_ERROR} found `_ =`; assignment is not allowed in an expression body."))?, // x= , invalid in this context = is only allowed in top most, before parse_e even. 
                            // consume ) on x) as the expected behavior is (x+y) -> z not z)
                            // Token::ClosedP => {tokens.next(); Self::leaf(a)},
                            _ => {Self::leaf(a)},
                        },
                        
                        None => Self::leaf(a),
                    }
                )
            }
            Token::OpenP => Ok( {
                let left = Self::parse_e(tokens)?;
                // check for (...)^...
                if let Some(nx) = tokens.peek(){
                    match nx.token {
                        Token::Carot => {tokens.next();Self::Exponent { left: Box::new(left), right: Box::new(Self::parse_f(tokens)?) }},
                        _ => left
                    }
                }else{left}
            } ), 
            Token::ClosedP => factor_error!(Token::ClosedP), // [(^*/+-]): illegal.
            Token::Plus => Ok(Self::parse_f(tokens)?), // +2 -> num 2; +x -> var x
            Token::Dash => Ok(Expression::Negate { right: Box::new(Self::parse_f(tokens)?) }),
            Token::Star => factor_error!(Token::Star),
            Token::Slash => factor_error!(Token::Slash),
            Token::Eq => factor_error!(Token::Eq),
            Token::Carot => factor_error!(Token::Carot), 
        };
        println!("f: {}",x.clone().unwrap());
        x
    }
    pub fn evaluate(&self) -> isize {
        match self {
            Self::Add { left, right } => left.evaluate()+right.evaluate(),
            Self::Subtract { left, right } =>left.evaluate()-right.evaluate(),
            Self::Negate { right } => -(right.evaluate()),
            Self::Multiply { left, right } =>left.evaluate()*right.evaluate(),
            Self::Divide { left, right } => left.evaluate()/right.evaluate(),
            Self::Modulo { left, right } => left.evaluate()%right.evaluate(),
            Self::Exponent { left, right } =>{
                let a = left.evaluate();
                let b = right.evaluate();
                if b.is_negative(){
                    1/a.pow(b as u32)
                }else{
                    a.pow(b as u32)
                }
            }
            // uhmmmm idk actually, this would need to maybe have a side-effect of mapping right to a symbol table ?
            Self::Assign { left: _, right } => right.evaluate(), 
            // Expression::Number(x) => *x,
            // // and this would read the symbol table
            // Expression::Variable(_) => todo!("symbol table not implemented"),
            Self::Leaf(a) => a.evaluate(),
        }
    }
}

impl Display for Expression{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self{
            Self::Add { left, right } => format!("({left}+{right})"),
            Self::Subtract { left, right } => format!("({left}-{right})"),
            Self::Negate { right } => format!("(-{right})"),
            Self::Multiply { left, right } => format!("({left}*{right})"),
            Self::Divide { left, right } => format!("({left}/{right})"),
            Self::Modulo { left, right } => format!("({left}%{right})"),
            Self::Exponent { left, right } => format!("({left}^{right})"),
            Self::Assign { left, right } => format!("({left}={right})"),
            Self::Leaf(atomic) => format!("{atomic}"),
                    };
        write!(f,"{s}")
    }
}

pub fn parse(tokens:Vec<TokenContainer>) -> Result<Expression,String>{
    Expression::parse(tokens).map_err(|e|format!("AST Generation Failed: {e}"))
}

#[cfg(test)]
mod tests{
    use crate::compiler::{lexizer::lexize, tokenizer::tokenize};
    use super::*;
    #[test]
    fn test_parser(){
        macro_rules! int {
            ($i:expr) => {
                Box::new(Expression::Leaf(Atomic::Integer($i)))
            };
        }
        let tests = [

            ("1+1", 1+1),
            ("1+2*3", 1+2*3),
            ("(1+2)*3", (1+2)*3),
            ("1*2*3*4+5", 1*2*3*4+5),
            ("--2", --2),
            ("1+2^(1+1)*2", 1+2_isize.pow(1+1)*2),

            ("((1+2)+3)", ((1+2)+3)),
            ("(1+(2*(3+4)))", 1+(2*(3+4))),
            ("((1+2)*(3+4))", (1+2)*(3+4)),

            ("-1+2", -1+2),
            ("-(1+2)", -(1+2)),
            ("-(-3)", -(-3)),

            ("2^3^2", 2_isize.pow(3_isize.pow(2) as u32)), // right-associative
            ("-2^3",-2_isize.pow(3)),
            ("(2^3)^2", (2_isize.pow(3)).pow(2)), // grouping check
            ("2^3*2", 2_isize.pow(3)*2), // precedence: ^ before *
            ("2*3^2", 2*3_isize.pow(2)),

            ("1+2*3^2", 1+2*3_isize.pow(2)),
            ("(1+2)*(3+4)^2", (1+2)*(3+4_isize).pow(2)),

            ("--1+2", --1+2),
            ("---2", -(-(-2))),

            ("8/4/2", (8/4)/2),
            ("10/(3+2)", 10/(3+2)),

            ("-(2^3)+4*2", -(2_isize.pow(3)) + 4*2),
            ("2^(1+1)*3-4/2", 2_isize.pow(1+1)*3 - 4/2)
        ];
        for (i,(input,evaluated)) in tests.iter().enumerate(){
            println!("[{i}]\ninput:\t({input})");
            let ast = parse(tokenize(lexize(input)).unwrap()).unwrap();
            let ast_str = ast.to_string();
            // let ast_str = ast.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
            println!("ast:\t{ast_str}");
            // assert_eq!(ast_str,*output);
            assert_eq!(ast.evaluate(),*evaluated)
        }
    }
}