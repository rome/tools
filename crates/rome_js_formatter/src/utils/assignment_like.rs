use crate::js::auxiliary::initializer_clause::FormatJsInitializerClauseOptions;
use crate::js::expressions::arrow_function_expression::FormatJsArrowFunctionExpressionOptions;
use crate::prelude::*;
use crate::utils::member_chain::is_member_call_chain;
use crate::utils::object::write_member_name;
use crate::utils::JsAnyBinaryLikeExpression;
use rome_formatter::{format_args, write, CstFormatContext, FormatOptions, VecBuffer};
use rome_js_syntax::JsAnyLiteralExpression;
use rome_js_syntax::{
    JsAnyAssignmentPattern, JsAnyBindingPattern, JsAnyCallArgument, JsAnyClassMemberName,
    JsAnyExpression, JsAnyFunctionBody, JsAnyObjectAssignmentPatternMember,
    JsAnyObjectBindingPatternMember, JsAnyObjectMemberName, JsAnyTemplateElement,
    JsAssignmentExpression, JsInitializerClause, JsLiteralMemberName, JsObjectAssignmentPattern,
    JsObjectAssignmentPatternProperty, JsObjectBindingPattern, JsPropertyClassMember,
    JsPropertyClassMemberFields, JsPropertyObjectMember, JsSyntaxKind, JsVariableDeclarator,
    TsAnyVariableAnnotation, TsIdentifierBinding, TsPropertySignatureClassMember,
    TsPropertySignatureClassMemberFields, TsType, TsTypeAliasDeclaration, TsTypeArguments,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxResult};
use std::iter;

declare_node_union! {
    pub(crate) JsAnyAssignmentLike =
        JsPropertyObjectMember |
        JsAssignmentExpression |
        JsObjectAssignmentPatternProperty |
        JsVariableDeclarator |
        TsTypeAliasDeclaration |
        JsPropertyClassMember |
        TsPropertySignatureClassMember
}

declare_node_union! {
    pub(crate) LeftAssignmentLike =
        JsAnyAssignmentPattern |
        JsAnyObjectMemberName |
        JsAnyBindingPattern |
        TsIdentifierBinding |
        JsLiteralMemberName |
        JsAnyClassMemberName
}

