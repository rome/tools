use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, hard_line_break, indent, space_token, FormatElement,
    FormatResult, Formatter,
};
use rome_js_syntax::AstNode;
use rome_js_syntax::{JsAnyExpression, JsConditionalExpression, TsConditionalType, TsType};

pub enum Conditional {
    Expression(JsConditionalExpression),
    Type(TsConditionalType),
}

impl Conditional {
    fn from_type(ts_type: TsType) -> Option<Self> {
        if let Some(TsType::TsConditionalType(conditional)) = TsType::cast(ts_type.syntax().clone())
        {
            Some(Self::Type(conditional))
        } else {
            None
        }
    }

    fn from_expression(any_expression: JsAnyExpression) -> Option<Self> {
        if let Some(JsAnyExpression::JsConditionalExpression(conditional)) =
            JsAnyExpression::cast(any_expression.syntax().clone())
        {
            Some(Self::Expression(conditional))
        } else {
            None
        }
    }

    fn into_format_element(
        self,
        formatter: &Formatter,
        parent_is_conditional: bool,
    ) -> FormatResult<FormatElement> {
        let (head, body) = match self {
            Conditional::Expression(_) => (
                self.format_head(formatter)?,
                self.format_body(formatter, parent_is_conditional)?,
            ),
            Conditional::Type(_) => (
                self.format_head(formatter)?,
                self.format_body(formatter, parent_is_conditional)?,
            ),
        };

        Ok(format_elements![head, body])
    }

    fn format_head(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Conditional::Expression(expr) => Ok(format_elements![
                expr.test()?.format(formatter)?,
                space_token(),
            ]),
            Conditional::Type(t) => Ok(format_elements![
                t.check_type()?.format(formatter)?,
                space_token(),
                t.extends_token()?.format(formatter)?,
                space_token(),
                t.extends_type()?.format(formatter)?,
                space_token(),
            ]),
        }
    }
    fn consequent_to_conditional(&self) -> FormatResult<Option<Self>> {
        match self {
            Conditional::Expression(conditional_expression) => Ok(Conditional::from_expression(
                conditional_expression.consequent()?,
            )),
            Conditional::Type(conditional_type) => {
                Ok(Conditional::from_type(conditional_type.true_type()?))
            }
        }
    }

    fn alternate_to_conditional(&self) -> FormatResult<Option<Self>> {
        match self {
            Conditional::Expression(conditional_expression) => Ok(Conditional::from_expression(
                conditional_expression.alternate()?,
            )),
            Conditional::Type(conditional_type) => {
                Ok(Conditional::from_type(conditional_type.false_type()?))
            }
        }
    }

    fn format_body(
        &self,
        formatter: &Formatter,
        parent_is_conditional: bool,
    ) -> FormatResult<FormatElement> {
        let mut left_or_right_is_conditional = false;
        let consequent = self.consequent_to_conditional()?;
        let alternate = self.alternate_to_conditional()?;

        let consequent = if let Some(consequent) = consequent {
            left_or_right_is_conditional = true;
            let consequent = format_conditional(consequent, formatter, true)?;
            self.format_with_consequent(formatter, Some(consequent))?
        } else {
            self.format_with_consequent(formatter, None)?
        };

        let alternate = if let Some(alternate) = alternate {
            left_or_right_is_conditional = true;
            let alternate = format_conditional(alternate, formatter, true)?;
            self.format_with_alternate(formatter, Some(alternate))?
        } else {
            self.format_with_alternate(formatter, None)?
        };

        let body = if left_or_right_is_conditional || parent_is_conditional {
            indent(format_elements![
                hard_line_break(),
                consequent,
                hard_line_break(),
                alternate
            ])
        } else {
            group_elements(format_elements![
                space_token(),
                consequent,
                space_token(),
                alternate
            ])
        };
        Ok(body)
    }

    fn format_with_consequent(
        &self,
        formatter: &Formatter,
        consequent: Option<FormatElement>,
    ) -> FormatResult<FormatElement> {
        match self {
            Conditional::Expression(expr) => {
                if let Some(consequent) = consequent {
                    Ok(format_elements![
                        expr.question_mark_token().format(formatter)?,
                        space_token(),
                        consequent
                    ])
                } else {
                    Ok(format_elements![
                        expr.question_mark_token().format(formatter)?,
                        space_token(),
                        expr.consequent().format(formatter)?
                    ])
                }
            }
            Conditional::Type(ty) => {
                if let Some(consequent) = consequent {
                    Ok(format_elements![
                        ty.question_mark_token().format(formatter)?,
                        space_token(),
                        consequent
                    ])
                } else {
                    Ok(format_elements![
                        ty.question_mark_token().format(formatter)?,
                        space_token(),
                        ty.true_type().format(formatter)?
                    ])
                }
            }
        }
    }

    fn format_with_alternate(
        &self,
        formatter: &Formatter,
        alternate: Option<FormatElement>,
    ) -> FormatResult<FormatElement> {
        match self {
            Conditional::Expression(expr) => {
                if let Some(alternate) = alternate {
                    Ok(format_elements![
                        expr.colon_token().format(formatter)?,
                        space_token(),
                        alternate
                    ])
                } else {
                    Ok(format_elements![
                        expr.colon_token().format(formatter)?,
                        space_token(),
                        expr.alternate().format(formatter)?
                    ])
                }
            }
            Conditional::Type(ty) => {
                if let Some(alternate) = alternate {
                    Ok(format_elements![
                        ty.colon_token().format(formatter)?,
                        space_token(),
                        alternate
                    ])
                } else {
                    Ok(format_elements![
                        ty.colon_token().format(formatter)?,
                        space_token(),
                        ty.false_type().format(formatter)?
                    ])
                }
            }
        }
    }
}

/// Utility function to use to format ternary operators
///
/// # Panics
///
/// It panics if it's used with nodes that are different from:
/// - [rome_js_syntax::TsConditionalType]
/// - [rome_js_syntax::JsConditionalExpression]
pub fn format_conditional(
    conditional: Conditional,
    formatter: &Formatter,
    parent_is_conditional: bool,
) -> FormatResult<FormatElement> {
    conditional.into_format_element(formatter, parent_is_conditional)
}
