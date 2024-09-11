use chumsky::prelude::*;

// Using parser combinators, this is a lexer and parser in one.
#[derive(Debug, Clone, PartialEq)]
pub enum Token<'src> {
    Lparen, Rparen, Lbrace, Rbrace, Semicolon, Keyword(&'src str), Identifier(&'src str), Constant(u32)
}

pub fn lexer<'src>() -> impl Parser<'src, &'src str, Vec<Token<'src>>, extra::Err<Rich<'src, char>>> {
    let lparen = just('(').to(Token::Lparen).padded();
    let rparen = just(')').to(Token::Rparen).padded();
    let lbrace = just('{').to(Token::Lbrace).padded();
    let rbrace = just('}').to(Token::Rbrace).padded();
    let semicolon = just(';').to(Token::Semicolon).padded();
    let keyword = text::keyword("int").or(text::keyword("void")).or(text::keyword("return")).map(|x: &str| Token::Keyword(x));
    let identifier = text::ascii::ident().map(|x: &str| Token::Identifier(x)).padded();
    let constant = text::int(10)
        .map(|s: &str| Token::Constant(s.parse().unwrap()))
        .padded();

    choice((lparen, rparen, lbrace, rbrace, semicolon, keyword, identifier, constant))
        .repeated()
        .collect()
}

/*
pub fn parser() -> impl Parser<char, AST, Simple<char>> {
    // To be filled in later...
}
*/

#[cfg(test)]
#[path = "./lexer_spec.rs"]
mod lexer_spec;
