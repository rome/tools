use crate::prelude::*;
use crate::utils::object::write_member_name;
use crate::utils::JsAnyBinaryLikeExpression;
use rome_formatter::{format_args, write, VecBuffer};
use rome_js_syntax::{
    JsAnyAssignmentPattern, JsAnyBindingPattern, JsAnyExpression, JsAnyFunctionBody,
    JsAnyObjectAssignmentPatternMember, JsAnyObjectBindingPatternMember, JsAnyObjectMemberName,
    JsAssignmentExpression, JsInitializerClause, JsObjectAssignmentPattern,
    JsObjectAssignmentPatternProperty, JsObjectBindingPattern, JsPropertyObjectMember,
    JsSyntaxKind, JsVariableDeclarator, TsAnyVariableAnnotation, TsType,
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
        match self {
            AnyObjectPattern::JsObjectAssignmentPattern(assignment_pattern) => {
                let properties_len = assignment_pattern.properties().len();
                if properties_len <= 2 {
                    return Ok(false);
                }
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
                Ok(has_at_least_a_complex_binding)
            }
            AnyObjectPattern::JsObjectBindingPattern(binding_pattern) => {
                let properties_len = binding_pattern.properties().len();
                if properties_len <= 2 {
                    return Ok(false);
                }
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
                Ok(has_at_least_a_complex_binding)
            }
        }
    }
}

impl LeftAssignmentLike {
    fn as_object_pattern(&self) -> Option<AnyObjectPattern> {
        match self {
            LeftAssignmentLike::JsAnyAssignmentPattern(
                JsAnyAssignmentPattern::JsObjectAssignmentPattern(node),
            ) => Some(AnyObjectPattern::from(node.clone())),
            LeftAssignmentLike::JsAnyBindingPattern(
                JsAnyBindingPattern::JsObjectBindingPattern(node),
            ) => Some(AnyObjectPattern::from(node.clone())),
            _ => None,
        }
    }
}

