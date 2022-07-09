use rome_js_syntax::JsIdentifierAssignment;

fn main() {
    use rome_js_semantic::{semantic_model, DeclarationExtensions};
    use rome_js_syntax::{JsReferenceIdentifier, SourceType};
    use rome_rowan::{AstNode, SyntaxNodeCast};

    let source = r#"
function foo() {}; 
foo = bar;
    "#;
    let r = rome_js_parser::parse(
        source,
        0,
        SourceType::js_module(),
    );
    let model = semantic_model(&r.tree());

    let arguments_reference = r
        .syntax()
        .descendants()
        .filter_map(|x| x.cast::<JsIdentifierAssignment>())
        .find(|x| x.text().trim() == "foo")
        .unwrap();
    println!("{:?}", arguments_reference);
    // let arguments_declaration = model.declaration(&arguments_reference);
    // println!("{:?}", arguments_declaration.map(|bind| bind.syntax().clone()));;
    // or
    // let arguments_declaration = arguments_reference.declaration(&model);
}
