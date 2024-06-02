use std::str::CharIndices;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex("![A-Z][A-Z0-9]*")]
    UserDefinedKeyword,

    #[regex("[A-Z][A-Z0-9]*", priority=3)]
    StandardKeyword,

    #[regex("[+-]?[0-9]+", priority=3)]
    Integer,

    #[regex("[+-]?[0-9]+\\.[0-9]+([eE][+-]?[0-9]+)?")]
    Real,

    #[regex(r"'([^'\\]|\\.)*'")]
    String,

    #[regex("#[0-9]+")]
    EntityInstanceName,

    #[regex("@[0-9]+")]
    ValueInstanceName,

    #[regex("#[A-Z][A-Z0-9]*")]
    ConstantEntityName,

    #[regex("@[A-Z][A-Z0-9]*")]
    ConstantValueName,

    #[regex(r"<[a-zA-Z0-9:/?#\[\]@!$&'()*+,;=.%-]+>")]
    Resource,

    #[regex(r"\.[A-Z][A-Z0-9]*\.")]
    Enumeration,

    #[regex(r#"""[0-3]?[0-9A-F]*"""#)]
    Binary,

    #[regex(r"[a-zA-Z0-9+/=]+")]
    SignatureContent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_user_defined_keyword() {
        let mut lex = Token::lexer("!HELLO");
        assert_eq!(lex.next(), Some(Ok(Token::UserDefinedKeyword)));
        assert_eq!(lex.span(), 0..6);
        assert_eq!(lex.slice(), "!HELLO");
    }

    #[test]
    fn test_lex_standard_keyword() {
        let mut lex = Token::lexer("HELLO");
        assert_eq!(lex.next(), Some(Ok(Token::StandardKeyword)));
        assert_eq!(lex.span(), 0..5);
        assert_eq!(lex.slice(), "HELLO");
    }

    #[test]
    fn test_lex_integer() {
        let mut lex = Token::lexer("12345");
        assert_eq!(lex.next(), Some(Ok(Token::Integer)));
        assert_eq!(lex.span(), 0..5);
        assert_eq!(lex.slice(), "12345");
    }

    #[test]
    fn test_lex_real() {
        let mut lex = Token::lexer("123.45");
        assert_eq!(lex.next(), Some(Ok(Token::Real)));
        assert_eq!(lex.span(), 0..6);
        assert_eq!(lex.slice(), "123.45");

        let mut lex = Token::lexer("123.45E-6");
        assert_eq!(lex.next(), Some(Ok(Token::Real)));
        assert_eq!(lex.span(), 0..9);
        assert_eq!(lex.slice(), "123.45E-6");
    }

    #[test]
    fn test_lex_string() {
        let mut lex = Token::lexer("'Hello'");
        assert_eq!(lex.next(), Some(Ok(Token::String)));
        assert_eq!(lex.span(), 0..7);
        assert_eq!(lex.slice(), "'Hello'");

        let mut lex = Token::lexer("'It\\'s a test'");
        assert_eq!(lex.next(), Some(Ok(Token::String)));
        assert_eq!(lex.span(), 0..14);
        assert_eq!(lex.slice(), "'It\\'s a test'");
    }

    #[test]
    fn test_lex_entity_instance_name() {
        let mut lex = Token::lexer("#12345");
        assert_eq!(lex.next(), Some(Ok(Token::EntityInstanceName)));
        assert_eq!(lex.span(), 0..6);
        assert_eq!(lex.slice(), "#12345");
    }

    #[test]
    fn test_lex_value_instance_name() {
        let mut lex = Token::lexer("@12345");
        assert_eq!(lex.next(), Some(Ok(Token::ValueInstanceName)));
        assert_eq!(lex.span(), 0..6);
        assert_eq!(lex.slice(), "@12345");
    }

    #[test]
    fn test_lex_constant_entity_name() {
        let mut lex = Token::lexer("#CONSTANT1");
        assert_eq!(lex.next(), Some(Ok(Token::ConstantEntityName)));
        assert_eq!(lex.span(), 0..10);
        assert_eq!(lex.slice(), "#CONSTANT1");
    }

    #[test]
    fn test_lex_constant_value_name() {
        let mut lex = Token::lexer("@VALUE1");
        assert_eq!(lex.next(), Some(Ok(Token::ConstantValueName)));
        assert_eq!(lex.span(), 0..7);
        assert_eq!(lex.slice(), "@VALUE1");
    }

    #[test]
    fn test_lex_resource() {
        let mut lex = Token::lexer("<http://example.com>");
        assert_eq!(lex.next(), Some(Ok(Token::Resource)));
        assert_eq!(lex.span(), 0..20);
        assert_eq!(lex.slice(), "<http://example.com>");
    }

    #[test]
    fn test_lex_enumeration() {
        let mut lex = Token::lexer(".ENUM1.");
        assert_eq!(lex.next(), Some(Ok(Token::Enumeration)));
        assert_eq!(lex.span(), 0..7);
        assert_eq!(lex.slice(), ".ENUM1.");
    }

    #[test]
    fn test_lex_binary() {
        let mut lex = Token::lexer(r#"""0012"""#);
        assert_eq!(lex.next(), Some(Ok(Token::Binary)));
        assert_eq!(lex.span(), 0..8);
        assert_eq!(lex.slice(), r#"""0012"""#);
    }

    #[test]
    fn test_lex_signature_content() {
        let mut lex = Token::lexer("dGVzdA==");
        assert_eq!(lex.next(), Some(Ok(Token::SignatureContent)));
        assert_eq!(lex.span(), 0..8);
        assert_eq!(lex.slice(), "dGVzdA==");
    }

    #[test]
    fn test_lex_multiple_tokens() {
        let mut lex = Token::lexer("!HELLO 12345 123.45 'Hello' #12345 @VALUE1");
        
        assert_eq!(lex.next(), Some(Ok(Token::UserDefinedKeyword)));
        assert_eq!(lex.slice(), "!HELLO");
        
        assert_eq!(lex.next(), Some(Ok(Token::Integer)));
        assert_eq!(lex.slice(), "12345");
        
        assert_eq!(lex.next(), Some(Ok(Token::Real)));
        assert_eq!(lex.slice(), "123.45");
        
        assert_eq!(lex.next(), Some(Ok(Token::String)));
        assert_eq!(lex.slice(), "'Hello'");
        
        assert_eq!(lex.next(), Some(Ok(Token::EntityInstanceName)));
        assert_eq!(lex.slice(), "#12345");
        
        assert_eq!(lex.next(), Some(Ok(Token::ConstantValueName)));
        assert_eq!(lex.slice(), "@VALUE1");
    }

    #[test]
    fn test_invalid_input() {
        let mut lex = Token::lexer("!HELLO @12345 @@123");
        
        assert_eq!(lex.next(), Some(Ok(Token::UserDefinedKeyword)));
        assert_eq!(lex.slice(), "!HELLO");
        
        assert_eq!(lex.next(), Some(Ok(Token::ValueInstanceName)));
        assert_eq!(lex.slice(), "@12345");
        
        assert_eq!(lex.next(), Some(Err(())));
        assert_eq!(lex.slice(), "@");
    }
}

pub struct Lexer<'input> {
    // chars: CharIndices<'input>,
    lexer: logos::Lexer<'input, Token>
}

pub enum LexicalError {
    Todo
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            lexer: Token::lexer(input)
        } 
    }
}

impl <'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.lexer.next() {
                Some(token_result) => {
                    match token_result {
                        Ok(token) => {
                            let span = self.lexer.span();
                            return Some(Ok((span.start, token, span.end)))
                        }
                        Err(_) => {
                            return Some(Err(LexicalError::Todo))
                        }
                    }
                }
                None => {
                    return None
                }
            }
        }
    }
}