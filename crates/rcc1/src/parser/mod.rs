use crate::ast::AST;
use crate::lexer::Token;
use chumsky::input::Input;
use chumsky::prelude::*;

pub fn parser<'a, I>() -> impl Parser<'a, I, AST, extra::Err<Rich<'a, Token>>>
where
    I: Input<'a, Token = Token, Span = SimpleSpan>,
{
    just(Token::Lparen).to(AST::Dummy)
}

#[cfg(test)]
#[path = "./parser_spec.rs"]
mod parser_spec;
