use crate::prelude::*;

use crate::js::declarations::function_declaration::FormatFunction;
use crate::js::expressions::call_arguments::ExpandCallArgumentLayout;
use crate::parentheses::{
    is_callee, is_first_in_statement, is_tag, FirstInStatementMode, NeedsParentheses,
};
use rome_formatter::{write, FormatRuleWithOptions};
use rome_js_syntax::{JsFunctionExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsFunctionExpression {
    call_argument_layout: Option<ExpandCallArgumentLayout>,
}

impl FormatRuleWithOptions<JsFunctionExpression> for FormatJsFunctionExpression {
    type Options = Option<ExpandCallArgumentLayout>;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.call_argument_layout = options;
        self
    }
}

impl FormatNodeRule<JsFunctionExpression> for FormatJsFunctionExpression {
    fn fmt_fields(&self, node: &JsFunctionExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let format_function = FormatFunction::from(node.clone());

        match self.call_argument_layout {
            None => format_function.fmt(f),
            Some(_) => format_function.fmt_with_expand(f, true),
        }
    }

    fn needs_parentheses(&self, item: &JsFunctionExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsFunctionExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        is_callee(self.syntax(), parent)
            || is_tag(self.syntax(), parent)
            || is_first_in_statement(
                self.clone().into(),
                FirstInStatementMode::ExpressionOrExportDefault,
            )
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::JsFunctionExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("console.log((function () {})())", JsFunctionExpression);
        assert_needs_parentheses!("console.log(new (function () {})())", JsFunctionExpression);

        assert_needs_parentheses!("(function() {}).test", JsFunctionExpression);
        assert_not_needs_parentheses!("a => function () {} ", JsFunctionExpression);

        assert_needs_parentheses!(
            "console.log((function () {})`template`)",
            JsFunctionExpression
        );

        assert_needs_parentheses!("export default (function () {})", JsFunctionExpression);
    }
}
