use crate::parentheses::{get_expression_left_side, AnyJsExpressionLeftSide, NeedsParentheses};
use crate::prelude::*;
use crate::utils::FormatStatementSemicolon;
use rome_formatter::{write, CstFormatContext};
use rome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, JsExpressionStatement, JsSyntaxKind,
    JsUnaryOperator,
};
use rome_js_syntax::{AnyJsLiteralExpression, JsExpressionStatementFields};
use rome_rowan::SyntaxNodeOptionExt;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExpressionStatement;

impl FormatNodeRule<JsExpressionStatement> for FormatJsExpressionStatement {
    fn fmt_node(&self, node: &JsExpressionStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let needs_parentheses = self.needs_parentheses(node);
        let is_after_bogus = f
            .elements()
            .start_tag(TagKind::Verbatim)
            .map_or(false, |signal| match signal {
                Tag::StartVerbatim(kind) => kind.is_bogus(),
                _ => unreachable!(),
            });

        if f.options().semicolons().is_as_needed()
            // Don't perform semicolon insertion if the previous statement is an bogus statement.
            && !is_after_bogus
            && (needs_parentheses || needs_semicolon(node))
        {
            write!(f, [text(";")])?;
        }

        if needs_parentheses {
            write!(f, [text("(")])?;
        }

        self.fmt_fields(node, f)?;

        if needs_parentheses {
            write!(f, [text(")")])?;
        }

        Ok(())
    }

    fn fmt_fields(&self, node: &JsExpressionStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExpressionStatementFields {
            expression,
            semicolon_token,
        } = node.as_fields();

        let has_dangling_comments = f.context().comments().has_dangling_comments(node.syntax());

        write!(
            f,
            [
                expression.format(),
                FormatStatementSemicolon::new(semicolon_token.as_ref())
            ]
        )?;

        if has_dangling_comments {
            write!(f, [space(), format_dangling_comments(node.syntax())])?;
        }

        Ok(())
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsExpressionStatement,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

/// Returns `true` if a semicolon is required to keep the semantics of the program.
///
/// Semicolons are optional in most places in JavaScript, but they are sometimes required. Generally,
/// semicolons are necessary if an identifier + start of a statement may form a valid expression. For example:
///
/// ```javascript
/// a
/// ["b"]
/// ```
///
/// The above can either be the computed member expression `a["b"]` or the identifier `a` followed by an
/// expression statement `["b"]`.
///
/// Tokens that need a semicolon are:
///
/// * binary operators: `<`, `+`, `-`,  ...
/// * `[` or `(`
/// * ticks: `\``
fn needs_semicolon(node: &JsExpressionStatement) -> bool {
    use AnyJsExpression::*;

    if !matches!(
        node.syntax().parent().kind(),
        Some(JsSyntaxKind::JS_MODULE_ITEM_LIST | JsSyntaxKind::JS_STATEMENT_LIST)
    ) {
        return false;
    }

    let Ok(expression) = node.expression() else { return false };

    let mut expression: Option<AnyJsExpressionLeftSide> = Some(expression.into());

    while let Some(current) = expression.take() {
        let needs_semi = match &current {
            AnyJsExpressionLeftSide::AnyJsExpression(expression) => match expression {
                JsArrayExpression(_)
                | JsParenthesizedExpression(_)
                | AnyJsLiteralExpression(self::AnyJsLiteralExpression::JsRegexLiteralExpression(
                    _,
                ))
                | TsTypeAssertionExpression(_)
                | JsArrowFunctionExpression(_)
                | JsxTagExpression(_) => true,

                JsTemplateExpression(template) => template.tag().is_none(),
                JsUnaryExpression(unary) => matches!(
                    unary.operator(),
                    Ok(JsUnaryOperator::Plus | JsUnaryOperator::Minus)
                ),

                _ => false,
            },
            AnyJsExpressionLeftSide::JsPrivateName(_) => false,
            AnyJsExpressionLeftSide::AnyJsAssignmentPattern(assignment) => matches!(
                assignment,
                AnyJsAssignmentPattern::JsArrayAssignmentPattern(_)
                    | AnyJsAssignmentPattern::AnyJsAssignment(
                        AnyJsAssignment::JsParenthesizedAssignment(_),
                    )
                    | AnyJsAssignmentPattern::AnyJsAssignment(
                        AnyJsAssignment::TsTypeAssertionAssignment(_),
                    )
            ),
        };

        if needs_semi || current.needs_parentheses() {
            return true;
        }

        expression = match get_expression_left_side(&current) {
            Some(inner) => Some(inner),
            None => return false,
        };
    }

    false
}
