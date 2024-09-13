mod parser_spec {
    use log::{error, info};
    use chumsky::{
        input::{Stream, ValueInput},
        prelude::*,
    }; 
    use hamcrest2::prelude::*;
    use crate::lexer::Token;
    use crate::parser::parser;

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn listing_1_1() {
        let tokens = vec![
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
        ];

        /*
        This is not compiling - how do the types align?
        let stream = Stream::from_iter(tokens.iter());
        //.spanned((tokens.len()..tokens.len()).into());

        let (ast, errs) = parser().parse(stream).into_output_errors();
        info!("{:#?}", ast);
        errs.into_iter().for_each(|e| error!("{:?}", e));
         */
    }
}
