use rome_js_parser::{parse, parse_module};
use rome_js_syntax::{source_type, JsAnyRoot};

fn main() {
    let source = "const b3 = f<t>?.();";

    let _root = parse(source, 0, source_type::SourceType::ts()).cast::<JsAnyRoot>();
    println!("{:#?}", _root.unwrap());
}
