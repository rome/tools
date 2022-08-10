use crate::prelude::*;
use crate::utils::format_class::FormatClass;

use crate::parentheses::{
    is_callee, is_first_in_statement, FirstInStatementMode, NeedsParentheses,
};
use rome_js_syntax::{JsClassExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsClassExpression;

impl FormatNodeRule<JsClassExpression> for FormatJsClassExpression {
    fn fmt_fields(&self, node: &JsClassExpression, f: &mut JsFormatter) -> FormatResult<()> {
        FormatClass::from(&node.clone().into()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsClassExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsClassExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        is_callee(self.syntax(), parent)
            || is_first_in_statement(
                self.syntax(),
                FirstInStatementMode::ExpressionOrExportDefault,
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::parentheses::NeedsParentheses;
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::JsClassExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("console.log((class {})())", JsClassExpression);
        assert_needs_parentheses!("console.log(new (class {})())", JsClassExpression);

        assert_needs_parentheses!("(class {}).test", JsClassExpression);
        assert_not_needs_parentheses!("a => class {} ", JsClassExpression);

        assert_needs_parentheses!("export default (class  {})", JsClassExpression);
    }
}
