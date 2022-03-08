use crate::utils::{is_simple_expression, token_has_comments};
use crate::{format_elements, hard_group_elements};
use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsCallArgumentsFields;
use rome_js_syntax::{AstSeparatedList, JsAnyCallArgument, JsCallArguments, SyntaxResult};

impl ToFormatElement for JsCallArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = self.as_fields();

        if is_simple_function_arguments(self)? {
            return Ok(hard_group_elements(format_elements![
                l_paren_token.format(formatter)?,
                args.format(formatter)?,
                r_paren_token.format(formatter)?,
            ]));
        }

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            args.format(formatter)?,
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
