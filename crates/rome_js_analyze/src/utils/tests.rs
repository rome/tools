use super::rename::*;
use crate::utils::batch::JsBatchMutation;
use rome_diagnostics::file::FileId;
use rome_js_semantic::semantic_model;
use rome_js_syntax::{JsFormalParameter, JsIdentifierBinding, JsVariableDeclarator, SourceType};
use rome_rowan::{AstNode, BatchMutationExt, SyntaxNodeCast};

/// Search and renames a binding named "a" to "b".
/// Asserts the renaming worked.
pub fn assert_rename_ok(before: &str, expected: &str) {
    let r = rome_js_parser::parse(before, FileId::zero(), SourceType::js_module());
    let model = semantic_model(&r.tree());

    let binding_a = r
        .syntax()
        .descendants()
        .filter_map(|x| x.cast::<JsIdentifierBinding>())
        .find(|x| x.text() == "a")
        .unwrap();

    let mut batch = r.tree().begin();
    assert!(batch.rename_node_declaration(&model, binding_a, "b"));
    let root = batch.commit();

    let after = root.to_string();
    assert_eq!(expected, after.as_str());
}

/// Search and renames a binding named "a" to "b".
/// Asserts the renaming to fail.
pub fn assert_rename_nok(before: &str) {
    let r = rome_js_parser::parse(before, FileId::zero(), SourceType::js_module());
    let model = semantic_model(&r.tree());

    let binding_a = r
        .syntax()
        .descendants()
        .filter_map(|x| x.cast::<JsIdentifierBinding>())
        .find(|x| x.text() == "a")
        .unwrap();

    let mut batch = r.tree().begin();
    assert!(!batch.rename_node_declaration(&model, binding_a, "b"));
}

/// Search a binding named "a" and remove it.
/// Asserts the removal worked.
pub fn assert_remove_ok(before: &str, expected: &str) {
    let r = rome_js_parser::parse(before, FileId::zero(), SourceType::js_module());

    let binding_a = r
        .syntax()
        .descendants()
        .filter_map(|x| x.cast::<JsIdentifierBinding>())
        .find(|x| x.text() == "a")
        .unwrap();

    let mut batch = r.tree().begin();

    let r = if let Some(parameter) = binding_a.parent::<JsFormalParameter>() {
        batch.remove_js_formal_parameter(&parameter)
    } else if let Some(declarator) = binding_a.parent::<JsVariableDeclarator>() {
        batch.remove_js_variable_declarator(&declarator)
    } else {
        panic!("Don't know how to remove this node: {:?}", binding_a);
    };
    assert!(r);
    let root = batch.commit();

    let after = root.to_string();
    assert_eq!(expected, after.as_str());
}

#[macro_export]
macro_rules! assert_rename_ok {
    ($(#[$attr:meta])* $($name:ident, $before:expr, $expected:expr,)*) => {
        $(
            #[test]
            pub fn $name() {
                $crate::utils::tests::assert_rename_ok($before, $expected);
            }
        )*
    };
}

#[macro_export]
macro_rules! assert_rename_nok {
    ($(#[$attr:meta])* $($name:ident, $before:expr,)*) => {
        $(
            #[test]
            pub fn $name() {
                $crate::utils::tests::assert_rename_nok($before);
            }
        )*
    };
}

#[macro_export]
macro_rules! assert_remove_ok {
    ($(#[$attr:meta])* $($name:ident, $before:expr, $expected:expr,)*) => {
        $(
            #[test]
            pub fn $name() {
                $crate::utils::tests::assert_remove_ok($before, $expected);
            }
        )*
    };
}
