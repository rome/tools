use crate::prelude::*;
use crate::utils::{FormatWithSemicolon, JsAnyBinaryLikeExpression, JsAnyBinaryLikeLeftExpression};

use rome_formatter::{format_args, write};

use crate::parentheses::get_expression_left_side;
use rome_js_syntax::{
    JsAnyExpression, JsReturnStatement, JsReturnStatementFields, JsSequenceExpression,
};
use rome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub struct FormatJsReturnStatement;

impl FormatNodeRule<JsReturnStatement> for FormatJsReturnStatement {
    fn fmt_fields(&self, node: &JsReturnStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsReturnStatementFields {
            return_token,
            argument,
            semicolon_token,
        } = node.as_fields();

        let format_inner = format_with(|f| {
            write!(f, [return_token.format()])?;

            if let Some(argument) = &argument {
                write!(f, [space(), FormatReturnOrThrowArgument(argument)])?;
            }

            Ok(())
        });

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_inner,
                semicolon_token.as_ref()
            )]
        )
    }
}

pub(super) struct FormatReturnOrThrowArgument<'a>(&'a JsAnyExpression);

impl<'a> FormatReturnOrThrowArgument<'a> {
    pub fn new(argument: &'a JsAnyExpression) -> Self {
        Self(argument)
    }
}

impl Format<JsFormatContext> for FormatReturnOrThrowArgument<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let argument = self.0;

        if has_argument_leading_comments(argument)? {
            let syntax = argument.syntax();
            let first_token = syntax.first_token();
            let last_token = syntax.last_token();
            write!(
                f,
                [format_parenthesize(
                    first_token.as_ref(),
                    &block_indent(&argument.format()),
                    last_token.as_ref()
                )]
            )
        } else if is_binary_or_sequence_argument(argument) {
            write!(
                f,
                [group(&format_args![
                    if_group_breaks(&text("(")),
                    soft_block_indent(&argument.format()),
                    if_group_breaks(&text(")"))
                ])]
            )
        } else {
            write!(f, [argument.format()])
        }
    }
}

/// Tests if the passed in argument has any leading comments. This is the case if
/// * the argument has any leading comment
/// * the argument's left side has any leading comment (see [get_expression_left_side]).
///
/// Traversing the left nodes is necessary in case the first node is parenthesized because
/// parentheses will be removed (and be re-added by the return statement, but only if the argument breaks)
fn has_argument_leading_comments(argument: &JsAnyExpression) -> SyntaxResult<bool> {
    if matches!(argument, JsAnyExpression::JsxTagExpression(_)) {
        // JSX formatting takes care of adding parens
        return Ok(false);
    }

    if argument.syntax().has_leading_comments() {
        return Ok(true);
    }

    let result = match get_expression_left_side(argument) {
        Some(JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression)) => {
            has_argument_leading_comments(&expression)?
        }
        Some(JsAnyBinaryLikeLeftExpression::JsPrivateName(name)) => {
            name.syntax().has_leading_comments()
        }
        None => false,
    };

    Ok(result)
}

fn is_binary_or_sequence_argument(argument: &JsAnyExpression) -> bool {
    JsSequenceExpression::can_cast(argument.syntax().kind())
        || JsAnyBinaryLikeExpression::can_cast(argument.syntax().kind())
}
