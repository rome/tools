use crate::prelude::*;
use crate::utils::object::write_member_name;
use crate::utils::JsAnyBinaryLikeExpression;

use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsAnyAssignmentPattern, JsAnyBindingPattern, JsAnyExpression, JsAnyFunctionBody,
    JsAnyObjectAssignmentPatternMember, JsAnyObjectBindingPatternMember, JsAnyObjectMemberName,
    JsAssignmentExpression, JsInitializerClause, JsObjectAssignmentPattern,
    JsObjectAssignmentPatternProperty, JsObjectBindingPattern, JsPropertyObjectMember,
    JsSyntaxKind, JsVariableDeclarator,
};
use rome_js_syntax::{JsAnyLiteralExpression, JsSyntaxNode};
use rome_rowan::{declare_node_union, AstNode, SyntaxResult};

declare_node_union! {
    pub(crate) JsAnyAssignmentLike =
        JsPropertyObjectMember |
        JsAssignmentExpression |
        JsObjectAssignmentPatternProperty |
        JsVariableDeclarator
}

declare_node_union! {
    pub(crate) LeftAssignmentLike = JsAnyAssignmentPattern | JsAnyObjectMemberName | JsAnyBindingPattern
}

declare_node_union! {
    pub(crate) RightAssignmentLike = JsAnyExpression | JsAnyAssignmentPattern | JsInitializerClause
}

declare_node_union! {
    /// This is a convenient enum to map object patterns.
    pub(crate) AnyObjectPattern = JsObjectAssignmentPattern | JsObjectBindingPattern
}

impl AnyObjectPattern {
    fn is_complex(&self) -> SyntaxResult<bool> {
        return match self {
            AnyObjectPattern::JsObjectAssignmentPattern(assignment_pattern) => {
                let properties_len = assignment_pattern.properties().len();
                // A binding is complex when we have at least one [JsObjectBindingPatternProperty]
                // e.g. a = { a: c = f } = a
                // The `c = f` will trigger the complex binding
                let has_at_least_a_complex_binding = assignment_pattern
                    .properties()
                    .iter()
                    .map(|p| p.ok())
                    .any(|property| {
                        let property = property;

                        matches!(
                            property,
                            Some(
                                JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(_),
                            )
                        )
                    });
                Ok(properties_len > 2 && has_at_least_a_complex_binding)
            }
            AnyObjectPattern::JsObjectBindingPattern(binding_pattern) => {
                let properties_len = binding_pattern.properties().len();
                // A binding is complex when we have at least one [JsObjectBindingPatternProperty]
                // e.g. const a = { a: c = f } = a
                // The `c = f` will trigger the complex binding
                let has_at_least_a_complex_binding = binding_pattern
                    .properties()
                    .iter()
                    .map(|p| p.ok())
                    .any(|property| {
                        let property = property;

                        matches!(
                            property,
                            Some(
                                JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(_),
                            )
                        )
                    });
                Ok(properties_len > 2 && has_at_least_a_complex_binding)
            }
        };
    }
}

