use crate::prelude::*;

use crate::js::expressions::computed_member_expression::JsAnyComputedMemberLike;
use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsAnyExpression, JsAnyName, JsAssignmentExpression,
    JsInitializerClause, JsStaticMemberAssignment, JsStaticMemberExpression, JsSyntaxToken,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsStaticMemberExpression;

impl FormatNodeRule<JsStaticMemberExpression> for FormatJsStaticMemberExpression {
    fn fmt_fields(&self, node: &JsStaticMemberExpression, f: &mut JsFormatter) -> FormatResult<()> {
        JsAnyStaticMemberLike::from(node.clone()).fmt(f)
    }
}

#[derive(Debug, Copy, Clone)]
enum StaticMemberLikeLayout {
    /// Forces that there's no line break between the object, operator, and member
    NoBreak,

    /// Breaks the static member expression after the object if the whole expression doesn't fit on a single line
    BreakAfterObject,
}

declare_node_union! {
    pub(crate) JsAnyStaticMemberLike = JsStaticMemberExpression | JsStaticMemberAssignment
}

impl Format<JsFormatContext> for JsAnyStaticMemberLike {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        write!(f, [self.object().format()])?;

        let layout = self.layout()?;

        match layout {
            StaticMemberLikeLayout::NoBreak => {
                write!(f, [self.operator_token().format(), self.member().format()])
            }
            StaticMemberLikeLayout::BreakAfterObject => {
                write!(
                    f,
                    [group(&indent(&format_args![
                        soft_line_break(),
                        self.operator_token().format(),
                        self.member().format(),
                    ]))]
                )
            }
        }
    }
}

impl JsAnyStaticMemberLike {
    fn object(&self) -> SyntaxResult<JsAnyExpression> {
        match self {
            JsAnyStaticMemberLike::JsStaticMemberExpression(expression) => expression.object(),
            JsAnyStaticMemberLike::JsStaticMemberAssignment(assignment) => assignment.object(),
        }
    }

    fn operator_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyStaticMemberLike::JsStaticMemberExpression(expression) => {
                expression.operator_token()
            }
            JsAnyStaticMemberLike::JsStaticMemberAssignment(assignment) => assignment.dot_token(),
        }
    }

    fn member(&self) -> SyntaxResult<JsAnyName> {
        match self {
            JsAnyStaticMemberLike::JsStaticMemberExpression(expression) => expression.member(),
            JsAnyStaticMemberLike::JsStaticMemberAssignment(assignment) => assignment.member(),
        }
    }

    fn layout(&self) -> SyntaxResult<StaticMemberLikeLayout> {
        let parent = self.syntax().parent();
        let object = self.object()?;

        let is_nested = match &parent {
            Some(parent) => {
                if JsAssignmentExpression::can_cast(parent.kind())
                    || JsInitializerClause::can_cast(parent.kind())
                {
                    let no_break = match &object {
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
                        return Ok(StaticMemberLikeLayout::NoBreak);
                    }
                }

                JsAnyStaticMemberLike::can_cast(parent.kind())
                    || JsAnyComputedMemberLike::can_cast(parent.kind())
            }
            None => false,
        };

        if !is_nested && matches!(&object, JsAnyExpression::JsIdentifierExpression(_)) {
            return Ok(StaticMemberLikeLayout::NoBreak);
        }

        let first_non_static_member_ancestor = self.syntax().ancestors().find(|parent| {
            !JsAnyStaticMemberLike::can_cast(parent.kind())
                || JsAnyComputedMemberLike::can_cast(parent.kind())
        });

        let layout = match first_non_static_member_ancestor.and_then(JsAnyExpression::cast) {
            Some(JsAnyExpression::JsNewExpression(_)) => StaticMemberLikeLayout::NoBreak,
            Some(JsAnyExpression::JsAssignmentExpression(assignment)) => {
                if matches!(
                    assignment.left()?,
                    JsAnyAssignmentPattern::JsAnyAssignment(
                        JsAnyAssignment::JsIdentifierAssignment(_)
                    )
                ) {
                    StaticMemberLikeLayout::BreakAfterObject
                } else {
                    StaticMemberLikeLayout::NoBreak
                }
            }
            _ => StaticMemberLikeLayout::BreakAfterObject,
        };

        Ok(layout)
    }
}
