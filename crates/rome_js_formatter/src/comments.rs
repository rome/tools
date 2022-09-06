use crate::prelude::*;
use crate::utils::JsAnyBinaryLikeExpression;
use rome_formatter::{write, CommentPosition, DecoratedComment};
use rome_formatter::{CommentKind, CommentStyle, SourceComment};
use rome_js_syntax::suppression::{parse_suppression_comment, SuppressionCategory};
use rome_js_syntax::{
    JsAnyStatement, JsArrayAssignmentPattern, JsArrayBindingPattern, JsArrayExpression,
    JsArrayHole, JsBlockStatement, JsBreakStatement, JsCallArgumentList, JsCallArguments,
    JsContinueStatement, JsDefaultClause, JsFunctionBody, JsLanguage, JsSyntaxKind, JsSyntaxNode,
    JsSyntaxToken,
};
use rome_rowan::{
    declare_node_union, match_ast, SyntaxKind, SyntaxResult, SyntaxTriviaPieceComments, TextLen,
};

#[derive(Default)]
pub struct FormatJsLeadingComment;

impl FormatRule<SourceComment<JsLanguage>> for FormatJsLeadingComment {
    type Context = JsFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<JsLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        if is_doc_comment(comment.piece()) {
            let mut source_offset = comment.piece().text_range().start();

            let mut lines = comment.piece().text().lines();

            // SAFETY: Safe, `is_doc_comment` only returns `true` for multiline comments
            let first_line = lines.next().unwrap();
            write!(f, [dynamic_text(first_line.trim_end(), source_offset)])?;

            source_offset += first_line.text_len();

            // Indent the remaining lines by one space so that all `*` are aligned.
            write!(
                f,
                [align(
                    1,
                    &format_once(|f| {
                        for line in lines {
                            write!(
                                f,
                                [hard_line_break(), dynamic_text(line.trim(), source_offset)]
                            )?;

                            source_offset += line.text_len();
                        }

                        Ok(())
                    })
                )]
            )
        } else {
            write!(f, [comment.piece().as_piece()])
        }
    }
}

