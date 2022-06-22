use rome_js_syntax::JsAnyExpression;

pub fn is_boolean_literal(expr: JsAnyExpression) -> bool {
    matches!(
        expr,
        JsAnyExpression::JsAnyLiteralExpression(
            rome_js_syntax::JsAnyLiteralExpression::JsBooleanLiteralExpression(_)
        )
    )
}
