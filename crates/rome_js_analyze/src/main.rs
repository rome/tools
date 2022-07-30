use rome_js_syntax::JsIdentifierBinding;

fn main() {
    use rome_js_semantic::{semantic_model, SemanticScopeExtensions};
    use rome_js_syntax::{JsReferenceIdentifier, SourceType};
    use rome_rowan::{AstNode, SyntaxNodeCast};

    let source = r#"
        // function a({a, b}, {c, d}){}
        let result = (a, b, c, d) => {

        }
        let result = 3;
    "#;
    let r = rome_js_parser::parse(source, 0, SourceType::js_module());
    let model = semantic_model(&r.tree());

    let arguments_reference = r
        .syntax()
        .descendants()
        .filter_map(|x| x.cast::<JsIdentifierBinding>())
        .find(|x| x.text() == "a")
        .unwrap();

    // println!("{}", arguments_reference);
    let block_scope = model.scope(&arguments_reference.syntax());
    // let hoisted_scope = model.scope_hoisted_to(arguments_reference.syntax()).unwrap();
    // or
    for binding in block_scope.bindings() {
        dbg!(binding.syntax());
    }
    println!("----------------");
    // for binding in hoisted_scope.bindings() {
    //     dbg!(binding.syntax());
    // }
    // dbg!(&block_scope.);
    // let block_scope = arguments_reference.scope(&model);
}
