use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, BufWriter, Write};

use lexlib::token::*;
use lexlib::utils::*;

fn main() {

    // Collect terminal arguments
    let args: Vec<String> = env::args().collect();
    if args.len()  < 3 {
        println!("/!\\ Expected at least 2 arguments, found {}", args.len() - 1);
        return;
    }
    let source_path = args.get(1).unwrap();
    let dest_path = args.get(2).unwrap();

    // read source file to string
    let source = read_file(&source_path)
        .map_err(|e| format!("/!\\ something went wrong reading source file: {}", e))
        .unwrap();
    
    // initialize necessary state variables
    let mut tokens: Vec<Token> = Vec::new();
    let mut id = 0;
    let mut row = 0;
    let mut col = 0;
    let mut scope = 0;

    // Iterate over characters of string and tokenize
    let mut char_itr = source.chars().peekable();
    while let Some(c) = char_itr.peek() {

        // whitespace characters
        if c.is_ascii_whitespace() {
            match c {
                '\n' => {
                    row += 1;
                    col = 0;
                },
                '\r' => col = 0,
                _ => col += 1
            }
            char_itr.next();
        }

        // numbers
        else if c.is_ascii_digit() {
            let number = extract_number(&mut char_itr)
                .map_err(|e| format!("/!\\ Tokenization Error at {}[{}] {}", row, col, e))
                .unwrap();
            
            let len = number.len();

            let token_type = if number.contains('.') {
                TokenType::FLOAT
            } else {
                TokenType::INT
            };

            tokens.push(
                Token::new(id, token_type, scope, row, col, number)
            );

            col += len;
            id += 1;
        }

        // strings
        else if *c == '\'' || *c == '"' {
            let string = extract_string(&mut char_itr)
                .map_err(|e| format!("/!\\ Tokenization Error at {}[{}] {}", row, col, e))
                .unwrap();
            
            let len = string.len();
            
            tokens.push(
                Token::new(id, TokenType::STRING, scope, row, col, string)
            );

            col += len;
            id += 1;
        }

        // operators
        else if is_operator(c) {
            let op = extract_operator(&mut char_itr);
            let len = op.len();

            tokens.push(
                Token::new(id, TokenType::OPERATOR, scope, row, col, op)
            );


            col += len;
            id += 1;
        }

        // identifier
        else if c.is_ascii_alphabetic() || *c == '_' {
            let ident = extract_ident(&mut char_itr);
            let len = ident.len();

            let token_type = if is_keyword(&ident) {
                TokenType::KEYWORD
            } else {
                TokenType::IDENTIFIER
            };

            tokens.push(
                Token::new(id, token_type, scope, row, col, ident)
            );

            col += len;
            id += 1;
        }

        // separators {}[]();,:.
        else if is_separator(c) {
            let sep = String::from(*c);

            match c {
                '{' | '(' => scope += 1,
                '}' | ')' => {
                    if scope == 0 {
                        panic!("/!\\Tokenization Error at {}[{}] {}", row, col, 
                            format!("Non-matching {} separator.", *c));
                    }
                    scope -= 1;
                },
                _ => ()
            }

            tokens.push(
                Token::new(id, TokenType::SEPARATOR, scope, row, col, sep)
            );

            char_itr.next();
            id += 1;
            col += 1;
        }

        // omit lines starting with # (preprocessor grammar)
        else if *c == '#' {
            while let Some(c) = char_itr.peek() {
                if *c == '\n' {
                    break;
                }
                char_itr.next();
                col += 1;
            }
        }

        // panic for other characters
        else {
            panic!("/!\\ Tokenization Error at {}[{}] {}", row, col,
                format!("Invalid character {}", *c));
        }
    }

    // writing collected tokens tor destination file
    write_tokens_to_file(dest_path, tokens)
        .map_err(|e| format!("/!\\ Writing to file error: {}", e))
        .unwrap();
}


fn read_file(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn write_tokens_to_file(path: &str, tokens: Vec<Token>) -> io::Result<()> {
    let file = File::create(path)?;

    let mut buf_writer = BufWriter::new(file);
    for token in tokens {
        buf_writer.write_all(
            token.to_string().as_bytes()
        )?;
    }
    
    Ok(())
}