use crate::prelude::*;

use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsAnyExpression, JsAnyLiteralExpression,
    JsAssignmentExpression, JsStaticMemberExpression, JsStaticMemberExpressionFields,
    JsVariableDeclarator,
};
use rome_rowan::{AstNode, SyntaxNodeCast};

#[derive(Debug, Clone, Default)]
pub struct FormatJsStaticMemberExpression;

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
            StaticMemberExpressionLayout::Inline => {
                write!(f, [operator_token.format(), member.format()])
            }
            StaticMemberExpressionLayout::Grouped => {
                write!(
                    f,
                    [group_elements(&indent(&format_args![
                        soft_line_break(),
                        operator_token.format(),
                        member.format(),
                    ]))]
                )
            }
        }
    }
}

enum StaticMemberExpressionLayout {
    Inline,
    Grouped,
}

fn compute_member_layout(
    member: &JsStaticMemberExpression,
) -> FormatResult<StaticMemberExpressionLayout> {
    let parent = member.syntax().parent();

    // ((parent.type === "AssignmentExpression" ||
    //     parent.type === "VariableDeclarator") &&
    //     ((isCallExpression(node.object) && node.object.arguments.length > 0) ||
    //         (node.object.type === "TSNonNullExpression" &&
    //     isCallExpression(node.object.expression) &&
    //     node.object.expression.arguments.length > 0) ||
    //     objectDoc.label === "member-chain"));

    let nested = parent
        .as_ref()
        .map_or(false, |p| JsStaticMemberExpression::can_cast(p.kind()));

    if let Some(parent) = &parent {
        if JsAssignmentExpression::can_cast(parent.kind())
            || JsVariableDeclarator::can_cast(parent.kind())
        {
            let inline = match member.object()? {
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

            if inline {
                return Ok(StaticMemberExpressionLayout::Inline);
            }
        }
    };

    if !nested && matches!(member.object()?, JsAnyExpression::JsIdentifierExpression(_)) {
        return Ok(StaticMemberExpressionLayout::Inline);
    }

    let first_non_static_member_ancestor = member
        .syntax()
        .ancestors()
        .find(|parent| !JsStaticMemberExpression::can_cast(parent.kind()));

    if matches!(
        first_non_static_member_ancestor.and_then(JsAnyExpression::cast),
        Some(JsAnyExpression::JsNewExpression(_))
    ) {
        return Ok(StaticMemberExpressionLayout::Inline);
    }

    Ok(StaticMemberExpressionLayout::Grouped)
}
