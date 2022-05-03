use crate::utils::is_call_like_expression;
use rome_js_syntax::{
    JsAnyArrayElement, JsAnyCallArgument, JsAnyExpression, JsAnyName, JsAnyObjectMember,
    JsAnyObjectMemberName, JsAnyTemplateElement, JsSpread, JsStaticMemberExpressionFields,
    JsTemplate, JsUnaryOperator,
};
use rome_rowan::{AstSeparatedList, SyntaxResult};

/// This enum tracks the arguments inside a call expressions and checks if they are
/// simple or not.
///
/// The heuristic changes based on its type and the depth of the expressions. For example
/// if we have expressions as arguments, having 2 or more them tags the first argument as "not simple".
///
/// Criteria are different:
/// - *complex*: if the chain of simple arguments exceeds the depth 2 or higher
/// - *simple*: the argument is a literal
/// - *simple*: the argument is a [JsThisExpression]
/// - *simple*: the argument is a [JsIdentifierExpression]
/// - *simple*: the argument is a [JsSuperExpression]
/// - *simple*: the argument is a [JsUnaryExpression], has `!` or `-` operator and the argument is simple
/// - *simple*: the argument is a [TsNonNullAssertionExpression] and the argument is simple
/// - if the argument is a template literal, check [is_simple_template_literal]
/// - if the argument is an object expression, all its members are checked if they are simple or not. Check [is_simple_static_member_expression]
/// - if the argument is an array expression, all its elements are checked if they are simple or not. Check [is_simple_array_expression]
///
/// This algorithm is inspired from [Prettier].
///
/// [JsThisExpression]: [rome_js_syntax::JsThisExpression]
/// [JsIdentifierExpression]: [rome_js_syntax::JsIdentifierExpression]
/// [JsSuperExpression]: [rome_js_syntax::JsSuperExpression]
/// [is_simple_static_member_expression]: [Simple::is_simple_static_member_expression]
/// [is_simple_array_expression]: [Simple::is_simple_array_expression]
/// [JsUnaryExpression]: [rome_js_syntax::JsUnaryExpression]
/// [TsNonNullAssertionExpression]: [rome_js_syntax::TsNonNullAssertionExpression]
/// [Prettier]: https://github.com/prettier/prettier/blob/a9de2a128cc8eea84ddd90efdc210378a894ab6b/src/language-js/utils/index.js#L802-L886
#[derive(Debug)]
pub(crate) enum SimpleArgument {
    Expression(JsAnyExpression),
    Name(JsAnyName),
    Member(JsAnyObjectMember),
    ArrayElement(JsAnyArrayElement),
    Spread,
}

impl SimpleArgument {
    pub fn new(node: JsAnyCallArgument) -> Self {
        match node {
            JsAnyCallArgument::JsAnyExpression(expr) => Self::Expression(expr),
            JsAnyCallArgument::JsSpread(_) => Self::Spread,
        }
    }

    pub fn is_simple(&self, depth: u8) -> bool {
        if depth >= 2 {
            return false;
        }
        if self.is_simple_literal() {
            return true;
        }

        self.is_simple_template(depth)
            || self.is_simple_object_expression(depth)
            || self.is_simple_array_expression(depth)
            || self.is_simple_unary_expression(depth).unwrap_or(false)
            || self
                .is_simple_non_null_assertion_expression(depth)
                .unwrap_or(false)
            || self
                .is_simple_static_member_expression(depth)
                .unwrap_or(false)
            || self.is_simple_call_like_expression(depth).unwrap_or(false)
            || self.is_simple_object_expression(depth)
    }

    fn is_simple_call_like_expression(&self, depth: u8) -> SyntaxResult<bool> {
        let result = if let SimpleArgument::Expression(any_expression) = self {
            if is_call_like_expression(any_expression) {
                let mut is_import_call_expression = false;
                let mut is_simple_callee = false;
                let arguments = match any_expression {
                    JsAnyExpression::JsNewExpression(expr) => {
                        let callee = expr.callee()?;
                        is_simple_callee = SimpleArgument::from(callee).is_simple(depth);
                        expr.arguments()
                    }
                    JsAnyExpression::JsCallExpression(expr) => {
                        let callee = expr.callee()?;
                        is_simple_callee = SimpleArgument::from(callee).is_simple(depth);
                        expr.arguments().ok()
                    }
                    JsAnyExpression::JsImportCallExpression(expr) => {
                        is_import_call_expression = true;
                        expr.arguments().ok()
                    }
                    _ => unreachable!("The check is done inside `is_call_like_expression`"),
                };

                let simple_arguments = if let Some(arguments) = arguments {
                    arguments.args().iter().all(|argument| {
                        argument.map_or(true, |argument| {
                            SimpleArgument::from(argument).is_simple(depth + 1)
                        })
                    })
                } else {
                    true
                };

                (is_import_call_expression || is_simple_callee) && simple_arguments
            } else {
                false
            }
        } else {
            false
        };

        Ok(result)
    }

