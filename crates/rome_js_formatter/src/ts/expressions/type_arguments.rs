use crate::prelude::*;
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
            ts_type_argument_list
                .iter()
                .next()
                .and_then(|ty| ty.ok())
                // first argument is mapped type or object type
                .and_then(|ty| {
                    if matches!(ty, TsType::TsObjectType(_) | TsType::TsMappedType(_)) {
                        ty.syntax().parent().and_then(|p| p.parent())
                    } else {
                        None
                    }
                })
                // we then go up two levels and see if we are inside a type annotation
                .and_then(|great_parent| {
                    if great_parent.kind() == JsSyntaxKind::TS_TYPE_ANNOTATION {
                        great_parent.parent()
                    } else {
                        None
                    }
                })
                // is so, we try to cast the parent into a variable declarator
                .and_then(JsVariableDeclarator::cast)
                // we extract the initializer
                .and_then(|variable_declarator| variable_declarator.initializer())
                // we verify if we have an arrow function expression
                .map(|initializer| {
                    let expression = initializer.expression().ok();
                    matches!(
                        expression,
                        Some(JsAnyExpression::JsArrowFunctionExpression(_))
                    )
                })
                .unwrap_or(false)
        };

        let should_inline = !is_arrow_function_variables
            && (ts_type_argument_list.len() == 0 || ts_type_argument_list.len() == 1);

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