impl LeftAssignmentLike {
    fn as_object_pattern(&self) -> Option<AnyObjectPattern> {
        match self {
            LeftAssignmentLike::JsAnyAssignmentPattern(pattern) => match pattern {
                JsAnyAssignmentPattern::JsObjectAssignmentPattern(node) => {
                    Some(AnyObjectPattern::from(node.clone()))
                }
                _ => None,
            },
            LeftAssignmentLike::JsAnyBindingPattern(pattern) => match pattern {
                JsAnyBindingPattern::JsObjectBindingPattern(node) => {
                    Some(AnyObjectPattern::from(node.clone()))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl RightAssignmentLike {
    fn as_expression(&self) -> Option<JsAnyExpression> {
        match self {
            RightAssignmentLike::JsAnyExpression(expression) => Some(expression.clone()),
            RightAssignmentLike::JsInitializerClause(initializer) => initializer.expression().ok(),
            RightAssignmentLike::JsAnyAssignmentPattern(_) => None,
        }
    }
}

impl Format<JsFormatContext> for RightAssignmentLike {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            RightAssignmentLike::JsAnyExpression(expression) => {
                write!(f, [expression.format()])
            }
            RightAssignmentLike::JsAnyAssignmentPattern(assignment) => {
                write!(f, [assignment.format()])
            }
            RightAssignmentLike::JsInitializerClause(initializer) => {
                write!(f, [space_token(), initializer.format()])
            }
        }
    }
}

/// Determines how a assignment like be formatted
///
/// Assignment like are:
/// - Assignment
/// - Object property member
/// - Variable declaration
#[derive(Debug)]
pub(crate) enum AssignmentLikeLayout {
    /// This is a special layout usually used for long variable declarations
    /// This layout is hit, usually, when variable declarator doesn't have initializer:
    OnlyLeft,

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

    ///
    BreakLeftHandSide,

    ///
    ChainTailArrowFunction,
}

impl JsAnyAssignmentLike {
    fn right(&self) -> Option<RightAssignmentLike> {
        match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => {
                Some(property.value().ok()?.into())
            }
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => {
                Some(assignment.right().ok()?.into())
            }
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(assignment_pattern) => {
                Some(assignment_pattern.pattern().ok()?.into())
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                Some(variable_declarator.initializer()?.into())
            }
        }
    }

    fn left(&self) -> SyntaxResult<LeftAssignmentLike> {
        match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => Ok(property.name()?.into()),
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => {
                Ok(assignment.left()?.into())
            }
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(property) => {
                Ok(property.pattern()?.into())
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                Ok(variable_declarator.id()?.into())
            }
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
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(property) => {
                let width = write_member_name(&property.member()?, f)?;
                let text_width_for_break =
                    (f.context().tab_width() + MIN_OVERLAP_FOR_BREAK) as usize;
                Ok(width < text_width_for_break)
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                let id = variable_declarator.id()?;
                let variable_annotation = variable_declarator.variable_annotation();
                write!(
                    f,
                    [group_elements(&format_args![
                        &id.format(),
                        variable_annotation.format()
                    ])]
                )?;
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
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(property) => {
                let colon_token = property.colon_token()?;
                write!(f, [colon_token.format()])
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                if let Some(initializer) = variable_declarator.initializer() {
                    let eq_token = initializer.eq_token()?;
                    write!(f, [space_token(), eq_token.format()])?
                }
                Ok(())
            }
        }
    }

    fn write_right<'t>(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => {
                let value = property.value()?;
                write!(f, [value.format()])
            }
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => {
                let right = assignment.right()?;
                write!(f, [space_token(), right.format()])
            }
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(property) => {
                let pattern = property.pattern()?;
                let init = property.init();
                write!(f, [pattern.format()])?;
                if let Some(init) = init {
                    write!(f, [space_token(), init.format()])?;
                }
                Ok(())
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                if let Some(initializer) = variable_declarator.initializer() {
                    let expression = initializer.expression()?;
                    write!(f, [space_token(), expression.format()])?;
                }
                Ok(())
            }
        }
    }

    /// Returns the layout variant for an assignment like depending on right expression and left part length
    /// [Prettier applies]: https://github.com/prettier/prettier/blob/main/src/language-js/print/assignment.js
    fn layout(&self, is_left_short: bool) -> FormatResult<AssignmentLikeLayout> {
        let right = match self.right() {
            Some(right) => right,
            None => return Ok(AssignmentLikeLayout::OnlyLeft),
        };

        let right = right.as_expression();

        if let Some(layout) = self.is_chain_formatting()? {
            return Ok(layout);
        }

        if self.should_break_left_hand_side()? {
            return Ok(AssignmentLikeLayout::BreakLeftHandSide);
        }

        if let Some(expression) = &right {
            if should_break_after_operator(expression)? {
                return Ok(AssignmentLikeLayout::BreakAfterOperator);
            }
        }

        if is_left_short {
            return Ok(AssignmentLikeLayout::NeverBreakAfterOperator);
        }

        if matches!(
            right,
            Some(JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsStringLiteralExpression(_)
            )),
        ) {
            return Ok(AssignmentLikeLayout::BreakAfterOperator);
        }

        if self.should_never_break_after_operator()? {
            return Ok(AssignmentLikeLayout::NeverBreakAfterOperator);
        }
        Ok(AssignmentLikeLayout::Fluid)
    }

    /// Checks if the right node is entitled of the chain formatting,
    /// and if so, it return the layout type
    fn is_chain_formatting(&self) -> SyntaxResult<Option<AssignmentLikeLayout>> {
        let right = match self.right() {
            Some(right) => right,
            None => return Ok(None),
        };

        let right_is_tail = !matches!(
            right,
            RightAssignmentLike::JsAnyExpression(JsAnyExpression::JsAssignmentExpression(_))
        );
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
                match right {
                    RightAssignmentLike::JsAnyExpression(
                        JsAnyExpression::JsArrowFunctionExpression(arrow),
                    ) => {
                        let body = arrow.body()?;
                        if matches!(
                            body,
                            JsAnyFunctionBody::JsAnyExpression(
                                JsAnyExpression::JsArrowFunctionExpression(_)
                            )
                        ) {
                            Some(AssignmentLikeLayout::ChainTailArrowFunction)
                        } else {
                            Some(AssignmentLikeLayout::Chain)
                        }
                    }

                    _ => Some(AssignmentLikeLayout::Chain),
                }
            }
        } else {
            None
        };

        Ok(result)
    }

    fn should_never_break_after_operator(&self) -> SyntaxResult<bool> {
        let right = match self.right() {
            Some(right) => right,
            None => return Ok(false),
        };

        let right = right.as_expression();

        if let Some(JsAnyExpression::JsCallExpression(call_expression)) = &right {
            if call_expression.callee()?.syntax().text() == "require" {
                return Ok(true);
            }
        }

        if matches!(
            right,
            Some(
                JsAnyExpression::JsClassExpression(_)
                    | JsAnyExpression::JsTemplate(_)
                    | JsAnyExpression::JsAnyLiteralExpression(
                        JsAnyLiteralExpression::JsBooleanLiteralExpression(_),
                    )
                    | JsAnyExpression::JsAnyLiteralExpression(
                        JsAnyLiteralExpression::JsNumberLiteralExpression(_)
                    )
            )
        ) {
            return Ok(true);
        }

        Ok(false)
    }

    /// Particular function that checks if the left hand side of a [JsAnyAssignmentLike] should
    /// be broken on multiple lines
    fn should_break_left_hand_side(&self) -> SyntaxResult<bool> {
        let is_complex_destructuring = self
            .left()?
            .as_object_pattern()
            .and_then(|pattern| pattern.is_complex().ok())
            .unwrap_or(false);

        Ok(is_complex_destructuring)
    }
}

