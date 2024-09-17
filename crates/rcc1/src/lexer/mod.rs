use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Lparen, Rparen, Lbrace, Rbrace, Semicolon, Keyword(String), Identifier(String), Constant(u32)
}

// TODO the lexer should take a Stream not a &str as input.
pub fn lexer<'src>() -> impl Parser<'src, &'src str, Vec<Token>, extra::Err<Rich<'src, char>>> {
    let lparen = just('(').to(Token::Lparen).padded();
    let rparen = just(')').to(Token::Rparen).padded();
    let lbrace = just('{').to(Token::Lbrace).padded();
    let rbrace = just('}').to(Token::Rbrace).padded();
    let semicolon = just(';').to(Token::Semicolon).padded();
    let keyword = text::keyword("int").or(text::keyword("void")).or(text::keyword("return")).map(|x: &str| Token::Keyword(x.to_owned()));
    let identifier = text::ascii::ident().map(|x: &str| Token::Identifier(x.to_owned())).padded();
    let constant = text::int(10)
        .map(|s: &str| Token::Constant(s.parse().unwrap()))
        .padded();

    let all_tokens = choice((lparen, rparen, lbrace, rbrace, semicolon, keyword, identifier, constant))
        .repeated()
        .collect();

    all_tokens
}

#[cfg(test)]
#[path = "./lexer_spec.rs"]
mod lexer_spec;

