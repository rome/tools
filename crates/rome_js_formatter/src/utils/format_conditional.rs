use crate::prelude::*;
use rome_formatter::{format_args, write, CstFormatContext};
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

    fn format_head(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            Conditional::Expression(expr) => {
                if f.context().comments().is_suppressed(expr.syntax()) {
                    write!(f, [format_suppressed_node(expr.syntax())])
                } else {
                    write![f, [expr.test()?.format(), space(),]]
                }
            }
            Conditional::Type(t) => {
                if f.context().comments().is_suppressed(t.syntax()) {
                    write!(f, [format_suppressed_node(t.syntax())])
                } else {
                    write![
                        f,
                        [
                            t.check_type()?.format(),
                            space(),
                            t.extends_token()?.format(),
                            space(),
                            t.extends_type()?.format(),
                            space(),
                        ]
                    ]
                }
            }
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

    fn format_body(&self, f: &mut JsFormatter, parent_is_conditional: bool) -> FormatResult<()> {
        let consequent = self.consequent_to_conditional()?;
        let alternate = self.alternate_to_conditional()?;
        let left_or_right_is_conditional = alternate.is_some() || consequent.is_some();

        let format_consequent =
            format_with(|f| self.format_with_consequent(f, consequent.as_ref()));

        let format_alternate = format_with(|f| self.format_with_alternate(f, alternate.as_ref()));

        if left_or_right_is_conditional || parent_is_conditional {
            write!(
                f,
                [indent(&format_args![
                    hard_line_break(),
                    format_consequent,
                    hard_line_break(),
                    format_alternate
                ])]
            )?;
        } else {
            write!(
                f,
                [group(&format_args![
                    space(),
                    format_consequent,
                    space(),
                    format_alternate
                ])]
            )?;
        };

        Ok(())
    }

    fn format_with_consequent(
        &self,
        f: &mut JsFormatter,
        consequent: Option<&Conditional>,
    ) -> FormatResult<()> {
        match consequent {
            Some(consequent) => {
                let format_consequent = format_with(|f| format_conditional(consequent, f, true));

                match self {
                    Conditional::Expression(expr) => {
                        write![
                            f,
                            [
                                expr.question_mark_token().format(),
                                space(),
                                format_consequent
                            ]
                        ]
                    }
                    Conditional::Type(ty) => {
                        write![
                            f,
                            [
                                ty.question_mark_token().format(),
                                space(),
                                format_consequent
                            ]
                        ]
                    }
                }
            }
            None => match self {
                Conditional::Expression(expr) => {
                    write![
                        f,
                        [
                            expr.question_mark_token().format(),
                            space(),
                            expr.consequent().format()
                        ]
                    ]
                }
                Conditional::Type(ty) => {
                    write![
                        f,
                        [
                            ty.question_mark_token().format(),
                            space(),
                            ty.true_type().format()
                        ]
                    ]
                }
            },
        }
    }

    fn format_with_alternate(
        &self,
        f: &mut JsFormatter,
        alternate: Option<&Conditional>,
    ) -> FormatResult<()> {
        match alternate {
            Some(alternate) => {
                let format_alternate = format_with(|f| format_conditional(alternate, f, true));

                match self {
                    Conditional::Expression(expr) => {
                        write![f, [expr.colon_token().format(), space(), format_alternate]]
                    }
                    Conditional::Type(ty) => {
                        write![f, [ty.colon_token().format(), space(), format_alternate]]
                    }
                }
            }

            None => match self {
                Conditional::Expression(expr) => write![
                    f,
                    [
                        expr.colon_token().format(),
                        space(),
                        expr.alternate().format()
                    ]
                ],
                Conditional::Type(ty) => write![
                    f,
                    [ty.colon_token().format(), space(), ty.false_type().format()]
                ],
            },
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
    conditional: &Conditional,
    f: &mut JsFormatter,
    parent_is_conditional: bool,
) -> FormatResult<()> {
    conditional.format_head(f)?;
    conditional.format_body(f, parent_is_conditional)
}