/// Checks if the function is entitled to be printed with layout [AssignmentLikeLayout::BreakAfterOperator]
pub(crate) fn should_break_after_operator(right: &JsAnyExpression) -> SyntaxResult<bool> {
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
        let format_content = format_with(|f| {
            // Compare name only if we are in a position of computing it.
            // If not (for example, left is not an identifier), then let's fallback to false,
            // so we can continue the chain of checks
            let is_left_short = self.write_left(f)?;
            self.write_operator(f)?;

            let layout = self.layout(is_left_short)?;
            dbg!(&layout);
            let inner_content = format_with(|f| match &layout {
                AssignmentLikeLayout::OnlyLeft => Ok(()),
                AssignmentLikeLayout::Fluid => {
                    let group_id = f.group_id("assignment_like");

                    let right = format_with(|f| self.write_right(f)).memoized();

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
                            format_with(|f| { self.write_right(f) }),
                        ])),]
                    ]
                }
                AssignmentLikeLayout::NeverBreakAfterOperator => {
                    write![
                        f,
                        [space_token(), format_with(|f| { self.write_right(f) }),]
                    ]
                }

                AssignmentLikeLayout::BreakLeftHandSide => {
                    write![
                        f,
                        [
                            space_token(),
                            group_elements(&format_with(|f| { self.write_right(f) }),),
                        ]
                    ]
                }

                AssignmentLikeLayout::Chain => {
                    write!(
                        f,
                        [
                            soft_line_break_or_space(),
                            format_with(|f| { self.write_right(f) }),
                        ]
                    )
                }

                AssignmentLikeLayout::ChainTail => {
                    write!(
                        f,
                        [&indent(&format_args![
                            soft_line_break_or_space(),
                            format_with(|f| { self.write_right(f) }),
                        ])]
                    )
                }

                AssignmentLikeLayout::ChainTailArrowFunction => {
                    write!(f, [space_token(), format_with(|f| { self.write_right(f) })])
                }
            });

            match layout {
                // Layouts that don't need enclosing group
                AssignmentLikeLayout::Chain
                | AssignmentLikeLayout::ChainTail
                | AssignmentLikeLayout::OnlyLeft => {
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
