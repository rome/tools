use crate::builders::format_delimited;
use crate::utils::should_hug_type;
use crate::{prelude::*, utils::is_object_like_type};
use rome_formatter::write;
use rome_js_syntax::{
    JsAnyExpression, JsSyntaxKind, JsVariableDeclarator, TsType, TsTypeArguments,
    TsTypeArgumentsFields,
};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeArguments;

impl FormatNodeRule<TsTypeArguments> for FormatTsTypeArguments {
    fn fmt_fields(&self, node: &TsTypeArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeArgumentsFields {
            l_angle_token,
            ts_type_argument_list,
            r_angle_token,
        } = node.as_fields();

        // We want to check if we are inside something like this:
        // const foo: SomeThing<{ [P in "x" | "y"]: number }> = func();
        //
        //                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                      |_________________________|
        //                      that's where we start from
        let is_arrow_function_variables = {
            if let Some(first_argument) = ts_type_argument_list.iter().next() {
                let first_argument = first_argument?;

                // first argument is not mapped type or object type
                if !is_object_like_type(&first_argument) {
                    // we then go up until we can find a potential type annotation,
                    // meaning four levels up
                    let maybe_type_annotation = first_argument.syntax().ancestors().nth(4);

                    let initializer = maybe_type_annotation
                        .and_then(|maybe_type_annotation| {
                            if maybe_type_annotation.kind() == JsSyntaxKind::TS_TYPE_ANNOTATION {
                                maybe_type_annotation.parent()
                            } else {
                                None
                            }
                        })
                        // is so, we try to cast the parent into a variable declarator
                        .and_then(JsVariableDeclarator::cast)
                        // we extract the initializer
                        .and_then(|variable_declarator| variable_declarator.initializer());

                    if let Some(initializer) = initializer {
                        // we verify if we have an arrow function expression
                        let expression = initializer.expression()?;
                        matches!(expression, JsAnyExpression::JsArrowFunctionExpression(_))
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        };

        let first_argument_can_be_hugged_or_is_null_type = ts_type_argument_list.len() == 1
            && ts_type_argument_list.iter().next().map_or(false, |node| {
                node.map_or(false, |node| {
                    matches!(node, TsType::TsNullLiteralType(_)) || should_hug_type(&node)
                })
            });

        let should_inline = !is_arrow_function_variables
            && (ts_type_argument_list.len() == 0 || first_argument_can_be_hugged_or_is_null_type);

        if should_inline {
            write!(
                f,
                [
                    l_angle_token.format(),
                    ts_type_argument_list.format(),
                    r_angle_token.format(),
                ]
            )
        } else {
            write!(
                f,
                [format_delimited(
                    &l_angle_token?,
                    &ts_type_argument_list.format(),
                    &r_angle_token?,
                )
                .soft_block_indent()]
            )
        }
    }
}
