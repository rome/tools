use super::rename::*;
use rome_js_semantic::semantic_model;
use rome_js_syntax::{JsIdentifierBinding, SourceType};
use rome_rowan::{AstNode, BatchMutationExt, SyntaxNodeCast};

/// Search and renames a binding named "a" to "b".
/// Asserts the renaming worked.
pub fn assert_rename_ok(before: &str, expected: &str) {
    let r = rome_js_parser::parse(before, 0, SourceType::js_module());
    let model = semantic_model(&r.tree());

    let binding_a = r
        .syntax()
        .descendants()
        .filter_map(|x| x.cast::<JsIdentifierBinding>())
        .find(|x| x.text() == "a")
        .unwrap();

    let mut batch = r.tree().begin();
    assert!(batch.rename(&model, binding_a, "b"));
    let root = batch.commit();

    let after = root.to_string();
    assert_eq!(expected, after.as_str());
}

/// Search and renames a binding named "a" to "b".
/// Asserts the renaming to fail.
pub fn assert_rename_nok(before: &str) {
    let r = rome_js_parser::parse(before, 0, SourceType::js_module());
    let model = semantic_model(&r.tree());

    let binding_a = r
        .syntax()
        .descendants()
        .filter_map(|x| x.cast::<JsIdentifierBinding>())
        .find(|x| x.text() == "a")
        .unwrap();

    let mut batch = r.tree().begin();
    assert!(!batch.rename(&model, binding_a, "b"));
}