    fn is_simple_static_member_expression(&self, depth: u8) -> SyntaxResult<bool> {
        if let SimpleArgument::Expression(JsAnyExpression::JsStaticMemberExpression(
            static_expression,
        )) = self
        {
            let JsStaticMemberExpressionFields { member, object, .. } =
                static_expression.as_fields();

            Ok(SimpleArgument::from(member?).is_simple(depth)
                && SimpleArgument::from(object?).is_simple(depth))
        } else {
            Ok(false)
        }
    }

    fn is_simple_non_null_assertion_expression(&self, depth: u8) -> SyntaxResult<bool> {
        if let SimpleArgument::Expression(JsAnyExpression::TsNonNullAssertionExpression(
            assertion,
        )) = self
        {
            Ok(SimpleArgument::from(assertion.expression()?).is_simple(depth))
        } else {
            Ok(false)
        }
    }

    fn is_simple_unary_expression(&self, depth: u8) -> SyntaxResult<bool> {
        if let SimpleArgument::Expression(JsAnyExpression::JsUnaryExpression(unary_expression)) =
            self
        {
            if matches!(
                unary_expression.operator()?,
                JsUnaryOperator::LogicalNot | JsUnaryOperator::Minus
            ) {
                Ok(SimpleArgument::from(unary_expression.argument()?).is_simple(depth))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn is_simple_array_expression(&self, depth: u8) -> bool {
        if let SimpleArgument::Expression(JsAnyExpression::JsArrayExpression(array_expression)) =
            self
        {
            array_expression
                .elements()
                .iter()
                .filter_map(|element| element.ok())
                .all(|element| SimpleArgument::from(element).is_simple(depth + 1))
        } else {
            false
        }
    }

    fn is_simple_template(&self, depth: u8) -> bool {
        if let SimpleArgument::Expression(JsAnyExpression::JsTemplate(template)) = self {
            is_simple_template_literal(template, depth + 1).unwrap_or(false)
        } else {
            false
        }
    }

    fn is_simple_literal(&self) -> bool {
        match self {
            SimpleArgument::Expression(expression) => {
                matches!(
                    expression,
                    JsAnyExpression::JsAnyLiteralExpression(_)
                        | JsAnyExpression::JsThisExpression(_)
                        | JsAnyExpression::JsIdentifierExpression(_)
                        | JsAnyExpression::JsSuperExpression(_)
                )
            }
            SimpleArgument::Name(JsAnyName::JsPrivateName(_) | JsAnyName::JsName(_)) => true,
            _ => false,
        }
    }

    fn is_simple_object_expression(&self, depth: u8) -> bool {
        if let SimpleArgument::Expression(JsAnyExpression::JsObjectExpression(object_expression)) =
            self
        {
            object_expression
                .members()
                .iter()
                .filter_map(|member| member.ok())
                .all(|member| {
                    let is_shorthand_property = matches!(
                        member,
                        JsAnyObjectMember::JsShorthandPropertyObjectMember(_)
                    );
                    let is_simple = SimpleArgument::from(member.clone()).is_simple(depth + 1);
                    let is_computed_property =
                        if let JsAnyObjectMember::JsPropertyObjectMember(property) = member {
                            matches!(
                                property.name(),
                                Ok(JsAnyObjectMemberName::JsComputedMemberName(_))
                            )
                        } else {
                            false
                        };

                    !is_computed_property && (is_shorthand_property || is_simple)
                })
        } else {
            false
        }
    }
}

impl From<JsAnyExpression> for SimpleArgument {
    fn from(expr: JsAnyExpression) -> Self {
        Self::Expression(expr)
    }
}

impl From<JsAnyName> for SimpleArgument {
    fn from(name: JsAnyName) -> Self {
        Self::Name(name)
    }
}

impl From<JsAnyObjectMember> for SimpleArgument {
    fn from(member: JsAnyObjectMember) -> Self {
        Self::Member(member)
    }
}

impl From<JsAnyArrayElement> for SimpleArgument {
    fn from(element: JsAnyArrayElement) -> Self {
        Self::ArrayElement(element)
    }
}

impl From<JsSpread> for SimpleArgument {
    fn from(_: JsSpread) -> Self {
        Self::Spread
    }
}

impl From<JsAnyCallArgument> for SimpleArgument {
    fn from(call_argument: JsAnyCallArgument) -> Self {
        match call_argument {
            JsAnyCallArgument::JsAnyExpression(expr) => SimpleArgument::from(expr),
            JsAnyCallArgument::JsSpread(spread) => SimpleArgument::from(spread),
        }
    }
}

/// A template literal is simple when:
///
/// - all strings dont contain newlines
/// - the expressions contained in the template are considered as `is_simple_call_argument`. Check
/// [is_simple_call_argument].
pub fn is_simple_template_literal(template: &JsTemplate, depth: u8) -> SyntaxResult<bool> {
    for element in template.elements() {
        match element {
            JsAnyTemplateElement::JsTemplateChunkElement(chunk) => {
                if chunk.template_chunk_token()?.text_trimmed().contains('\n') {
                    return Ok(false);
                }
            }
            JsAnyTemplateElement::JsTemplateElement(element) => {
                let expression = element.expression()?;
                if !(SimpleArgument::from(expression).is_simple(depth)) {
                    return Ok(false);
                }
            }
        }
    }

    Ok(true)
}
