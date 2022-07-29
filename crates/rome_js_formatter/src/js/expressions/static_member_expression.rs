use crate::prelude::*;

use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsAnyExpression, JsAssignmentExpression, JsStaticMemberExpression,
    JsStaticMemberExpressionFields, JsVariableDeclarator,
};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsStaticMemberExpression;

struct MemberLabel;

impl FormatNodeRule<JsStaticMemberExpression> for FormatJsStaticMemberExpression {
    fn fmt_fields(&self, node: &JsStaticMemberExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsStaticMemberExpressionFields {
            object,
            operator_token,
            member,
        } = node.as_fields();

        write!(f, [object.format()])?;

        let layout = compute_member_layout(node)?;

        match layout {
            StaticMemberExpressionLayout::NoBreak => {
                write!(f, [operator_token.format(), member.format()])
            }
            StaticMemberExpressionLayout::BreakAfterObject => {
                write!(
                    f,
                    [labelled(
                        LabelId::of::<MemberLabel>(),
                        &group(&indent(&format_args![
                            soft_line_break(),
                            operator_token.format(),
                            member.format(),
                        ]))
                    )]
                )
            }
        }
    }
}

enum StaticMemberExpressionLayout {
    /// Forces that there's no line break between the object, operator, and member
    NoBreak,

    /// Breaks the static member expression after the object if the whole expression doesn't fit on a single line
    BreakAfterObject,
}

fn compute_member_layout(
    member: &JsStaticMemberExpression,
) -> FormatResult<StaticMemberExpressionLayout> {
    let parent = member.syntax().parent();

    let nested = parent
        .as_ref()
        .map_or(false, |p| JsStaticMemberExpression::can_cast(p.kind()));

    if let Some(parent) = &parent {
        if JsAssignmentExpression::can_cast(parent.kind())
            || JsVariableDeclarator::can_cast(parent.kind())
        {
            let no_break = match member.object()? {
                JsAnyExpression::JsCallExpression(call_expression) => {
                    !call_expression.arguments()?.args().is_empty()
                }
                JsAnyExpression::TsNonNullAssertionExpression(non_null_assertion) => {
                    match non_null_assertion.expression()? {
                        JsAnyExpression::JsCallExpression(call_expression) => {
                            !call_expression.arguments()?.args().is_empty()
                        }
                        _ => false,
                    }
                }
                _ => false,
            };

            if no_break {
                return Ok(StaticMemberExpressionLayout::NoBreak);
            }
        }
    };

    if !nested && matches!(member.object()?, JsAnyExpression::JsIdentifierExpression(_)) {
        return Ok(StaticMemberExpressionLayout::NoBreak);
    }

    let first_non_static_member_ancestor = member
        .syntax()
        .ancestors()
        .find(|parent| !JsStaticMemberExpression::can_cast(parent.kind()));

    if matches!(
        first_non_static_member_ancestor.and_then(JsAnyExpression::cast),
        Some(JsAnyExpression::JsNewExpression(_))
    ) {
        return Ok(StaticMemberExpressionLayout::NoBreak);
    }

    Ok(StaticMemberExpressionLayout::BreakAfterObject)
}
