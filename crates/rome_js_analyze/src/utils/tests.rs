use rome_js_syntax::JsSyntaxNode;
use std::{any::type_name, fmt::Debug};

use super::rename::*;
use crate::utils::batch::JsBatchMutation;
use rome_diagnostics::location::FileId;
use rome_js_semantic::{semantic_model, SemanticModelOptions};
use rome_js_syntax::{
    AnyJsObjectMember, JsFormalParameter, JsIdentifierBinding, JsLanguage, JsVariableDeclarator,
    SourceType,
};
use rome_rowan::{AstNode, BatchMutationExt, SyntaxNodeCast};

/// Search and renames a binding named "a" to "b".
/// Asserts the renaming worked.
pub fn assert_rename_binding_a_to_b_ok(before: &str, expected: &str) {
    let r = rome_js_parser::parse(before, FileId::zero(), SourceType::js_module());
    let model = semantic_model(&r.tree(), SemanticModelOptions::default());

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
pub fn assert_rename_binding_a_to_b_nok(before: &str) {
    let r = rome_js_parser::parse(before, FileId::zero(), SourceType::js_module());
    let model = semantic_model(&r.tree(), SemanticModelOptions::default());

    let binding_a = r
        .syntax()
        .descendants()
        .filter_map(|x| x.cast::<JsIdentifierBinding>())
        .find(|x| x.text() == "a")
        .unwrap();

    let mut batch = r.tree().begin();
    assert!(!batch.rename_node_declaration(&model, binding_a, "b"));
}

/// Search an identifier named "a" and remove the entire node of type Anc around it.
/// Asserts the removal worked.
pub fn assert_remove_identifier_a_ok<Anc: AstNode<Language = JsLanguage> + Debug>(
    before: &str,
    expected: &str,
) {
    let r = rome_js_parser::parse(before, FileId::zero(), SourceType::js_module());

    let identifiers_a: Vec<JsSyntaxNode> = r
        .syntax()
        .descendants()
        .filter(|x| x.tokens().any(|token| token.text_trimmed() == "a"))
        .collect();
    let node_to_remove = match identifiers_a.as_slice() {
        [identifier_a] => identifier_a
            .ancestors()
            .find_map(|ancestor| ancestor.cast::<Anc>())
            .unwrap_or_else(|| {
                panic!(
                    "Trying to remove the {} ancestor of identifier a, but it has no such ancestor",
                    type_name::<Anc>()
                )
            }),
        _ => panic!(
            "Expected exactly one identifier named a, but got {:?}",
            identifiers_a
        ),
    };

    let mut batch = r.tree().begin();
    let batch_result =
        if let Some(parameter) = node_to_remove.syntax().clone().cast::<JsFormalParameter>() {
            batch.remove_js_formal_parameter(&parameter)
        } else if let Some(declarator) = node_to_remove
            .syntax()
            .clone()
            .cast::<JsVariableDeclarator>()
        {
            batch.remove_js_variable_declarator(&declarator)
        } else if let Some(member) = node_to_remove.syntax().clone().cast::<AnyJsObjectMember>() {
            batch.remove_js_object_member(&member)
        } else {
            panic!("Don't know how to remove this node: {:?}", node_to_remove);
        };
    assert!(batch_result);
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
                $crate::utils::tests::assert_rename_binding_a_to_b_ok($before, $expected);
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
                $crate::utils::tests::assert_rename_binding_a_to_b_nok($before);
            }
        )*
    };
}

#[macro_export]
macro_rules! assert_remove_ok {
    ($(#[$attr:meta])* $ancestor:ty, $($name:ident, $before:expr, $expected:expr,)*) => {
        $(
            #[test]
            pub fn $name() {
                $crate::utils::tests::assert_remove_identifier_a_ok::<$ancestor>($before, $expected);
            }
        )*
    };
}

#[test]
pub fn ok_find_attributes_by_name() {
    let r = rome_js_parser::parse(r#"<a a="A" c="C" b="B" />"#, 0.into(), SourceType::jsx());
    let list = r
        .syntax()
        .descendants()
        .find_map(rome_js_syntax::JsxAttributeList::cast)
        .unwrap();
    let [a, c, d] = list.find_by_names(["a", "c", "d"]);
    assert_eq!(
        a.unwrap()
            .initializer()
            .unwrap()
            .value()
            .unwrap()
            .to_string(),
        "\"A\" "
    );
    assert_eq!(
        c.unwrap()
            .initializer()
            .unwrap()
            .value()
            .unwrap()
            .to_string(),
        "\"C\" "
    );
    assert!(d.is_none());
}
