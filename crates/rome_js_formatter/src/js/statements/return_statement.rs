use crate::prelude::*;
use crate::utils::{FormatWithSemicolon, JsAnyBinaryLikeExpression, JsAnyBinaryLikeLeftExpression};

use rome_formatter::{
    format_args, has_leading_own_line_comment, write, Comments, CstFormatContext,
};

use crate::parentheses::get_expression_left_side;
use rome_js_syntax::{
    JsAnyExpression, JsLanguage, JsReturnStatement, JsSequenceExpression, JsSyntaxToken,
    JsThrowStatement,
};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsReturnStatement;

impl FormatNodeRule<JsReturnStatement> for FormatJsReturnStatement {
    fn fmt_fields(&self, node: &JsReturnStatement, f: &mut JsFormatter) -> FormatResult<()> {
        JsAnyStatementWithArgument::from(node.clone()).fmt(f)
    }
}

declare_node_union! {
    pub(super) JsAnyStatementWithArgument = JsThrowStatement | JsReturnStatement
}

impl Format<JsFormatContext> for JsAnyStatementWithArgument {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        write!(f, [self.operation_token().format()])?;

        let argument = self.argument()?;

        if let Some(semicolon) = self.semicolon_token() {
            if let Some(argument) = argument {
                write!(f, [space(), FormatReturnOrThrowArgument(&argument)])?;
            }

            let comments = f.context().comments();
            let has_dangling_comments = comments.has_dangling_comments(&semicolon);

            let is_last_comment_line = has_dangling_comments
                && comments
                    .dangling_comments(&semicolon)
                    .chain(comments.trailing_comments(self.syntax()))
                    .last()
                    .map_or(false, |comment| comment.kind().is_line());

            // We'll format it after the semicolon
            f.state_mut().mark_token_trivia_formatted(&semicolon);

            if is_last_comment_line {
                write!(f, [semicolon.format()])?;
            }

            if has_dangling_comments {
                write!(
                    f,
                    [
                        space(),
                        format_dangling_trivia(&semicolon).ignore_formatted_check()
                    ]
                )?;
            }

            if !is_last_comment_line {
                write!(f, [semicolon.format()])?;
            }

            Ok(())
        } else {
            write!(
                f,
                [FormatWithSemicolon::new(
                    &format_with(|f| {
                        if let Some(argument) = &argument {
                            write!(f, [space(), FormatReturnOrThrowArgument(&argument)])?;
                        }

                        Ok(())
                    }),
                    None
                )]
            )
        }
    }
}

impl JsAnyStatementWithArgument {
    fn operation_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyStatementWithArgument::JsThrowStatement(throw) => throw.throw_token(),
            JsAnyStatementWithArgument::JsReturnStatement(ret) => ret.return_token(),
        }
    }

    fn argument(&self) -> SyntaxResult<Option<JsAnyExpression>> {
        match self {
            JsAnyStatementWithArgument::JsThrowStatement(throw) => throw.argument().map(Some),
            JsAnyStatementWithArgument::JsReturnStatement(ret) => Ok(ret.argument()),
        }
    }

    fn semicolon_token(&self) -> Option<JsSyntaxToken> {
        match self {
            JsAnyStatementWithArgument::JsThrowStatement(throw) => throw.semicolon_token(),
            JsAnyStatementWithArgument::JsReturnStatement(ret) => ret.semicolon_token(),
        }
    }
}

pub(super) struct FormatReturnOrThrowArgument<'a>(&'a JsAnyExpression);

impl Format<JsFormatContext> for FormatReturnOrThrowArgument<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let argument = self.0;

        if has_argument_leading_comments(argument, f.context().comments()) {
            write!(f, [text("("), &block_indent(&argument.format()), text(")")])
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
fn has_argument_leading_comments(
    argument: &JsAnyExpression,
    comments: &Comments<JsLanguage>,
) -> bool {
    let mut current: Option<JsAnyBinaryLikeLeftExpression> = Some(argument.clone().into());

    while let Some(expression) = current {
        if has_leading_own_line_comment(expression.syntax(), comments) {
            return true;
        }

        match expression {
            JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => {
                current = get_expression_left_side(&expression);
            }
            JsAnyBinaryLikeLeftExpression::JsPrivateName(_) => {
                break;
            }
        }
    }

    false
}

fn is_binary_or_sequence_argument(argument: &JsAnyExpression) -> bool {
    JsSequenceExpression::can_cast(argument.syntax().kind())
        || JsAnyBinaryLikeExpression::can_cast(argument.syntax().kind())
}
