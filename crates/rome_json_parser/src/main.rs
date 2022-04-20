use rome_js_parser::lexer::Lexer;
use rome_js_parser::LexContext;
use rome_js_syntax::T;
fn main() {
    let mut lexer = Lexer::from_str(r#"{"tes": -20}"#, 0);
    while lexer.current() != T![EOF] {
        let a = lexer.next_token(LexContext::default());
        println!("{:?}, {:?}", a, lexer.current_range());
    }
    println!("{:#?}", lexer.finish());
}
