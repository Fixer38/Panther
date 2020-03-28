#[derive(Debug)]
pub enum TokenType {  
    // use for default derive macro as it's not implemented for enums yet
    DEFAULT,
    // Single-character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens for comparing
    BANG,
    BANG_EQUAL,
    EQUAL, 
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    
    // literals
    IDENTIFIER,
    STRING,
    NUMBER,

    //keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    // End of Line
    EOF
}

impl Default for TokenType {
    fn default() -> Self { TokenType::DEFAULT }
}
