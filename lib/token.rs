use std::fmt::Display;

pub struct Token {
    id: usize,
    token_type: TokenType,
    scope: usize,
    row: usize,
    col: usize,
    symbol: String
}

impl Token {
    pub fn new(id: usize, 
            token_type: TokenType,
            scope: usize,
            row: usize,
            col: usize,
            symbol: String
        ) -> Self {
            Self {id, token_type, scope, row, col, symbol}
        }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {}\t\tType: {:?}       \tBlock: {}\tLine: {}[{:>4}]\tSymbol: {}\n", 
            self.id,
            self.token_type, 
            self.scope, 
            self.row, 
            self.col, 
            self.symbol)
    }
}

#[derive(Debug)]
pub enum TokenType {
    INT,
    FLOAT,
    STRING,
    OPERATOR,
    SEPARATOR,
    KEYWORD,
    IDENTIFIER
}