use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    empty_element, format_elements, group_elements, soft_line_indent_or_space, space_token, token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsAnyExpression, JsAnyInProperty, JsAssignmentExpression, JsAwaitExpression,
    JsBinaryExpression, JsComputedMemberExpression, JsConditionalExpression, JsInExpression,
    JsInstanceofExpression, JsLogicalExpression, JsNewExpression, JsParenthesizedExpression,
    JsThisExpression, JsUnaryExpression, JsYieldArgument, JsYieldExpression, NewTarget,
};
use rslint_parser::{token_set, T};

impl ToFormatElement for JsAnyExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyExpression::JsArrowFunctionExpression(arrow) => arrow.to_format_element(formatter),
            JsAnyExpression::JsAnyLiteralExpression(literal) => {
                literal.to_format_element(formatter)
            }
            JsAnyExpression::JsTemplate(node) => node.to_format_element(formatter),
            JsAnyExpression::JsIdentifierExpression(identifier_expr) => {
                identifier_expr.to_format_element(formatter)
            }
            JsAnyExpression::JsThisExpression(this_expression) => {
                this_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsArrayExpression(array_expression) => {
                array_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsObjectExpression(object_expression) => {
                object_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsParenthesizedExpression(parenthesized_expression) => {
                parenthesized_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsComputedMemberExpression(computed_member_expression) => {
                computed_member_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsStaticMemberExpression(static_member_expression) => {
                static_member_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsNewExpression(new_expr) => new_expr.to_format_element(formatter),
            JsAnyExpression::JsCallExpression(call_expression) => {
                call_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsUnaryExpression(unary_expression) => {
                unary_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsBinaryExpression(binary_expression) => {
                binary_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsConditionalExpression(conditional_expression) => {
                conditional_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsAssignmentExpression(assignment_expression) => {
                assignment_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsSequenceExpression(expr) => expr.to_format_element(formatter),
            JsAnyExpression::JsFunctionExpression(function_expression) => {
                function_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsClassExpression(class_expression) => {
                class_expression.to_format_element(formatter)
            }
            JsAnyExpression::NewTarget(expr) => expr.to_format_element(formatter),
            JsAnyExpression::ImportMeta(import_meta) => import_meta.to_format_element(formatter),
            JsAnyExpression::JsImportCallExpression(import_call_expr) => {
                import_call_expr.to_format_element(formatter)
            }
            JsAnyExpression::JsYieldExpression(yield_expression) => {
                yield_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsAwaitExpression(await_expression) => {
                await_expression.to_format_element(formatter)
            }
            JsAnyExpression::TsNonNull(_) => todo!(),
            JsAnyExpression::JsPreUpdateExpression(pre_update_expression) => {
                pre_update_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsPostUpdateExpression(post_update_expression) => {
                post_update_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsUnknownExpression(unknown_expression) => {
                unknown_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsLogicalExpression(logical_expression) => {
                logical_expression.to_format_element(formatter)
            }
            JsAnyExpression::JsSuperExpression(expr) => expr.to_format_element(formatter),
            JsAnyExpression::JsInExpression(expression) => expression.to_format_element(formatter),
            JsAnyExpression::JsInstanceofExpression(expression) => {
                expression.to_format_element(formatter)
            }
            JsAnyExpression::TsAsExpression(_) => todo!(),
            JsAnyExpression::TsTypeAssertionExpression(_) => todo!(),
        }
    }
}

impl ToFormatElement for JsThisExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.this_token().format(formatter)
    }
}

impl ToFormatElement for JsParenthesizedExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.l_paren_token().format(formatter)?,
            self.expression().format(formatter)?,
            self.r_paren_token().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsComputedMemberExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let optional_chain_token = self.optional_chain_token().format_or_empty(formatter)?;

        Ok(format_elements![
            self.object().format(formatter)?,
            optional_chain_token,
            self.l_brack_token().format(formatter)?,
            self.member().format(formatter)?,
            self.r_brack_token().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsNewExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let arguments = self.arguments().format_with_or(
            formatter,
            |arguments| arguments,
            || format_elements![token("("), token(")")],
        )?;

        Ok(format_elements![
            self.new_token().format(formatter)?,
            // TODO handle TsTypeArgs
            space_token(),
            self.callee().format(formatter)?,
            arguments,
        ])
    }
}

impl ToFormatElement for JsUnaryExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let operator = self.operator()?;
        let space_or_empty =
            if token_set![T![delete], T![void], T![typeof]].contains(operator.kind()) {
                space_token()
            } else {
                empty_element()
            };
        Ok(format_elements![
            self.operator().format(formatter)?,
            space_or_empty,
            self.argument().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsBinaryExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.left().format(formatter)?,
            space_token(),
            self.operator().format(formatter)?,
            space_token(),
            self.right().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsConditionalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.test().format(formatter)?,
            space_token(),
            self.question_mark_token().format(formatter)?,
            space_token(),
            self.consequent().format(formatter)?,
            space_token(),
            self.colon_token().format(formatter)?,
            space_token(),
            self.alternate().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsAssignmentExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(group_elements(format_elements![
            self.left().format(formatter)?,
            space_token(),
            self.operator_token().format(formatter)?,
            group_elements(soft_line_indent_or_space(self.right().format(formatter)?)),
        ]))
    }
}

impl ToFormatElement for NewTarget {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.new_token().format(formatter)?,
            self.dot_token().format(formatter)?,
            self.target_token().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsYieldExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let argument = self.argument().format_or_empty(formatter)?;

        Ok(format_elements![
            self.yield_token().format(formatter)?,
            argument
        ])
    }
}

impl ToFormatElement for JsYieldArgument {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let star_token = self.star_token().format_or_empty(formatter)?;

        Ok(format_elements![
            star_token,
            space_token(),
            self.expression().format(formatter)?
        ])
    }
}

impl ToFormatElement for JsAwaitExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.await_token().format(formatter)?,
            space_token(),
            self.argument().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsLogicalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.left().format(formatter)?,
            space_token(),
            self.operator().format(formatter)?,
            space_token(),
            self.right().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsInstanceofExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.left().format(formatter)?,
            space_token(),
            self.instanceof_token().format(formatter)?,
            space_token(),
            self.right().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsInExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.property().format(formatter)?,
            space_token(),
            self.in_token().format(formatter)?,
            space_token(),
            self.object().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsAnyInProperty {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyInProperty::JsAnyExpression(expression) => expression.to_format_element(formatter),
            JsAnyInProperty::JsPrivateName(name) => name.to_format_element(formatter),
        }
    }
}
