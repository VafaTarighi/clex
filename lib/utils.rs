use std::{str::Chars, iter::Peekable};

const KEYWORDS: &[&str] = &["auto", "break", "case", "char",
                             "const", "continue", "default", "do",
                             "double", "else", "enum", "extern",
                             "float", "for", "goto", "if",
                             "int", "long", "register", "return",
                             "short", "signed", "sizeof", "static",
                             "struct", "switch", "typedef", "union",
                             "unsigned", "void", "volatile", "while"];

const OPERATORS: &[char] = &['+', '-', '*', '/', '%',
                             '=', '!', '&', '|', '~',
                             '<', '>', '^', '?', ':', '.'];

const SEPARATORS: &[char] = &['{', '}', '(', ')', '[', ']', ';', ',', ':'];

pub fn is_keyword(s: &str) -> bool {
    KEYWORDS.contains(&s)
}

pub fn is_operator(c: &char) -> bool {
    OPERATORS.contains(c)
}

pub fn is_separator(c: &char) -> bool {
    SEPARATORS.contains(c)
}

pub fn extract_number(char_itr: &mut Peekable<Chars>) -> Result<String, String> {
    let mut number_str = String::new();

    while let Some(c) = char_itr.peek() {
        if c.is_ascii_digit() {
            number_str.push(*c);
            char_itr.next();
        } else if *c == '.' {
            number_str.push(*c);
            char_itr.next();
        } else {
            break;
        }
    }
    
    // check if number contains only one decimal point
    if number_str.find('.') != number_str.rfind('.') {
        return Err(
            format!("Found more than one decimal point character {}", number_str)
        )
    }

    // check if number ends with number and not with a decimal point
    if number_str.ends_with(".") {
        return Err(
            format!("Expected digit after decimal point character {}", number_str)
        )
    }

    Ok(number_str)
}

pub fn extract_string(char_itr: &mut Peekable<Chars>) -> Result<String, String> {

    let mut s = String::new();
    let quotation = char_itr.next().unwrap();
    s.push(quotation); // ' or "

    let mut ended = false;
    while let Some(c) = char_itr.peek() {
        if *c == quotation {
            ended = true;
            s.push(char_itr.next().unwrap());
            break;
        }

        s.push(char_itr.next().unwrap());
    }

    if ended {
        Ok(s)
    } else {
        Err(format!("Invalid string literal {}", s))
    }
    
}

pub fn extract_operator(char_itr: &mut Peekable<Chars>) -> String {
    let mut op = String::new();

    op.push(char_itr.next().unwrap());

    if let Some(c) = char_itr.peek() {
        match op.as_str() {
            "+" => if let '+' = c {
                op.push(char_itr.next().unwrap());
            } else if let '=' = c {
                op.push(char_itr.next().unwrap());
            },
            
            "-" => if let '-' = c {
                op.push(char_itr.next().unwrap());
            } else if let '=' = c {
                op.push(char_itr.next().unwrap());
            } else if let '>' = c {
                op.push(char_itr.next().unwrap());
            }

            "=" => if let '=' = c {
                op.push(char_itr.next().unwrap());
            },

            "!" => if let '=' = c {
                op.push(char_itr.next().unwrap());
            },

            "&" => if let '&' = c {
                op.push(char_itr.next().unwrap());
            },

            "|" => if let '|' = c {
                op.push(char_itr.next().unwrap())
            },
            
            "<" => if let '<' = c {
                op.push(char_itr.next().unwrap());
            } else if let '=' = c {
                op.push(char_itr.next().unwrap());
            },

            ">" => if let '>' = c {
                op.push(char_itr.next().unwrap());
            } else if let '=' = c {
                op.push(char_itr.next().unwrap());
            },

            _ => ()
        }
    }

    op
}

pub fn extract_ident(char_itr: &mut Peekable<Chars>) -> String {
    let mut ident = String::new();
    while let Some(c) = char_itr.peek() {
        if c.is_ascii_alphanumeric() || *c == '_' {
            ident.push(char_itr.next().unwrap());
        } else {
            break;
        }
    }

    ident
}