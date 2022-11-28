use crate::prelude::*;

use crate::parentheses::{
    is_binary_like_left_or_right, is_callee, is_member_object, NeedsParentheses,
};
use crate::ts::expressions::type_assertion_expression::type_cast_like_needs_parens;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsAnyExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, TsAsExpression};
use rome_js_syntax::{TsSatisfiesExpression, TsType};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatTsAsExpression;

impl FormatNodeRule<TsAsExpression> for FormatTsAsExpression {
    fn fmt_fields(&self, node: &TsAsExpression, f: &mut JsFormatter) -> FormatResult<()> {
        TsAsOrSatisfiesExpression::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &TsAsExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsAsExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        TsAsOrSatisfiesExpression::from(self.clone()).needs_parentheses_with_parent(parent)
    }
}

declare_node_union! {
    pub(crate) TsAsOrSatisfiesExpression = TsAsExpression | TsSatisfiesExpression
}

impl TsAsOrSatisfiesExpression {
    fn ty(&self) -> SyntaxResult<TsType> {
        match self {
            TsAsOrSatisfiesExpression::TsAsExpression(expression) => expression.ty(),
            TsAsOrSatisfiesExpression::TsSatisfiesExpression(expression) => expression.ty(),
        }
    }

    fn operation_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            TsAsOrSatisfiesExpression::TsAsExpression(expression) => expression.as_token(),
            TsAsOrSatisfiesExpression::TsSatisfiesExpression(expression) => {
                expression.satisfies_token()
            }
        }
    }

    fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        match self {
            TsAsOrSatisfiesExpression::TsAsExpression(expression) => expression.expression(),
            TsAsOrSatisfiesExpression::TsSatisfiesExpression(expression) => expression.expression(),
        }
    }
}

impl Format<JsFormatContext> for TsAsOrSatisfiesExpression {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let expression = self.expression();
        let operation_token = self.operation_token()?;
        let ty = self.ty()?;

        let format_inner = format_with(|f| {
            write!(f, [expression.format(), space(), operation_token.format()])?;

            if f.comments().has_leading_own_line_comment(ty.syntax()) {
                write!(f, [indent(&format_args![hard_line_break(), &ty.format()])])
            } else {
                write!(f, [space(), ty.format()])
            }
        });

        let parent = self.syntax().parent();

        let is_callee_or_object = parent.map_or(false, |parent| {
            is_callee(self.syntax(), &parent) || is_member_object(self.syntax(), &parent)
        });

        if is_callee_or_object {
            write!(f, [group(&soft_block_indent(&format_inner))])
        } else {
            write!(f, [format_inner])
        }
    }
}

impl NeedsParentheses for TsAsOrSatisfiesExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match parent.kind() {
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => true,

            _ => {
                type_cast_like_needs_parens(self.syntax(), parent)
                    || is_binary_like_left_or_right(self.syntax(), parent)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::{SourceType, TsAsExpression};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("5 as number ? true : false", TsAsExpression);
        assert_needs_parentheses!("cond ? x as number : false", TsAsExpression);
        assert_needs_parentheses!("cond ? true : x as number", TsAsExpression);

        assert_needs_parentheses!("class X extends (B as number) {}", TsAsExpression);

        assert_needs_parentheses!("(x as Function)()", TsAsExpression);
        assert_needs_parentheses!("(x as Function)?.()", TsAsExpression);
        assert_needs_parentheses!("new (x as Function)()", TsAsExpression);

        assert_needs_parentheses!("<number>(x as any)", TsAsExpression);
        assert_needs_parentheses!("(x as any)`template`", TsAsExpression);
        assert_needs_parentheses!("!(x as any)", TsAsExpression);
        assert_needs_parentheses!("[...(x as any)]", TsAsExpression);
        assert_needs_parentheses!("({...(x as any)})", TsAsExpression);
        assert_needs_parentheses!(
            "<test {...(x as any)} />",
            TsAsExpression,
            SourceType::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(x as any)}</test>",
            TsAsExpression,
            SourceType::tsx()
        );
        assert_needs_parentheses!("await (x as any)", TsAsExpression);
        assert_needs_parentheses!("(x as any)!", TsAsExpression);

        assert_needs_parentheses!("(x as any).member", TsAsExpression);
        assert_needs_parentheses!("(x as any)[member]", TsAsExpression);
        assert_not_needs_parentheses!("object[x as any]", TsAsExpression);

        assert_needs_parentheses!("(x as any) + (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) + (y as any)", TsAsExpression[1]);

        assert_needs_parentheses!("(x as any) && (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) && (y as any)", TsAsExpression[1]);

        assert_needs_parentheses!("(x as any) in (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) in (y as any)", TsAsExpression[1]);

        assert_needs_parentheses!("(x as any) instanceof (y as any)", TsAsExpression[0]);
        assert_needs_parentheses!("(x as any) instanceof (y as any)", TsAsExpression[1]);

        assert_not_needs_parentheses!("x as number as string", TsAsExpression[1]);
    }
}
