
mod lexer_spec {
    use log::{error, info};
    use chumsky::Parser;
    use hamcrest2::prelude::*;
    use crate::lexer::{lexer, Token};

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn emptiness() {
        let input = "";
        let (tokens, errs) = lexer().parse(input).into_output_errors();
        info!("{:#?}", tokens);
        errs.into_iter().for_each(|e| error!("{:?}", e));
        assert!(tokens.unwrap().is_empty());
    }

    #[test]
    fn listing_1_1() {
        let input = "int main(void) {
                 return 2;
}";
        let (tokens, errs) = lexer().parse(input).into_output_errors();
        info!("{:#?}", tokens);
        errs.into_iter().for_each(|e| error!("{:?}", e));
        assert_that!(tokens.unwrap(), eq(vec![
            Token::Keyword("int"),
            Token::Identifier("main"),
            Token::Lparen,
            Token::Keyword("void"),
            Token::Rparen,
            Token::Lbrace,
            Token::Keyword("return"),
            Token::Constant(2),
            Token::Semicolon,
            Token::Rbrace,
        ]));
    }
}