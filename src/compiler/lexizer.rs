#[derive(PartialEq,Debug,Clone)]
pub struct Lexeme{
    pub inner:String,
    pub line:usize,
    pub column:usize,
}
impl Lexeme{
    pub fn new(inner:&str,line:usize,column:usize) -> Self{
        Lexeme {
            inner: inner.to_owned(),
            line,
            column,
        }
    }
}
pub fn lexize(input:&str) -> Vec<Lexeme>{
    let mut stream = input.chars().peekable();
    let mut advance = true;
    let mut last_char : Option<char> = None;
    let mut lexemes = Vec::<Lexeme>::new();
    let mut lexeme_buffer  = String::new();
    // let mut lexeme_marked_finish_flag = false;
   
    let mut line = 1;
    let mut column = 0;
    let mut current_column_start = 1;
    
    macro_rules! push_lexeme {
    () => {
            if !(lexeme_buffer.is_empty()) {
                println!("pushing '{lexeme_buffer}' {line}:{current_column_start} {column}");
                lexemes.push(Lexeme::new(&lexeme_buffer, line, current_column_start));
                lexeme_buffer.clear();
            }
        };
    }
    'main: loop  {
        let character = if advance {
            match stream.next(){
                Some(c) => {
                    column+=1;
                    println!("fuck {c}: {line}:{column} {current_column_start}");
                    c
                },
                None => break 'main,
            }
        }else{
            advance=true;
            last_char.unwrap()
        };
        match character {
            // single char lexemes
            '('|')'|'+'|'-'|'/'|'%'|'^'|'=' => {
                push_lexeme!();
                current_column_start=column;
                lexeme_buffer.push(character);
                push_lexeme!();
            },
            // deliminators
            ' '|'\t'|'\n' => {
                push_lexeme!();
                if character == '\n'{
                    line+=1;
                    column=1;
                }
            },
            // multi char lexemes
            _ => {
                if lexeme_buffer.is_empty(){
                    current_column_start=column; // mark start of next lexeme
                }
                lexeme_buffer.push(character)
            },
            // _ => return Err(format!("'{lexeme_buffer}{character}'"))
        }
    }
    lexemes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexemes() {
        let test_string = lexize("22( 1+ 3 )");
        let equal = vec![
            Lexeme::new("22", 1, 1),
            Lexeme::new("(", 1, 3),
            Lexeme::new("1", 1, 5),
            Lexeme::new("+", 1, 6),
            Lexeme::new("3", 1, 8),
            Lexeme::new(")", 1, 10),
        ];
        assert_eq!(test_string,equal);
        println!("{test_string:?}");
    }
}