declare_node_union! {
    pub(crate) RightAssignmentLike = JsAnyExpression | JsAnyAssignmentPattern | JsInitializerClause | TsType
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
            RightAssignmentLike::TsType(_) => None,
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
                write!(f, [space(), initializer.format()])
            }
            RightAssignmentLike::TsType(ty) => {
                write!(f, [space(), ty.format()])
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
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AssignmentLikeLayout {
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

    /// Layout used when the operator and right hand side are part of a `JsInitializerClause<
    /// that has a suppression comment.
    SuppressedInitializer,
}

const MIN_OVERLAP_FOR_BREAK: u8 = 3;

impl JsAnyAssignmentLike {
    fn right(&self) -> SyntaxResult<RightAssignmentLike> {
        let right = match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => property.value()?.into(),
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => assignment.right()?.into(),
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(assignment_pattern) => {
                assignment_pattern.pattern()?.into()
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                // SAFETY: Calling `unwrap` here is safe because we check `has_only_left_hand_side` variant at the beginning of the `layout` function
                variable_declarator.initializer().unwrap().into()
            }
            JsAnyAssignmentLike::TsTypeAliasDeclaration(type_alias_declaration) => {
                type_alias_declaration.ty()?.into()
            }
            JsAnyAssignmentLike::JsPropertyClassMember(n) => {
                // SAFETY: Calling `unwrap` here is safe because we check `has_only_left_hand_side` variant at the beginning of the `layout` function
                n.value().unwrap().into()
            }
            JsAnyAssignmentLike::TsPropertySignatureClassMember(_) => {
                unreachable!("TsPropertySignatureClassMember doesn't have any right side. If you're here, `has_only_left_hand_side` hasn't been called")
            }
        };

        Ok(right)
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
            JsAnyAssignmentLike::TsTypeAliasDeclaration(type_alias_declaration) => {
                Ok(type_alias_declaration.binding_identifier()?.into())
            }
            JsAnyAssignmentLike::JsPropertyClassMember(property_class_member) => {
                Ok(property_class_member.name()?.into())
            }
            JsAnyAssignmentLike::TsPropertySignatureClassMember(
                property_signature_class_member,
            ) => Ok(property_signature_class_member.name()?.into()),
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

    fn write_left(&self, f: &mut JsFormatter) -> FormatResult<bool> {
        match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => {
                let name = property.name()?;

                // It's safe to mark the name as checked here because it is at the beginning of the property
                // and any suppression comment that would apply to the name applies to the property too and is,
                // thus, handled on the property level.
                f.context()
                    .comments()
                    .mark_suppression_checked(name.syntax());

                let width = write_member_name(&name.into(), f)?;
                let text_width_for_break =
                    (u8::from(f.options().tab_width()) + MIN_OVERLAP_FOR_BREAK) as usize;
                Ok(width < text_width_for_break)
            }
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => {
                let left = assignment.left()?;
                write!(f, [&left.format()])?;
                Ok(false)
            }
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(property) => {
                let member_name = property.member()?;

                // It's safe to mark the name as checked here because it is at the beginning of the property
                // and any suppression comment that would apply to the name applies to the property too and is,
                // thus, handled on the property level.
                f.context()
                    .comments()
                    .mark_suppression_checked(member_name.syntax());

                let width = write_member_name(&member_name.into(), f)?;
                let text_width_for_break =
                    (u8::from(f.options().tab_width()) + MIN_OVERLAP_FOR_BREAK) as usize;
                Ok(width < text_width_for_break)
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                let id = variable_declarator.id()?;
                let variable_annotation = variable_declarator.variable_annotation();

                write!(f, [id.format(), variable_annotation.format()])?;
                Ok(false)
            }
            JsAnyAssignmentLike::TsTypeAliasDeclaration(type_alias_declaration) => {
                let binding_identifier = type_alias_declaration.binding_identifier()?;
                let type_parameters = type_alias_declaration.type_parameters();

                write!(f, [binding_identifier.format()])?;
                if let Some(type_parameters) = type_parameters {
                    write!(f, [type_parameters.format(),])?;
                }
                Ok(false)
            }
            JsAnyAssignmentLike::JsPropertyClassMember(property_class_member) => {
                let JsPropertyClassMemberFields {
                    modifiers,
                    name,
                    property_annotation,
                    value: _,
                    semicolon_token: _,
                } = property_class_member.as_fields();
                write!(f, [modifiers.format(), space()])?;

                let name = name?;

                let is_short = if f.context().comments().is_suppressed(name.syntax()) {
                    write!(f, [format_suppressed_node(name.syntax())])?;
                    false
                } else {
                    let width = write_member_name(&name.into(), f)?;
                    let text_width_for_break =
                        (u8::from(f.options().tab_width()) + MIN_OVERLAP_FOR_BREAK) as usize;
                    width < text_width_for_break && property_annotation.is_none()
                };

                write!(f, [property_annotation.format()])?;

                Ok(is_short)
            }
            JsAnyAssignmentLike::TsPropertySignatureClassMember(
                property_signature_class_member,
            ) => {
                let TsPropertySignatureClassMemberFields {
                    modifiers,
                    name,
                    property_annotation,
                    semicolon_token: _,
                } = property_signature_class_member.as_fields();

                write!(f, [modifiers.format(), space(),])?;

                let width = write_member_name(&name?.into(), f)?;

                write!(f, [property_annotation.format()])?;
                let text_width_for_break =
                    (u8::from(f.options().tab_width()) + MIN_OVERLAP_FOR_BREAK) as usize;
                Ok(width < text_width_for_break)
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
                write!(f, [space(), operator_token.format()])
            }
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(property) => {
                let colon_token = property.colon_token()?;
                write!(f, [colon_token.format()])
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                if let Some(initializer) = variable_declarator.initializer() {
                    let eq_token = initializer.eq_token()?;
                    write!(f, [space(), eq_token.format()])?
                }
                Ok(())
            }
            JsAnyAssignmentLike::TsTypeAliasDeclaration(type_alias_declaration) => {
                let eq_token = type_alias_declaration.eq_token()?;
                write!(f, [space(), eq_token.format()])
            }
            JsAnyAssignmentLike::JsPropertyClassMember(property_class_member) => {
                if let Some(initializer) = property_class_member.value() {
                    let eq_token = initializer.eq_token()?;
                    write!(f, [space(), eq_token.format()])?
                }
                Ok(())
            }
            // this variant doesn't have any operator
            JsAnyAssignmentLike::TsPropertySignatureClassMember(_) => Ok(()),
        }
    }

    fn write_right(&self, f: &mut JsFormatter, layout: AssignmentLikeLayout) -> FormatResult<()> {
        match self {
            JsAnyAssignmentLike::JsPropertyObjectMember(property) => {
                let value = property.value()?;
                write!(f, [with_assignment_layout(&value, Some(layout))])
            }
            JsAnyAssignmentLike::JsAssignmentExpression(assignment) => {
                let right = assignment.right()?;
                write!(f, [space(), with_assignment_layout(&right, Some(layout))])
            }
            JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(property) => {
                let pattern = property.pattern()?;
                let init = property.init();
                write!(f, [pattern.format()])?;
                if let Some(init) = init {
                    write!(
                        f,
                        [
                            space(),
                            init.format()
                                .with_options(FormatJsInitializerClauseOptions {
                                    assignment_layout: Some(layout)
                                })
                        ]
                    )?;
                }
                Ok(())
            }
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                if let Some(initializer) = variable_declarator.initializer() {
                    let expression = initializer.expression()?;
                    write!(
                        f,
                        [
                            space(),
                            format_leading_comments(initializer.syntax()),
                            with_assignment_layout(&expression, Some(layout)),
                            format_trailing_comments(initializer.syntax())
                        ]
                    )?;
                }
                Ok(())
            }
            JsAnyAssignmentLike::TsTypeAliasDeclaration(type_alias_declaration) => {
                let ty = type_alias_declaration.ty()?;
                write!(f, [space(), ty.format()])
            }
            JsAnyAssignmentLike::JsPropertyClassMember(property_class_member) => {
                if let Some(initializer) = property_class_member.value() {
                    let expression = initializer.expression()?;
                    write!(
                        f,
                        [
                            space(),
                            format_leading_comments(initializer.syntax()),
                            with_assignment_layout(&expression, Some(layout)),
                            format_trailing_comments(initializer.syntax())
                        ]
                    )?;
                }
                Ok(())
            }
            // this variant doesn't have any right part
            JsAnyAssignmentLike::TsPropertySignatureClassMember(_) => Ok(()),
        }
    }

    fn write_suppressed_initializer(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let initializer = match self {
            JsAnyAssignmentLike::JsPropertyClassMember(class_member) => class_member.value(),
            JsAnyAssignmentLike::JsVariableDeclarator(variable_declarator) => {
                variable_declarator.initializer()
            }

            JsAnyAssignmentLike::JsPropertyObjectMember(_)
            | JsAnyAssignmentLike::JsAssignmentExpression(_)
            | JsAnyAssignmentLike::JsObjectAssignmentPatternProperty(_)
            | JsAnyAssignmentLike::TsTypeAliasDeclaration(_)
            | JsAnyAssignmentLike::TsPropertySignatureClassMember(_) => {
                unreachable!("These variants have no initializer")
            }
        };

        let initializer =
            initializer.expect("Expected an initializer because it has a suppression comment");

        write!(f, [soft_line_indent_or_space(&initializer.format())])
    }

    /// Returns the layout variant for an assignment like depending on right expression and left part length
    /// [Prettier applies]: https://github.com/prettier/prettier/blob/main/src/language-js/print/assignment.js
    fn layout(
        &self,
        is_left_short: bool,
        f: &mut Formatter<JsFormatContext>,
    ) -> FormatResult<AssignmentLikeLayout> {
        if self.has_only_left_hand_side() {
            return Ok(AssignmentLikeLayout::OnlyLeft);
        }

        let right = self.right()?;

        if let RightAssignmentLike::JsInitializerClause(initializer) = &right {
            if f.context().comments().is_suppressed(initializer.syntax()) {
                return Ok(AssignmentLikeLayout::SuppressedInitializer);
            }
        }
        let right_expression = right.as_expression();

        if let Some(layout) = self.chain_formatting_layout(right_expression.as_ref())? {
            return Ok(layout);
        }

        if let Some(JsAnyExpression::JsCallExpression(call_expression)) = &right_expression {
            if call_expression.callee()?.syntax().text() == "require" {
                return Ok(AssignmentLikeLayout::NeverBreakAfterOperator);
            }
        }

        if self.should_break_left_hand_side()? {
            return Ok(AssignmentLikeLayout::BreakLeftHandSide);
        }

        if self.should_break_after_operator(&right, f.context().comments())? {
            return Ok(AssignmentLikeLayout::BreakAfterOperator);
        }

        if is_left_short {
            return Ok(AssignmentLikeLayout::NeverBreakAfterOperator);
        }

        // Before checking `BreakAfterOperator` layout, we need to unwrap the right expression from `JsUnaryExpression` or `TsNonNullAssertionExpression`
        // [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/assignment.js#L199-L211
        // Example:
        //  !"123" -> "123"
        //  void "123" -> "123"
        //  !!"string"! -> "string"
        let right_expression = iter::successors(right_expression, |expression| match expression {
            JsAnyExpression::JsUnaryExpression(unary) => unary.argument().ok(),
            JsAnyExpression::TsNonNullAssertionExpression(assertion) => assertion.expression().ok(),
            _ => None,
        })
        .last();

        if matches!(
            right_expression,
            Some(JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsStringLiteralExpression(_)
            )),
        ) {
            return Ok(AssignmentLikeLayout::BreakAfterOperator);
        }

        let is_poorly_breakable = match &right_expression {
            Some(expression) => is_poorly_breakable_member_or_call_chain(expression, f)?,
            None => false,
        };

        if is_poorly_breakable {
            return Ok(AssignmentLikeLayout::BreakAfterOperator);
        }

        if matches!(
            right_expression,
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
            return Ok(AssignmentLikeLayout::NeverBreakAfterOperator);
        }

        Ok(AssignmentLikeLayout::Fluid)
    }

    /// Checks that a [JsAnyAssignmentLike] consists only of the left part
    /// usually, when a [variable declarator](JsVariableDeclarator) doesn't have initializer
    fn has_only_left_hand_side(&self) -> bool {
        if let JsAnyAssignmentLike::JsVariableDeclarator(declarator) = self {
            declarator.initializer().is_none()
        } else if let JsAnyAssignmentLike::JsPropertyClassMember(class_member) = self {
            class_member.value().is_none()
        } else {
            matches!(self, JsAnyAssignmentLike::TsPropertySignatureClassMember(_))
        }
    }

    /// Checks if the right node is entitled of the chain formatting,
    /// and if so, it return the layout type
    fn chain_formatting_layout(
        &self,
        right_expression: Option<&JsAnyExpression>,
    ) -> SyntaxResult<Option<AssignmentLikeLayout>> {
        let right_is_tail = !matches!(
            right_expression,
            Some(JsAnyExpression::JsAssignmentExpression(_))
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
                            | JsSyntaxKind::JS_INITIALIZER_CLAUSE
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
                match right_expression {
                    Some(JsAnyExpression::JsArrowFunctionExpression(arrow)) => {
                        let this_body = arrow.body()?;
                        match this_body {
                            JsAnyFunctionBody::JsAnyExpression(expression) => {
                                if matches!(
                                    expression,
                                    JsAnyExpression::JsArrowFunctionExpression(_)
                                ) {
                                    Some(AssignmentLikeLayout::ChainTailArrowFunction)
                                } else {
                                    Some(AssignmentLikeLayout::ChainTail)
                                }
                            }
                            _ => Some(AssignmentLikeLayout::ChainTail),
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

    fn is_complex_type_alias(&self) -> SyntaxResult<bool> {
        let result = if let JsAnyAssignmentLike::TsTypeAliasDeclaration(type_alias_declaration) =
            self
        {
            let type_parameters = type_alias_declaration.type_parameters();

            if let Some(type_parameters) = type_parameters {
                let items = type_parameters.items();
                if items.len() <= 1 {
                    return Ok(false);
                };
                for type_parameter in type_parameters.items() {
                    let type_parameter = type_parameter?;

                    if type_parameter.constraint().is_some() || type_parameter.default().is_some() {
                        return Ok(true);
                    }
                }
                return Ok(false);
            } else {
                false
            }
        } else {
            false
        };

        Ok(result)
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

        let is_complex_type_alias = self.is_complex_type_alias()?;

        Ok(is_complex_destructuring || has_complex_type_annotation || is_complex_type_alias)
    }

    /// Checks if the the current assignment is eligible for [AssignmentLikeLayout::BreakAfterOperator]
    ///
    /// This function is small wrapper around [should_break_after_operator] because it has to work
    /// for nodes that belong to TypeScript too.
    fn should_break_after_operator(
        &self,
        right: &RightAssignmentLike,
        comments: &JsComments,
    ) -> SyntaxResult<bool> {
        let result = match right {
            RightAssignmentLike::JsAnyExpression(expression) => {
                should_break_after_operator(expression, comments)?
            }
            RightAssignmentLike::JsInitializerClause(initializer) => {
                comments.has_leading_own_line_comment(initializer.syntax())
                    || should_break_after_operator(&initializer.expression()?, comments)?
            }
            RightAssignmentLike::TsType(TsType::TsUnionType(ty)) => {
                comments.has_leading_comments(ty.syntax())
            }
            right => comments.has_leading_own_line_comment(right.syntax()),
        };

        Ok(result)
    }
}

/// Checks if the function is entitled to be printed with layout [AssignmentLikeLayout::BreakAfterOperator]
pub(crate) fn should_break_after_operator(
    right: &JsAnyExpression,
    comments: &JsComments,
) -> SyntaxResult<bool> {
    if comments.has_leading_own_line_comment(right.syntax())
        && !matches!(right, JsAnyExpression::JsxTagExpression(_))
    {
        return Ok(true);
    }

    let result = match right {
        // head is a long chain, meaning that right -> right are both assignment expressions
        JsAnyExpression::JsAssignmentExpression(assignment) => {
            matches!(
                assignment.right()?,
                JsAnyExpression::JsAssignmentExpression(_)
            )
        }
        right if JsAnyBinaryLikeExpression::can_cast(right.syntax().kind()) => {
            let binary_like = JsAnyBinaryLikeExpression::unwrap_cast(right.syntax().clone());

            !binary_like.should_inline_logical_expression()
        }

        JsAnyExpression::JsSequenceExpression(_) => true,

        JsAnyExpression::JsConditionalExpression(conditional) => {
            JsAnyBinaryLikeExpression::cast(conditional.test()?.into_syntax())
                .map_or(false, |expression| {
                    !expression.should_inline_logical_expression()
                })
        }

        _ => false,
    };

    Ok(result)
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
            let is_left_short = self.write_left(&mut Formatter::new(&mut buffer))?;
            let formatted_left = buffer.into_vec();

            // Compare name only if we are in a position of computing it.
            // If not (for example, left is not an identifier), then let's fallback to false,
            // so we can continue the chain of checks
            let layout = self.layout(is_left_short, f)?;

            let left = format_once(|f| f.write_elements(formatted_left));
            let right = format_with(|f| self.write_right(f, layout));

            let inner_content = format_with(|f| {
                if matches!(
                    &layout,
                    AssignmentLikeLayout::BreakLeftHandSide | AssignmentLikeLayout::OnlyLeft
                ) {
                    write!(f, [left])?;
                } else {
                    write!(f, [group(&left)])?;
                }

                if layout != AssignmentLikeLayout::SuppressedInitializer {
                    self.write_operator(f)?;
                }

                match layout {
                    AssignmentLikeLayout::OnlyLeft => Ok(()),
                    AssignmentLikeLayout::Fluid => {
                        let group_id = f.group_id("assignment_like");

                        write![
                            f,
                            [
                                group(&indent(&soft_line_break_or_space()),)
                                    .with_group_id(Some(group_id)),
                                line_suffix_boundary(),
                                indent_if_group_breaks(&right, group_id)
                            ]
                        ]
                    }
                    AssignmentLikeLayout::BreakAfterOperator => {
                        write![
                            f,
                            [group(&indent(&format_args![
                                soft_line_break_or_space(),
                                right,
                            ]))]
                        ]
                    }
                    AssignmentLikeLayout::NeverBreakAfterOperator => {
                        write![f, [space(), right]]
                    }

                    AssignmentLikeLayout::BreakLeftHandSide => {
                        write![f, [space(), group(&right)]]
                    }

                    AssignmentLikeLayout::Chain => {
                        write!(f, [soft_line_break_or_space(), right])
                    }

                    AssignmentLikeLayout::ChainTail => {
                        write!(
                            f,
                            [&indent(&format_args![soft_line_break_or_space(), right])]
                        )
                    }

                    AssignmentLikeLayout::ChainTailArrowFunction => {
                        write!(f, [space(), right])
                    }
                    AssignmentLikeLayout::SuppressedInitializer => {
                        self.write_suppressed_initializer(f)
                    }
                }
            });

            match layout {
                // Layouts that don't need enclosing group
                AssignmentLikeLayout::Chain
                | AssignmentLikeLayout::ChainTail
                | AssignmentLikeLayout::SuppressedInitializer
                | AssignmentLikeLayout::OnlyLeft => {
                    write!(f, [&inner_content])
                }
                _ => {
                    write!(f, [group(&inner_content)])
                }
            }
        });

        write!(f, [format_content])
    }
}

/// A chain that has no calls at all or all of whose calls have no arguments
/// or have only one which [is_short_argument], except for member call chains
/// [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/assignment.js#L329
fn is_poorly_breakable_member_or_call_chain(
    expression: &JsAnyExpression,
    f: &Formatter<JsFormatContext>,
) -> SyntaxResult<bool> {
    let threshold = f.options().line_width().value() / 4;

    // Only call and member chains are poorly breakable
    // - `obj.member.prop`
    // - `obj.member()()`
    let mut is_chain = false;

    // Only chains with simple head are poorly breakable
    // Simple head is `JsIdentifierExpression` or `JsThisExpression`
    let mut is_chain_head_simple = false;

    // Keeping track of all call expressions in the chain to check them later
    let mut call_expressions = vec![];

    let mut expression = Some(expression.clone());

    while let Some(node) = expression.take() {
        expression = match node {
            JsAnyExpression::JsCallExpression(call_expression) => {
                is_chain = true;
                let callee = call_expression.callee()?;
                call_expressions.push(call_expression);
                Some(callee)
            }
            JsAnyExpression::JsStaticMemberExpression(node) => {
                is_chain = true;
                Some(node.object()?)
            }
            JsAnyExpression::JsComputedMemberExpression(node) => {
                is_chain = true;
                Some(node.object()?)
            }
            JsAnyExpression::JsIdentifierExpression(_) | JsAnyExpression::JsThisExpression(_) => {
                is_chain_head_simple = true;
                break;
            }
            _ => {
                break;
            }
        }
    }

    if !is_chain || !is_chain_head_simple {
        return Ok(false);
    }

    for call_expression in call_expressions {
        if is_member_call_chain(
            call_expression.clone(),
            f.comments(),
            f.options().tab_width(),
        )? {
            return Ok(false);
        }

        let args = call_expression.arguments()?.args();

        let is_breakable_call = match args.len() {
            0 => false,
            1 => match args.iter().next() {
                Some(first_argument) => {
                    !is_short_argument(first_argument?, threshold, f.context().comments())?
                }
                None => false,
            },
            _ => true,
        };

        if is_breakable_call {
            return Ok(false);
        }

        let is_breakable_type_arguments = match call_expression.type_arguments() {
            Some(type_arguments) => is_complex_type_arguments(type_arguments)?,
            None => false,
        };

        if is_breakable_type_arguments {
            return Ok(false);
        }
    }

    Ok(true)
}

/// This function checks if `JsAnyCallArgument` is short
/// We need it to decide if `JsCallExpression` with the argument is breakable or not
/// If the argument is short the function call isn't breakable
/// [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/assignment.js#L374
fn is_short_argument(
    argument: JsAnyCallArgument,
    threshold: u16,
    comments: &JsComments,
) -> SyntaxResult<bool> {
    if comments.has_comments(argument.syntax()) {
        return Ok(false);
    }

    if let JsAnyCallArgument::JsAnyExpression(expression) = argument {
        let is_short_argument = match expression {
            JsAnyExpression::JsThisExpression(_) => true,
            JsAnyExpression::JsIdentifierExpression(identifier) => {
                identifier.name()?.value_token()?.text_trimmed().len() <= threshold as usize
            }
            JsAnyExpression::JsUnaryExpression(unary_expression) => {
                let has_comments = comments.has_comments(unary_expression.argument()?.syntax());

                unary_expression.is_signed_numeric_literal()? && !has_comments
            }
            JsAnyExpression::JsAnyLiteralExpression(literal) => match literal {
                JsAnyLiteralExpression::JsRegexLiteralExpression(regex) => {
                    regex.pattern()?.chars().count() <= threshold as usize
                }
                JsAnyLiteralExpression::JsStringLiteralExpression(string) => {
                    string.value_token()?.text_trimmed().len() <= threshold as usize
                }
                _ => true,
            },
            JsAnyExpression::JsTemplate(template) => {
                let elements = template.elements();

                // Besides checking length exceed we also need to check that the template doesn't have any expressions.
                // It means that the elements of the template are empty or have only one `JsTemplateChunkElement` element
                // Prettier: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/assignment.js#L402-L405
                match elements.len() {
                    0 => true,
                    1 => match elements.iter().next() {
                        Some(JsAnyTemplateElement::JsTemplateChunkElement(element)) => {
                            let token = element.template_chunk_token()?;
                            let text_trimmed = token.text_trimmed();
                            !text_trimmed.contains('\n') && text_trimmed.len() <= threshold as usize
                        }
                        _ => false,
                    },
                    _ => false,
                }
            }
            _ => false,
        };
        Ok(is_short_argument)
    } else {
        Ok(false)
    }
}

/// This function checks if `TsTypeArguments` is complex
/// We need it to decide if `JsCallExpression` with the type arguments is breakable or not
/// If the type arguments is complex the function call is breakable
/// [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/assignment.js#L432
fn is_complex_type_arguments(type_arguments: TsTypeArguments) -> SyntaxResult<bool> {
    let ts_type_argument_list = type_arguments.ts_type_argument_list();

    if ts_type_argument_list.len() > 1 {
        return Ok(true);
    }

    let is_first_argument_complex = ts_type_argument_list
        .iter()
        .next()
        .transpose()?
        .map(|first_argument| {
            matches!(
                first_argument,
                TsType::TsUnionType(_) | TsType::TsIntersectionType(_) | TsType::TsObjectType(_)
            )
        })
        .unwrap_or(false);

    if is_first_argument_complex {
        return Ok(true);
    }

    // TODO: add here will_break logic
    // https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/assignment.js#L454

    Ok(false)
}

/// Formats an expression and passes the assignment layout to its formatting function if the expressions
/// formatting rule takes the layout as an option.
pub(crate) struct WithAssignmentLayout<'a> {
    expression: &'a JsAnyExpression,
    layout: Option<AssignmentLikeLayout>,
}

pub(crate) fn with_assignment_layout(
    expression: &JsAnyExpression,
    layout: Option<AssignmentLikeLayout>,
) -> WithAssignmentLayout {
    WithAssignmentLayout { expression, layout }
}

impl Format<JsFormatContext> for WithAssignmentLayout<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self.expression {
            JsAnyExpression::JsArrowFunctionExpression(arrow) => arrow
                .format()
                .with_options(FormatJsArrowFunctionExpressionOptions {
                    assignment_layout: self.layout,
                    ..FormatJsArrowFunctionExpressionOptions::default()
                })
                .fmt(f),
            expression => expression.format().fmt(f),
        }
    }
}
