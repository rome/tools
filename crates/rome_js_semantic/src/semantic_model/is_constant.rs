use rome_js_syntax::{JsAnyExpression, JsSyntaxKind};
use rome_rowan::AstNode;

pub fn is_constant(expr: &JsAnyExpression) -> bool {
    for node in expr.syntax().descendants() {
        if matches!(node.kind(), JsSyntaxKind::JS_REFERENCE_IDENTIFIER) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use rome_diagnostics::v2::FileId;
    use rome_js_syntax::{JsIdentifierBinding, JsVariableDeclarator, SourceType};

    use crate::{semantic_model, SemanticModelOptions};

    fn assert_is_const(code: &str, is_const: bool) {
        use rome_rowan::AstNode;
        use rome_rowan::SyntaxNodeCast;
        let r = rome_js_parser::parse(code, FileId::zero(), SourceType::js_module());
        let model = semantic_model(&r.tree(), SemanticModelOptions::default());

        let a_reference = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsIdentifierBinding>())
            .find(|x| x.text() == "a")
            .unwrap();
        let declarator = a_reference.parent::<JsVariableDeclarator>().unwrap();
        let initializer = declarator.initializer().unwrap();
        let expr = initializer.expression().ok().unwrap();

        assert_eq!(model.is_constant(&expr), is_const, "{}", code);
    }

    #[test]
    pub fn ok_semantic_model_is_constant() {
        assert_is_const("const a = 1;", true);
        assert_is_const("const a = 1 + 1;", true);
        assert_is_const("const a = \"a\";", true);
        assert_is_const("const a = b = 1;", true);

        assert_is_const("const a = 1 + f();", false);
        assert_is_const("const a = `${a}`;", false);
        assert_is_const("const a = b = 1 + f();", false);
    }
}
