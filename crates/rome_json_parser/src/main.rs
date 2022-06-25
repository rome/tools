// use rome_js_parser::lexer::Lexer;
// use rome_js_parser::{LexContext, Parser, SourceType};
// use rome_js_syntax::T;
use rome_json_parser::lexer::Lexer;
use rome_json_parser::parse_json_root;
use rome_json_parser::token_source::TokenSource;
use rome_json_syntax::JsonSyntaxKind;
fn main() {
    let mut res = TokenSource::from_str(
        r#"{
        "test": 20
    } "#,
        0,
    );
    loop {
        println!("{:?}, {:?}", res.current(), res.current_range());
        res.bump();
        if res.current() == JsonSyntaxKind::EOF {
            break;
        }
        // res.advance();
    }
    // while res.current() != JsonSyntaxKind::EOF {
    //     // res.next_non_trivia_token(false);
    //     res.advance();
    // }

    println!("{:#?}", res.trivia_list);

    // parse_json_root();
}
