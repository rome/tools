use crate::prelude::*;
use rome_js_syntax::{JsAnyExpression, JsConditionalExpression, TsConditionalType, TsType};
use rome_rowan::AstNode;

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
        formatter: &JsFormatter,
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

        formatted![formatter, [head, body]]
    }

    fn format_head(&self, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        match self {
            Conditional::Expression(expr) => {
                formatted![formatter, [expr.test()?.format(), space_token(),]]
            }
            Conditional::Type(t) => formatted![
                formatter,
                [
                    t.check_type()?.format(),
                    space_token(),
                    t.extends_token()?.format(),
                    space_token(),
                    t.extends_type()?.format(),
                    space_token(),
                ]
            ],
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
        formatter: &JsFormatter,
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
            indent(formatted![
                formatter,
                [hard_line_break(), consequent, hard_line_break(), alternate]
            ]?)
        } else {
            group_elements(formatted![
                formatter,
                [space_token(), consequent, space_token(), alternate]
            ]?)
        };
        Ok(body)
    }

    fn format_with_consequent(
        &self,
        formatter: &JsFormatter,
        consequent: Option<FormatElement>,
    ) -> FormatResult<FormatElement> {
        match self {
            Conditional::Expression(expr) => {
                if let Some(consequent) = consequent {
                    formatted![
                        formatter,
                        [
                            expr.question_mark_token().format(),
                            space_token(),
                            consequent
                        ]
                    ]
                } else {
                    formatted![
                        formatter,
                        [
                            expr.question_mark_token().format(),
                            space_token(),
                            expr.consequent().format()
                        ]
                    ]
                }
            }
            Conditional::Type(ty) => {
                if let Some(consequent) = consequent {
                    formatted![
                        formatter,
                        [ty.question_mark_token().format(), space_token(), consequent]
                    ]
                } else {
                    formatted![
                        formatter,
                        [
                            ty.question_mark_token().format(),
                            space_token(),
                            ty.true_type().format()
                        ]
                    ]
                }
            }
        }
    }

    fn format_with_alternate(
        &self,
        formatter: &JsFormatter,
        alternate: Option<FormatElement>,
    ) -> FormatResult<FormatElement> {
        match self {
            Conditional::Expression(expr) => {
                if let Some(alternate) = alternate {
                    formatted![
                        formatter,
                        [expr.colon_token().format(), space_token(), alternate]
                    ]
                } else {
                    formatted![
                        formatter,
                        [
                            expr.colon_token().format(),
                            space_token(),
                            expr.alternate().format()
                        ]
                    ]
                }
            }
            Conditional::Type(ty) => {
                if let Some(alternate) = alternate {
                    formatted![
                        formatter,
                        [ty.colon_token().format(), space_token(), alternate]
                    ]
                } else {
                    formatted![
                        formatter,
                        [
                            ty.colon_token().format(),
                            space_token(),
                            ty.false_type().format()
                        ]
                    ]
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
    formatter: &JsFormatter,
    parent_is_conditional: bool,
) -> FormatResult<FormatElement> {
    conditional.into_format_element(formatter, parent_is_conditional)
}
