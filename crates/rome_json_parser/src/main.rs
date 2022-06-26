use std::time::Instant;

// use rome_js_parser::lexer::Lexer;
// use rome_js_parser::{LexContext, Parser, SourceType};
// use rome_js_syntax::T;
use rome_json_parser::lexer::Lexer;
use rome_json_parser::parse;
use rome_json_parser::token_source::TokenSource;
use rome_json_syntax::JsonSyntaxKind;
fn main() {
    let text = r#"{}"#;
    // let text = include_str!("../pass1.json");
    // let start = Instant::now();
    let _root = parse(text, 0);
    for ele in _root.diagnostics() {
        println!("{:?}", ele);
    }
    // println!("{:?}", start.elapsed());
    println!("{:#?}", _root.tree());
    // let mut res = TokenSource::from_str(text, 0);
    // loop {
    //     println!("{:?}, {:?}", res.current(), res.current_range());
    //     res.bump();
    //     if res.current() == JsonSyntaxKind::EOF {
    //         break;
    //     }
    //     // res.advance();
    // }
    // while res.current() != JsonSyntaxKind::EOF {
    //     res.next_non_trivia_token(false);
    //     if res.current() == JsonSyntaxKind::ERROR_TOKEN {
    //         println!("{:?}, {:?}", res.current(), res.current_range());
    //     }
    //     res.advance();
    // }

    // println!("{:#?}", res.trivia_list);

    // parse_json_root();
}
