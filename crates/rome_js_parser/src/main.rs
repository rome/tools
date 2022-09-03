use rome_js_parser::{parse_module, parse};
use rome_js_syntax::{JsAnyRoot, source_type};

fn main() {
    let source = "const b3 = f<t>?.();";

    let _root = parse(source, 0, source_type::SourceType::ts()).cast::<JsAnyRoot>();
    println!("{:#?}", _root.unwrap());
}