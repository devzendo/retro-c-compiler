
mod lexer_spec {
    use log::{error, info};
    use crate::parser::lexer;
    use chumsky::Parser;

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn emptiness() {
        let input = "";
        let (lexemes, errs) = lexer().parse(input).into_output_errors();
        info!("{:#?}", lexemes);
        errs.into_iter().for_each(|e| error!("{:?}", e))
    }

    #[test]
    fn listing_1_1() {
        let input = "int main(void) {
                 return 2;
}";
        let (lexemes, errs) = lexer().parse(input).into_output_errors();
        info!("{:#?}", lexemes);
        errs.into_iter().for_each(|e| error!("{:?}", e))
    }
}