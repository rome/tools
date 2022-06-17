use crate::prelude::*;
use crate::utils::object::write_member_name;
use crate::utils::JsAnyBinaryLikeExpression;
use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsAnyAssignmentPattern, JsAnyExpression, JsAnyObjectMemberName, JsAssignmentExpression,
    JsPropertyObjectMember, JsSyntaxKind,
};
use rome_js_syntax::{JsAnyLiteralExpression, JsSyntaxNode};
use rome_rowan::{declare_node_union, AstNode, SyntaxResult};

declare_node_union! {
    pub(crate) JsAnyAssignmentLike = JsPropertyObjectMember | JsAssignmentExpression

}

declare_node_union! {
    pub(crate) LeftAssignmentLike = JsAnyAssignmentPattern | JsAnyObjectMemberName
}

/// Determines how a assignment like be formatted
///
/// Assignment like are:
/// - Assignment
/// - Object property member
#[derive(Debug)]
pub(crate) enum AssignmentLikeLayout {
    /// First break right-hand side, then after operator.
    /// ```js
    /// {
    ///   "array-key": [
    ///     {
    ///       "nested-key-1": 1,
    ///       "nested-key-2": 2,
    ///     },
    ///   ]
    /// }
    /// ```
    Fluid,
    /// First break after operator, then the sides are broken independently on their own lines.
    /// There is a soft line break after operator token.
    /// ```js
    /// {
    ///     "enough-long-key-to-break-line":
    ///         1 + 2,
    ///     "not-long-enough-key":
    ///         "but long enough string to break line",
    /// }
    /// ```
    BreakAfterOperator,
    /// First break right-hand side, then left-hand side. There are not any soft line breaks
    /// between left and right parts
    /// ```js
    /// {
    ///     key1: "123",
    ///     key2: 123,
    ///     key3: class MyClass {
    ///        constructor() {},
    ///     },
    /// }
    /// ```
    NeverBreakAfterOperator,

    /// This is a special layout usually used for long variable declarations or assignment expressions
    /// This layout is hit, usually, when we are in the "middle" of the chain:
    ///
    /// ```js
    /// var a =
    ///     loreum =
    ///     ipsum =
    ///         "foo";
    /// ```
    ///
    /// Given the previous snippet, then `loreum` and `ipsum` will be formatted using the [Chain] layout.
    Chain,

    /// This is a special layout usually used for long variable declarations or assignment expressions
    /// This layout is hit, usually, when we are in the end of a chain:
    /// ```js
    /// var a = loreum = ipsum = "foo";
    /// ```
    ///
    /// Given the previous snippet, then `"foo"` formatted  using the [ChainTail] layout.
    ChainTail,
}

impl JsAnyAssignmentLike {
    fn right(&self) -> SyntaxResult<JsAnyExpression> {
        match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => property.value(),
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => assignment.right(),
        }
    }
}

const MIN_OVERLAP_FOR_BREAK: u8 = 3;

impl JsAnyAssignmentLike {
    fn write_left(&self, f: &mut JsFormatter) -> FormatResult<bool> {
        match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => {
                let width = write_member_name(&property.name()?, f)?;
                let text_width_for_break =
                    (f.context().tab_width() + MIN_OVERLAP_FOR_BREAK) as usize;
                Ok(width < text_width_for_break)
            }
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => {
                let left = assignment.left()?;
                write!(f, [group_elements(&left.format())])?;
                Ok(false)
            }
        }
    }

    fn write_operator(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => {
                let colon_token = property.colon_token()?;
                write!(f, [colon_token.format()])
            }
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => {
                let operator_token = assignment.operator_token()?;
                write!(f, [space_token(), operator_token.format()])
            }
        }
    }

    /// Returns the layout variant for an assignment like depending on right expression and left part length
    /// [Prettier applies]: https://github.com/prettier/prettier/blob/main/src/language-js/print/assignment.js
    fn layout(&self, is_left_short: bool) -> FormatResult<AssignmentLikeLayout> {
        let right = self.right()?;
        if let Some(layout) = self.is_chain_formatting()? {
            Ok(layout)
        } else if is_break_after_operator(&right)? {
            Ok(AssignmentLikeLayout::BreakAfterOperator)
        } else if is_left_short {
            Ok(AssignmentLikeLayout::NeverBreakAfterOperator)
        } else if matches!(
            right,
            JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsStringLiteralExpression(_)
            )
        ) {
            Ok(AssignmentLikeLayout::BreakAfterOperator)
        } else if self.is_never_break_after_operator()? {
            Ok(AssignmentLikeLayout::NeverBreakAfterOperator)
        } else {
            Ok(AssignmentLikeLayout::Fluid)
        }
    }

    /// Checks if the right node is entitled of the chain formatting,
    /// and if so, it return the layout type
    fn is_chain_formatting(&self) -> SyntaxResult<Option<AssignmentLikeLayout>> {
        let right = self.right()?;
        let right_is_tail = !matches!(right, JsAnyExpression::JsAssignmentExpression(_));
        // Here we surf the upper levels and make sure that the current node
        // is eligible of chain formatting
        //
        // The chain goes up two levels, by checking up to the great parent if all the conditions
        // are correctly met.
        let upper_chain_is_eligible =
            // First, we check if the current node is an assignment expression
            if let JsAnyAssignmentLike::JsAssignmentExpression(assignment) = self {
                assignment.syntax().parent().map_or(false, |parent| {
                    // Then we check if the parent is assignment expression or variable declarator
                    if matches!(
                        parent.kind(),
                        JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                            | JsSyntaxKind::JS_VARIABLE_DECLARATOR
                    ) {
                        let great_parent_kind = parent.parent().map(|n| n.kind());
                        // Finally, we check the great parent.
                        // The great parent triggers the eligibility when
                        // - the current node that we were inspecting is not a "tail"
                        // - or the great parent is not an expression statement or a variable declarator
                        !right_is_tail
                            || !matches!(
                                great_parent_kind,
                                Some(
                                    JsSyntaxKind::JS_EXPRESSION_STATEMENT
                                        | JsSyntaxKind::JS_VARIABLE_DECLARATOR
                                )
                            )
                    } else {
                        false
                    }
                })
            } else {
                false
            };

        let result = if upper_chain_is_eligible {
            if right_is_tail {
                Some(AssignmentLikeLayout::ChainTail)
            } else {
                Some(AssignmentLikeLayout::Chain)
            }
        } else {
            None
        };

        Ok(result)
    }

    fn is_never_break_after_operator(&self) -> SyntaxResult<bool> {
        let right = self.right()?;
        if let JsAnyExpression::JsCallExpression(call_expression) = &right {
            if call_expression.callee()?.syntax().text() == "require" {
                return Ok(true);
            }
        }

        if matches!(
            right,
            JsAnyExpression::JsClassExpression(_)
                | JsAnyExpression::JsTemplate(_)
                | JsAnyExpression::JsAnyLiteralExpression(
                    JsAnyLiteralExpression::JsBooleanLiteralExpression(_),
                )
                | JsAnyExpression::JsAnyLiteralExpression(
                    JsAnyLiteralExpression::JsNumberLiteralExpression(_)
                )
        ) {
            return Ok(true);
        }

        Ok(false)
    }
}

