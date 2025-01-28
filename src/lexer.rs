use crate::token::Token;
use anyhow::Result;

fn tokenize(expr: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            _ => {
                return Err(anyhow::anyhow!("unexpected character: {}", ch));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operators() {
        let result = tokenize("+ - * /").unwrap();
        assert_eq!(
            result,
            vec![Token::Plus, Token::Minus, Token::Multiply, Token::Divide]
        );
    }

    #[test]
    fn test_parentheses() {
        let result = tokenize("()").unwrap();
        assert_eq!(result, vec![Token::LParen, Token::RParen]);
    }

    #[test]
    fn test_whitespace_handling() {
        let result = tokenize("   +\t*\n/  ").unwrap();
        assert_eq!(result, vec![Token::Plus, Token::Multiply, Token::Divide]);
    }

    #[test]
    fn test_complex_expression() {
        let result = tokenize("(+ * /)").unwrap();
        assert_eq!(
            result,
            vec![
                Token::LParen,
                Token::Plus,
                Token::Multiply,
                Token::Divide,
                Token::RParen
            ]
        );
    }

    #[test]
    fn test_empty_input() {
        let result = tokenize("").unwrap();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_only_whitespace() {
        let result = tokenize("   \t\n   ").unwrap();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_invalid_character() {
        let result = tokenize("+ a *");
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_invalid_characters() {
        let result = tokenize("@#$");
        assert!(result.is_err());
    }
}
