use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Lparen, Rparen, Lbrace, Rbrace, Semicolon, Keyword(String), Identifier(String), Constant(u32)
}

pub fn lexer<'src>() -> impl Parser<'src, &'src str, Vec<crate::lexer::Token>, extra::Err<Rich<'src, char>>> {
    let lparen = just('(').to(crate::lexer::Token::Lparen).padded();
    let rparen = just(')').to(crate::lexer::Token::Rparen).padded();
    let lbrace = just('{').to(crate::lexer::Token::Lbrace).padded();
    let rbrace = just('}').to(crate::lexer::Token::Rbrace).padded();
    let semicolon = just(';').to(crate::lexer::Token::Semicolon).padded();
    let keyword = text::keyword("int").or(text::keyword("void")).or(text::keyword("return")).map(|x: &str| crate::lexer::Token::Keyword(x.to_owned()));
    let identifier = text::ascii::ident().map(|x: &str| crate::lexer::Token::Identifier(x.to_owned())).padded();
    let constant = text::int(10)
        .map(|s: &str| crate::lexer::Token::Constant(s.parse().unwrap()))
        .padded();

    choice((lparen, rparen, lbrace, rbrace, semicolon, keyword, identifier, constant))
        .repeated()
        .collect()
}

#[cfg(test)]
#[path = "./lexer_spec.rs"]
mod lexer_spec;