/// Checks if the function is entitled to be printed with layout [AssignmentLikeLayout::BreakAfterOperator]
pub(crate) fn is_break_after_operator(right: &JsAnyExpression) -> SyntaxResult<bool> {
    if has_new_line_before_comment(right.syntax()) {
        return Ok(true);
    }

    // head is a long chain, meaning that right -> right are both assignment expressions
    if let JsAnyExpression::JsAssignmentExpression(assignment) = right {
        let right = assignment.right()?;
        if matches!(right, JsAnyExpression::JsAssignmentExpression(_)) {
            return Ok(true);
        }
    }

    if JsAnyBinaryLikeExpression::cast(right.syntax().clone())
        .map_or(false, |expression| !expression.should_inline())
    {
        return Ok(true);
    }

    if matches!(right, JsAnyExpression::JsSequenceExpression(_)) {
        return Ok(true);
    }

    if let JsAnyExpression::JsConditionalExpression(conditional) = &right {
        if JsAnyBinaryLikeExpression::cast(conditional.test()?.syntax().clone())
            .map_or(false, |expression| !expression.should_inline())
        {
            return Ok(true);
        }
    }

    Ok(false)
}
/// If checks if among leading trivias, we there's a sequence of [Newline, Comment]
pub(crate) fn has_new_line_before_comment(node: &JsSyntaxNode) -> bool {
    if let Some(leading_trivia) = node.first_leading_trivia() {
        let mut seen_newline = false;
        for piece in leading_trivia.pieces() {
            if piece.is_comments() && seen_newline {
                return true;
            }
            if piece.is_newline() {
                seen_newline = true
            }
        }
    }
    false
}

impl Format<JsFormatContext> for JsAnyAssignmentLike {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let right = self.right()?;
        let format_content = format_with(|f| {
            // Compare name only if we are in a position of computing it.
            // If not (for example, left is not an identifier), then let's fallback to false,
            // so we can continue the chain of checks
            let is_left_short = self.write_left(f)?;
            self.write_operator(f)?;

            let layout = self.layout(is_left_short)?;

            let inner_content = format_with(|f| match &layout {
                AssignmentLikeLayout::Fluid => {
                    let group_id = f.group_id("assignment_like");

                    let right = right.format().memoized();

                    write![
                        f,
                        [
                            group_elements(&indent(&soft_line_break_or_space()),)
                                .with_group_id(Some(group_id)),
                            line_suffix_boundary(),
                            if_group_breaks(&indent(&right)).with_group_id(Some(group_id)),
                            if_group_fits_on_line(&right).with_group_id(Some(group_id)),
                        ]
                    ]
                }
                AssignmentLikeLayout::BreakAfterOperator => {
                    write![
                        f,
                        [group_elements(&indent(&format_args![
                            soft_line_break_or_space(),
                            right.format()
                        ])),]
                    ]
                }
                AssignmentLikeLayout::NeverBreakAfterOperator => {
                    write![f, [space_token(), right.format(),]]
                }

                AssignmentLikeLayout::Chain => {
                    write!(f, [soft_line_break_or_space(), right.format()])
                }

                AssignmentLikeLayout::ChainTail => {
                    write!(
                        f,
                        [&indent(&format_args![
                            soft_line_break_or_space(),
                            right.format()
                        ])]
                    )
                }
            });

            match layout {
                // Layouts that don't need enclosing group
                AssignmentLikeLayout::Chain | AssignmentLikeLayout::ChainTail => {
                    write!(f, [&inner_content])
                }
                _ => {
                    write!(f, [group_elements(&inner_content)])
                }
            }
        });

        write!(f, [format_content])
    }
}
