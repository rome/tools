use crate::prelude::*;
use crate::utils::{is_simple_expression, token_has_comments};

use crate::FormatNodeFields;
use rome_js_syntax::JsCallArgumentsFields;
use rome_js_syntax::{JsAnyCallArgument, JsCallArguments};
use rome_rowan::{AstSeparatedList, SyntaxResult};

impl FormatNodeFields<JsCallArguments> for FormatNodeRule<JsCallArguments> {
    fn format_fields(node: &JsCallArguments, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = node.as_fields();

        if is_simple_function_arguments(node)? {
            return Ok(hard_group_elements(formatted![
                formatter,
                l_paren_token.format(),
                args.format(),
                r_paren_token.format(),
            ]?));
        }

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            formatted![formatter, args.format()]?,
            &r_paren_token?,
        )
    }
}

/// Returns true if the passed [JsCallArguments] has a single argument
/// that is a simple function expression, array expression or object expression
fn is_simple_function_arguments(node: &JsCallArguments) -> SyntaxResult<bool> {
    let JsCallArgumentsFields {
        l_paren_token,
        args,
        r_paren_token,
    } = node.as_fields();

    if token_has_comments(l_paren_token?) || token_has_comments(r_paren_token?) {
        return Ok(false);
    }

    if args.syntax_list().len() > 1 {
        return Ok(false);
    }

    for item in args {
        match item {
            Ok(JsAnyCallArgument::JsAnyExpression(expr)) => {
                if !is_simple_expression(expr)? {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }
    }

    Ok(true)
}
