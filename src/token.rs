use super::token_type::TokenType;

#[derive(Default, Debug)]
pub struct Token<T> {
    pub type_token: TokenType,
    pub lexeme: String,
    pub literal: Option<T>,
    pub line: i32,
}

impl<T> Token<T> {
    fn new() -> Token<T> {
        Token {
            type_token: Default::default(),
            lexeme: Default::default(),
            literal: None,
            line: Default::default(),
        }
    }

}

pub trait ToStringTrait {
    fn to_string(&mut self) -> String;
}

impl ToStringTrait for Token<i32> {
    fn to_string(&mut self) -> String {
        format!("With decimal literal: {:?}, {}, {:?}", self.type_token, self.lexeme, self.literal)
    }
}

impl ToStringTrait for Token<f32> {
    fn to_string(&mut self) -> String {
        format!("With floating point literal: {:?}, {}, {:?}", self.type_token, self.lexeme, self.literal)
    }
}

impl ToStringTrait for Token<String> {
    fn to_string(&mut self) -> String {
        format!("With String Literal: {:?}, {}, {:?}", self.type_token, self.lexeme, self.literal)
    }
}


//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_create_token() {
//        token = Token
//    }
//}
