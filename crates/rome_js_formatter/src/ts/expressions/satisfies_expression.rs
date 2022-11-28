use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use crate::ts::expressions::as_expression::TsAsOrSatisfiesExpression;
use rome_js_syntax::{JsSyntaxNode, TsSatisfiesExpression};

#[derive(Debug, Clone, Default)]
pub struct FormatTsSatisfiesExpression;

impl FormatNodeRule<TsSatisfiesExpression> for FormatTsSatisfiesExpression {
    fn fmt_fields(&self, node: &TsSatisfiesExpression, f: &mut JsFormatter) -> FormatResult<()> {
        TsAsOrSatisfiesExpression::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &TsSatisfiesExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsSatisfiesExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        TsAsOrSatisfiesExpression::from(self.clone()).needs_parentheses_with_parent(parent)
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::{SourceType, TsSatisfiesExpression};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("5 satisfies number ? true : false", TsSatisfiesExpression);
        assert_needs_parentheses!("cond ? x satisfies number : false", TsSatisfiesExpression);
        assert_needs_parentheses!("cond ? true : x satisfies number", TsSatisfiesExpression);

        assert_needs_parentheses!(
            "class X extends (B satisfies number) {}",
            TsSatisfiesExpression
        );

        assert_needs_parentheses!("(x satisfies Function)()", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies Function)?.()", TsSatisfiesExpression);
        assert_needs_parentheses!("new (x satisfies Function)()", TsSatisfiesExpression);

        assert_needs_parentheses!("<number>(x satisfies any)", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies any)`template`", TsSatisfiesExpression);
        assert_needs_parentheses!("!(x satisfies any)", TsSatisfiesExpression);
        assert_needs_parentheses!("[...(x satisfies any)]", TsSatisfiesExpression);
        assert_needs_parentheses!("({...(x satisfies any)})", TsSatisfiesExpression);
        assert_needs_parentheses!(
            "<test {...(x satisfies any)} />",
            TsSatisfiesExpression,
            SourceType::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(x satisfies any)}</test>",
            TsSatisfiesExpression,
            SourceType::tsx()
        );
        assert_needs_parentheses!("await (x satisfies any)", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies any)!", TsSatisfiesExpression);

        assert_needs_parentheses!("(x satisfies any).member", TsSatisfiesExpression);
        assert_needs_parentheses!("(x satisfies any)[member]", TsSatisfiesExpression);
        assert_not_needs_parentheses!("object[x satisfies any]", TsSatisfiesExpression);

        assert_needs_parentheses!(
            "(x satisfies any) + (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) + (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_needs_parentheses!(
            "(x satisfies any) && (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) && (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_needs_parentheses!(
            "(x satisfies any) in (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) in (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_needs_parentheses!(
            "(x satisfies any) instanceof (y satisfies any)",
            TsSatisfiesExpression[0]
        );
        assert_needs_parentheses!(
            "(x satisfies any) instanceof (y satisfies any)",
            TsSatisfiesExpression[1]
        );

        assert_not_needs_parentheses!(
            "x satisfies number satisfies string",
            TsSatisfiesExpression[1]
        );
    }
}
