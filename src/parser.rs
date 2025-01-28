use anyhow::{anyhow, Result};
use std::{iter::Peekable, slice::Iter};

use crate::{
    expr::{Expr, Op},
    token::Token,
};

/// --------------------------------------------------------------------
/// 4) PARSER (Recursive Descent)
///
/// Grammar used (no unary for simplicity, but it's easy to extend):
///
/// Expr   = Term (('+' | '-') Term)*
/// Term   = Factor (('*' | '/') Factor)*
/// Factor = Number | '(' Expr ')'
///
/// --------------------------------------------------------------------

pub fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr> {
    // Parse the first term
    let mut expr = parse_term(tokens)?;

    // Then see if we have any + or - operations
    while let Some(&&token) = tokens.peek() {
        match token {
            Token::Plus => {
                tokens.next(); // consume '+'
                let right = parse_term(tokens)?;
                expr = Expr::BinaryOp {
                    op: Op::Add,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Token::Minus => {
                tokens.next(); // consume '-'
                let right = parse_term(tokens)?;
                expr = Expr::BinaryOp {
                    op: Op::Sub,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_term(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr> {
    let mut term = parse_factor(tokens)?;

    // Then see if we have any * or / operations
    while let Some(&&token) = tokens.peek() {
        match token {
            Token::Multiply => {
                tokens.next(); // consume '*'
                let right = parse_factor(tokens)?;
                term = Expr::BinaryOp {
                    op: Op::Mul,
                    left: Box::new(term),
                    right: Box::new(right),
                };
            }
            Token::Divide => {
                tokens.next(); // consume '/'
                let right = parse_factor(tokens)?;
                term = Expr::BinaryOp {
                    op: Op::Div,
                    left: Box::new(term),
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Ok(term)
}

fn parse_factor(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr> {
    // Peek next token
    let token = tokens
        .next()
        .ok_or_else(|| anyhow!("Unexpected end of tokens in factor"))?;

    match token {
        Token::Number(n) => Ok(Expr::Number(*n)),
        Token::LParen => {
            // Parse sub-expression
            let expr = parse_expression(tokens)?;
            // Expect a closing parenthesis
            let next = tokens
                .next()
                .ok_or_else(|| anyhow!("Missing closing parenthesis"))?;
            match next {
                Token::RParen => Ok(expr),
                _ => anyhow::bail!("Expected closing parenthesis, found {:?}", next),
            }
        }
        // If you want to handle unary minus here, you could:
        // Token::Minus => {
        //     let factor = parse_factor(tokens)?;
        //     Ok(Expr::UnaryOp {
        //         op: Op::Sub,
        //         expr: Box::new(factor),
        //     })
        // }
        other => anyhow::bail!("Unexpected token in factor: {:?}", other),
    }
}