/// [Prettier applies]: https://github.com/prettier/prettier/blob/fde0b49d7866e203ca748c306808a87b7c15548f/src/language-js/print/assignment.js#L278
pub(crate) fn is_complex_type_annotation(
    annotation: TsAnyVariableAnnotation,
) -> SyntaxResult<bool> {
    let is_complex = annotation
        .type_annotation()?
        .and_then(|type_annotation| type_annotation.ty().ok())
        .and_then(|ty| match ty {
            TsType::TsReferenceType(reference_type) => {
                let type_arguments = reference_type.type_arguments()?;
                let argument_list_len = type_arguments.ts_type_argument_list().len();

                if argument_list_len <= 1 {
                    return Some(false);
                }

                let has_at_least_a_complex_type = type_arguments
                    .ts_type_argument_list()
                    .iter()
                    .flat_map(|p| p.ok())
                    .any(|argument| {
                        if matches!(argument, TsType::TsConditionalType(_)) {
                            return true;
                        }

                        let is_complex_type = argument
                            .as_ts_reference_type()
                            .and_then(|reference_type| reference_type.type_arguments())
                            .map_or(false, |type_arguments| {
                                type_arguments.ts_type_argument_list().len() > 0
                            });

                        is_complex_type
                    });
                Some(has_at_least_a_complex_type)
            }
            _ => Some(false),
        })
        .unwrap_or(false);

    Ok(is_complex)
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
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum AssignmentLikeLayout {
    /// This is a special layout usually used for variable declarations.
    /// This layout is hit, usually, when a [variable declarator](JsVariableDeclarator) doesn't have initializer:
    /// ```js
    ///     let variable;
    /// ```
    /// ```ts
    ///     let variable: Map<string, number>;
    /// ```
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

    /// This layout is used in cases where we want to "break" the left hand side
    /// of assignment like expression, but only when the group decides to do it.
    ///
    /// ```js
    /// const a {
    ///     loreum: { ipsum },
    ///     something_else,
    ///     happy_days: { fonzy }
    /// } = obj;
    /// ```
    ///
    /// The snippet triggers the layout because the left hand side contains a "complex destructuring"
    /// which requires having the properties broke on different lines.
    BreakLeftHandSide,

    /// This is a special case of the "chain" layout collection. This is triggered when there's
    /// a series of simple assignments (at least three) and in the middle we have an arrow function
    /// and this function followed by two more arrow functions.
    ///
    /// This layout will break the right hand side of the tail on a new line and add a new level
    /// of indentation
    ///
    /// ```js
    /// lorem =
    ///     fff =
    ///     ee =
    ///         () => (fff) => () => (fefef) => () => fff;
    /// ```
    ChainTailArrowFunction,
}

impl JsAnyAssignmentLike {
    fn right(&self) -> SyntaxResult<RightAssignmentLike> {
        match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => Ok(property.value()?.into()),
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => {
                Ok(assignment.right()?.into())
            }
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(assignment_pattern) => {
                Ok(assignment_pattern.pattern()?.into())
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                // SAFETY: Calling `unwrap` here is safe because we check `should_only_left` variant at the beginning of the `layout` function
                Ok(variable_declarator.initializer().unwrap().into())
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

    fn annotation(&self) -> Option<TsAnyVariableAnnotation> {
        match self {
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                variable_declarator.variable_annotation()
            }
            _ => None,
        }
    }
}

const MIN_OVERLAP_FOR_BREAK: u8 = 3;

impl JsAnyAssignmentLike {
    fn write_left(&self, buffer: &mut VecBuffer<JsFormatContext>) -> FormatResult<bool> {
        match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => {
                let width = write_member_name(&property.name()?, buffer)?;
                let text_width_for_break =
                    (buffer.context().tab_width() + MIN_OVERLAP_FOR_BREAK) as usize;
                Ok(width < text_width_for_break)
            }
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => {
                let left = assignment.left()?;
                write!(buffer, [&left.format()])?;
                Ok(false)
            }
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(property) => {
                let width = write_member_name(&property.member()?, buffer)?;
                let text_width_for_break =
                    (buffer.context().tab_width() + MIN_OVERLAP_FOR_BREAK) as usize;
                Ok(width < text_width_for_break)
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                let id = variable_declarator.id()?;
                let variable_annotation = variable_declarator.variable_annotation();
                write!(buffer, [&id.format(), variable_annotation.format()])?;
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

    fn write_right(&self, f: &mut JsFormatter) -> FormatResult<()> {
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
        if self.should_only_left() {
            return Ok(AssignmentLikeLayout::OnlyLeft);
        }

        let right = self.right()?.as_expression();

        if let Some(layout) = self.chain_formatting_layout()? {
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

    /// Checks that a [JsAnyAssignmentLike] consists only of the left part
    /// usually, when a [variable declarator](JsVariableDeclarator) doesn't have initializer
    fn should_only_left(&self) -> bool {
        if let JsAnyAssignmentLike::JsVariableDeclarator(declarator) = self {
            declarator.initializer().is_none()
        } else {
            false
        }
    }

    /// Checks if the right node is entitled of the chain formatting,
    /// and if so, it return the layout type
    fn chain_formatting_layout(&self) -> SyntaxResult<Option<AssignmentLikeLayout>> {
        let right = self.right()?;

        let right_is_tail = !matches!(
            right,
            RightAssignmentLike::JsAnyExpression(JsAnyExpression::JsAssignmentExpression(_))
        );
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
            if !right_is_tail {
                Some(AssignmentLikeLayout::Chain)
            } else {
                match right {
                    RightAssignmentLike::JsAnyExpression(
                        JsAnyExpression::JsArrowFunctionExpression(arrow),
                    ) => {
                        let this_body = arrow.body()?;
                        if matches!(
                            this_body,
                            JsAnyFunctionBody::JsAnyExpression(
                                JsAnyExpression::JsArrowFunctionExpression(_)
                            )
                        ) {
                            Some(AssignmentLikeLayout::ChainTailArrowFunction)
                        } else {
                            Some(AssignmentLikeLayout::ChainTail)
                        }
                    }

                    _ => Some(AssignmentLikeLayout::ChainTail),
                }
            }
        } else {
            None
        };

        Ok(result)
    }

    fn should_never_break_after_operator(&self) -> SyntaxResult<bool> {
        let right = self.right()?.as_expression();

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

        let has_complex_type_annotation = self
            .annotation()
            .and_then(|annotation| is_complex_type_annotation(annotation).ok())
            .unwrap_or(false);

        Ok(is_complex_destructuring || has_complex_type_annotation)
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
            // We create a temporary buffer because the left hand side has to conditionally add
            // a group based on the layout, but the layout can only be computed by knowing the
            // width of the left hand side. The left hand side can be a member, and that has a width
            // can can be known only when it's formatted (it can incur in some transformation,
            // like removing some escapes, etc.).
            //
            // 1. we crate a temporary buffer
            // 2. we write the left hand side into the buffer and retrieve the `is_left_short` info
            // which is computed only when we format it
            // 3. we compute the layout
            // 4. we write the left node inside the main buffer based on the layout
            let mut buffer = VecBuffer::new(f.state_mut());
            let is_left_short = self.write_left(&mut buffer)?;

            // Compare name only if we are in a position of computing it.
            // If not (for example, left is not an identifier), then let's fallback to false,
            // so we can continue the chain of checks
            let layout = self.layout(is_left_short)?;

            let formatted_element = buffer.into_element();

            if matches!(
                layout,
                AssignmentLikeLayout::BreakAfterOperator | AssignmentLikeLayout::OnlyLeft
            ) {
                write!(
                    f,
                    [&format_once(|f| { f.write_element(formatted_element) })]
                )?;
            } else {
                write!(
                    f,
                    [group_elements(&format_once(|f| {
                        f.write_element(formatted_element)
                    }))]
                )?;
            }

            self.write_operator(f)?;

            let right = &format_with(|f| self.write_right(f)).memoized();

            let inner_content = format_with(|f| match &layout {
                AssignmentLikeLayout::OnlyLeft => Ok(()),
                AssignmentLikeLayout::Fluid => {
                    let group_id = f.group_id("assignment_like");

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
                            right,
                        ])),]
                    ]
                }
                AssignmentLikeLayout::NeverBreakAfterOperator => {
                    write![f, [space_token(), right,]]
                }

                AssignmentLikeLayout::BreakLeftHandSide => {
                    write![f, [space_token(), group_elements(right),]]
                }

                AssignmentLikeLayout::Chain => {
                    write!(f, [soft_line_break_or_space(), right,])
                }

                AssignmentLikeLayout::ChainTail => {
                    write!(
                        f,
                        [&indent(&format_args![soft_line_break_or_space(), right,])]
                    )
                }

                AssignmentLikeLayout::ChainTailArrowFunction => {
                    let group_id = f.group_id("arrow_chain");

                    write!(
                        f,
                        [
                            space_token(),
                            group_elements(&indent(&format_args![hard_line_break(), right]))
                                .with_group_id(Some(group_id)),
                        ]
                    )
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