/// Returns `true` if `comment` is a multi line block comment:
///
/// # Examples
///
/// ```
/// # use rome_js_parser::parse_module;
/// # use rome_js_syntax::JsLanguage;
/// # use rome_rowan::{Direction, SyntaxTriviaPieceComments};
///  use rome_js_formatter::comments::is_doc_comment;
///
/// # fn parse_comment(source: &str) -> SyntaxTriviaPieceComments<JsLanguage> {
/// #     let root = parse_module(source, 0).tree();
/// #     root
/// #        .eof_token()
/// #        .expect("Root to have an EOF token")
/// #        .leading_trivia()
/// #        .pieces()
/// #        .filter_map(|piece| piece.as_comments())
/// #        .next()
/// #        .expect("Source to contain a comment.")
/// # }
///
/// assert!(is_doc_comment(&parse_comment(r#"
///     /**
///      * Multiline doc comment
///      */
/// "#)));
///
/// assert!(is_doc_comment(&parse_comment(r#"
///     /*
///      * Single star
///      */
/// "#)));
///
///
/// // Non doc-comments
/// assert!(!is_doc_comment(&parse_comment(r#"/** has no line break */"#)));
///
/// assert!(!is_doc_comment(&parse_comment(r#"
/// /*
///  *
///  this line doesn't start with a star
///  */
/// "#)));
/// ```
pub fn is_doc_comment(comment: &SyntaxTriviaPieceComments<JsLanguage>) -> bool {
    if !comment.has_newline() {
        return false;
    }

    let text = comment.text();

    text.lines().enumerate().all(|(index, line)| {
        if index == 0 {
            line.starts_with("/*")
        } else {
            line.trim_start().starts_with('*')
        }
    })
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct JsCommentStyle;

impl CommentStyle for JsCommentStyle {
    type Language = JsLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .flat_map(|suppression| suppression.categories)
            .any(|(category, _)| category == SuppressionCategory::Format)
    }

    fn get_comment_kind(comment: &SyntaxTriviaPieceComments<JsLanguage>) -> CommentKind {
        if comment.text().starts_with("/*") {
            if comment.has_newline() {
                CommentKind::Block
            } else {
                CommentKind::InlineBlock
            }
        } else {
            CommentKind::Line
        }
    }

    fn position_comment(
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPosition<Self::Language> {
        let enclosing_node = comment.enclosing_node();

        if let Some(following_node) = comment.following_node() {
            match following_node.kind() {
                // Move leading comments in front of the `{` inside of the block
                // ```
                // if (test) /* comment */ {
                //  console.log('test');
                // }
                // ```
                //
                // becomes
                // ```
                // if (test) {
                //  /* comment */ console.log('test');
                // }
                // ```
                JsSyntaxKind::JS_BLOCK_STATEMENT
                    if !JsDefaultClause::can_cast(enclosing_node.kind()) =>
                {
                    let block = JsBlockStatement::unwrap_cast(following_node.clone());

                    if let (Ok(_), Ok(r_curly_token)) =
                        (block.l_curly_token(), block.r_curly_token())
                    {
                        return match block.statements().first() {
                            Some(JsAnyStatement::JsEmptyStatement(_)) => {
                                CommentPosition::Dangling {
                                    token: r_curly_token,
                                    comment,
                                }
                            }
                            Some(first_statement) => CommentPosition::Leading {
                                node: first_statement.into_syntax(),
                                comment,
                            },
                            _ => CommentPosition::Dangling {
                                token: r_curly_token,
                                comment,
                            },
                        };
                    }
                }

                // Move comments in front of the `{` inside of the function body
                JsSyntaxKind::JS_FUNCTION_BODY
                    if (!comment.is_trailing_token_trivia() || comment.kind().is_line()) =>
                {
                    let function_body = JsFunctionBody::unwrap_cast(following_node.clone());

                    if let (Ok(_), Ok(r_curly_token)) =
                        (function_body.l_curly_token(), function_body.r_curly_token())
                    {
                        let first_directive = function_body
                            .directives()
                            .first()
                            .map(|node| node.into_syntax());
                        let first_statement = function_body
                            .statements()
                            .first()
                            .map(|node| node.into_syntax());
                        return if let Some(first_node) = first_directive.or(first_statement) {
                            CommentPosition::Leading {
                                node: first_node,
                                comment,
                            }
                        } else {
                            CommentPosition::Dangling {
                                token: r_curly_token,
                                comment,
                            }
                        };
                    }
                }
                _ => {
                    // fall through
                }
            }
        };

        match enclosing_node.kind() {
            // TODO move to general formatter handling?
            // WHat does that mean for invalid syntax...
            kind if kind.is_unknown() => {
                return CommentPosition::Dangling {
                    token: comment.enclosing_token(),
                    comment,
                }
            }
            // Handles comments attached to operators of binary like expressions.
            //
            // Associates trailing comments with the left expression if they're directly followed by a line break.
            // ```javascript
            // a = b || /** Comment */
            // c;
            //
            // a = b /** Comment */ ||
            // c;
            // ```
            //
            // Associates leading operator comments with the right side
            // ```javascript
            // 	0
            // 	// Comment
            // 	+ x
            // ```
            kind if JsAnyBinaryLikeExpression::can_cast(kind) => {
                let binary_like = JsAnyBinaryLikeExpression::unwrap_cast(enclosing_node.clone());

                if comment.is_trailing_token_trivia() && comment.lines_after() > 0 {
                    if let Ok(left) = binary_like.left() {
                        return CommentPosition::Trailing {
                            node: left.into_syntax(),
                            comment,
                        };
                    }
                } else {
                    if let Ok(right) = binary_like.right() {
                        return CommentPosition::Leading {
                            node: right.into_syntax(),
                            comment,
                        };
                    }
                }
            }

            // Makes comments of `break` and `continue` statements trailing comments EXCEPT if there's a label
            //
            // ```javascript
            // break /* comment */
            // break /* comment */;
            // ```
            JsSyntaxKind::JS_BREAK_STATEMENT | JsSyntaxKind::JS_CONTINUE_STATEMENT => {
                let (argument, semicolon) = match_ast! {
                    match &enclosing_node {
                        JsBreakStatement(break_statement) => (break_statement.label_token(), break_statement.semicolon_token()),
                        JsContinueStatement(continue_statement) => (continue_statement.label_token(), continue_statement.semicolon_token()),
                        _ => unreachable!()
                    }
                };

                if argument.is_none()
                    && (semicolon.is_none()
                        || Some(comment.following_token()) == semicolon.as_ref())
                {
                    return CommentPosition::Trailing {
                        node: enclosing_node,
                        comment,
                    };
                }
            }

            JsSyntaxKind::JS_FOR_IN_STATEMENT | JsSyntaxKind::JS_FOR_OF_STATEMENT => {
                return CommentPosition::Leading {
                    node: enclosing_node,
                    comment,
                }
            }

            _ => {
                // fall through
            }
        }

        if let Some(preceding_node) = comment.preceding_node() {
            // Handles array hole comments. Array holes have no token so all comments
            // become trailing comments by default. Override it that all comments are elading comments.
            if JsArrayHole::can_cast(preceding_node.kind()) {
                return CommentPosition::Leading {
                    node: preceding_node.clone(),
                    comment,
                };
            }
        }

        match dbg!(comment.following_token().kind()) {
            JsSyntaxKind::R_BRACK => {
                // Handles comments before the `]` token of an array
                //
                // ```javascript
                // let example = [
                // 	"FOO",
                // 	"BAR",
                // 	// Comment
                // ];
                // ```
                // Makes the comment before the `]` a trailing comment of the last element.
                if !comment.is_trailing_token_trivia()
                    && JsAnyArrayLike::can_cast(enclosing_node.kind())
                {
                    let array = JsAnyArrayLike::unwrap_cast(enclosing_node.clone());

                    if array.r_brack_token().as_ref() == Ok(comment.following_token()) {
                        if let Some(Ok(last_element)) = array.last_element() {
                            if last_element.kind() == JsSyntaxKind::JS_ARRAY_HOLE {
                                return CommentPosition::Leading {
                                    node: last_element,
                                    comment,
                                };
                            } else {
                                return CommentPosition::Trailing {
                                    node: last_element,
                                    comment,
                                };
                            }
                        }
                    }
                }
                // Handles trailing comments after the last array element before the `]` token.
                // else if matches!(
                //     enclosing_node.kind(),
                //     JsSyntaxKind::JS_ARRAY_ELEMENT_LIST
                //         | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
                //         | JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST
                // ) {
                //     if let Some(last_element) = enclosing_node.last_child() {
                //         if last_element.kind() == JsSyntaxKind::JS_ARRAY_HOLE {
                //             return CommentPosition::Leading {
                //                 node: last_element,
                //                 comment,
                //             };
                //         }
                //     }
                // }
            }

            JsSyntaxKind::R_PAREN => {
                // Make line comments inside of an empty call arguments trailing comments of the call arguments
                // so that they get moved out of the parentheses.
                // ```javascript
                // expect( // remains a dangling comment
                //     // test
                // )
                // ```
                // becomes
                // ```javascript
                // expect(); // remains a dangling comment
                // // test
                // ```
                if let Some(arguments) = JsCallArguments::cast_ref(&enclosing_node) {
                    if arguments.r_paren_token().as_ref() == Ok(comment.following_token()) {
                        if arguments.args().is_empty() && comment.kind().is_line() {
                            return CommentPosition::Trailing {
                                node: arguments.into_syntax(),
                                comment,
                            };
                        }
                    }
                }
                // Makes the last comment in a non-empty call arguments list a trailing comment of the
                // last argument
                // ```javascript
                // f(a, /* test */) // make test a trailing comment of a
                // ```
                else if let Some(arguments_list) = JsCallArgumentList::cast_ref(&enclosing_node) {
                    if let Some(Ok(last_argument)) = arguments_list.last() {
                        return CommentPosition::Trailing {
                            node: last_argument.into_syntax(),
                            comment,
                        };
                    }
                }
            }
            _ => {
                // fall through
            }
        }

        CommentPosition::Default(comment)
    }
}

declare_node_union! {
    JsAnyArrayLike = JsArrayExpression
        | JsArrayAssignmentPattern
        | JsArrayBindingPattern
}

impl JsAnyArrayLike {
    fn r_brack_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyArrayLike::JsArrayExpression(expression) => expression.r_brack_token(),
            JsAnyArrayLike::JsArrayAssignmentPattern(assignment) => assignment.r_brack_token(),
            JsAnyArrayLike::JsArrayBindingPattern(binding) => binding.r_brack_token(),
        }
    }

    fn last_element(&self) -> Option<SyntaxResult<JsSyntaxNode>> {
        match self {
            JsAnyArrayLike::JsArrayExpression(array) => match array.elements().iter().last() {
                Some(Ok(element)) => Some(Ok(element.into_syntax())),
                Some(Err(error)) => Some(Err(error)),
                None => None,
            },
            JsAnyArrayLike::JsArrayAssignmentPattern(array) => match array.elements().iter().last()
            {
                Some(Ok(element)) => Some(Ok(element.into_syntax())),
                Some(Err(error)) => Some(Err(error)),
                None => None,
            },
            JsAnyArrayLike::JsArrayBindingPattern(array) => match array.elements().iter().last() {
                Some(Ok(element)) => Some(Ok(element.into_syntax())),
                Some(Err(error)) => Some(Err(error)),
                None => None,
            },
        }
    }
}
