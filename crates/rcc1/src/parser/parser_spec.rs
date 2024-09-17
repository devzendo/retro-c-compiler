mod parser_spec {
    use log::{error, info};
    use chumsky::{
        input::Stream,
        prelude::*,
    }; 
    use crate::lexer::Token;
    use crate::parser::parser;

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn listing_1_1() {
        let tokens = vec![
            Token::Keyword(String::from("int")),
            Token::Identifier(String::from("main")),
            Token::Lparen,
            Token::Keyword(String::from("void")),
            Token::Rparen,
            Token::Lbrace,
            Token::Keyword(String::from("return")),
            Token::Constant(2),
            Token::Semicolon,
            Token::Rbrace,
        ];

        let stream = Stream::from_iter(tokens.into_iter());

        let parser = parser();
        let (ast, errs) = parser.parse(stream).into_output_errors();
        info!("AST output of parser: {:#?}", ast);
        errs.into_iter().for_each(|e| error!("{:?}", e));

    }
}
