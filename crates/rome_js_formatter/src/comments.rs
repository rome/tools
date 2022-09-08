use crate::prelude::*;
use rome_formatter::{write, CommentPlacement, CommentPosition, Comments, DecoratedComment};
use rome_formatter::{CommentKind, CommentStyle, SourceComment};
use rome_js_syntax::suppression::{parse_suppression_comment, SuppressionCategory};
use rome_js_syntax::{
    JsAnyRoot, JsAnyStatement, JsArrayAssignmentPattern, JsArrayBindingPattern, JsArrayExpression,
    JsArrayHole, JsBlockStatement, JsIfStatement, JsLanguage, JsSyntaxKind, JsSyntaxNode,
    JsSyntaxToken,
};
use rome_rowan::{
    declare_node_union, AstNode, SyntaxNode, SyntaxResult, SyntaxTriviaPieceComments, TextLen,
};

pub type JsComments = Comments<JsLanguage>;

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

    fn get_comment_kind(&self, comment: &SyntaxTriviaPieceComments<JsLanguage>) -> CommentKind {
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

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        fn place_comment(
            mut comment: DecoratedComment<JsLanguage>,
            rules: &[fn(
                DecoratedComment<JsLanguage>,
            )
                -> Result<CommentPlacement<JsLanguage>, DecoratedComment<JsLanguage>>],
        ) -> CommentPlacement<JsLanguage> {
            for rule in rules {
                match rule(comment) {
                    Ok(placement) => return placement,
                    Err(unplaced) => comment = unplaced,
                }
            }

            CommentPlacement::Default(comment)
        }

        match comment.position() {
            CommentPosition::EndOfLine => place_comment(
                comment,
                &[
                    handle_if_statement_comment,
                    handle_typecast_comment,
                    // handle_block_statement_comment,
                    handle_root_comments,
                    handle_array_hole_comment,
                ],
            ),
            CommentPosition::OwnLine => place_comment(
                comment,
                &[
                    handle_if_statement_comment,
                    // handle_block_statement_comment,
                    handle_root_comments,
                    handle_array_hole_comment,
                ],
            ),
            CommentPosition::SameLine => place_comment(
                comment,
                &[
                    handle_if_statement_comment,
                    // handle_block_statement_comment,
                    handle_root_comments,
                    handle_array_hole_comment,
                ],
            ),
        }
    }
}

/// Force end of line type cast comments to remain leading comments of the next node, if any
fn handle_typecast_comment(
    comment: DecoratedComment<JsLanguage>,
) -> Result<CommentPlacement<JsLanguage>, DecoratedComment<JsLanguage>> {
    match comment.following_node() {
        Some(following_node) if is_type_comment(comment.piece()) => Ok(CommentPlacement::Leading {
            node: following_node.clone(),
            comment,
        }),
        _ => Err(comment),
    }
}

/// Move leading comments in front of the `{` inside of the block
///
/// ```javascript
/// if (test) /* comment */ {
///  console.log('test');
/// }
/// ```
///
/// becomes
/// ```javascript
/// if (test) {
///  /* comment */ console.log('test');
/// }
/// ```
fn handle_block_statement_comment(
    comment: DecoratedComment<JsLanguage>,
) -> Result<CommentPlacement<JsLanguage>, DecoratedComment<JsLanguage>> {
    if let Some(block) = comment
        .following_node()
        .and_then(JsBlockStatement::cast_ref)
    {
        Ok(match block.statements().first() {
            Some(JsAnyStatement::JsEmptyStatement(_)) | None => CommentPlacement::Dangling {
                node: block.into_syntax(),
                comment,
            },
            Some(first_statement) => CommentPlacement::Leading {
                node: first_statement.into_syntax(),
                comment,
            },
        })
    } else {
        Err(comment)
    }
}

/// Handles array hole comments. Array holes have no token so all comments
/// become trailing comments by default. Override it that all comments are leading comments.
fn handle_array_hole_comment(
    comment: DecoratedComment<JsLanguage>,
) -> Result<CommentPlacement<JsLanguage>, DecoratedComment<JsLanguage>> {
    if let Some(array_hole) = comment.preceding_node().and_then(JsArrayHole::cast_ref) {
        Ok(CommentPlacement::Leading {
            node: array_hole.into_syntax(),
            comment,
        })
    } else {
        Err(comment)
    }
}

/// Handle a all comments document.
/// See `blank.js`
fn handle_root_comments(
    comment: DecoratedComment<JsLanguage>,
) -> Result<CommentPlacement<JsLanguage>, DecoratedComment<JsLanguage>> {
    if let Some(root) = JsAnyRoot::cast_ref(comment.enclosing_node()) {
        let is_blank = match &root {
            JsAnyRoot::JsExpressionSnipped(_) => false,
            JsAnyRoot::JsModule(module) => {
                module.directives().is_empty() && module.items().is_empty()
            }
            JsAnyRoot::JsScript(script) => {
                script.directives().is_empty() && script.statements().is_empty()
            }
        };

        if is_blank {
            return Ok(CommentPlacement::Leading {
                node: root.into_syntax(),
                comment,
            });
        }
    }

    Err(comment)
}

fn handle_if_statement_comment(
    comment: DecoratedComment<JsLanguage>,
) -> Result<CommentPlacement<JsLanguage>, DecoratedComment<JsLanguage>> {
    fn handle_else_clause(
        comment: DecoratedComment<JsLanguage>,
        consequent: JsSyntaxNode,
        if_statement: JsSyntaxNode,
    ) -> CommentPlacement<JsLanguage> {
        // Make all comments trailing comments of the `consequent` if the `consequent` is a `JsBlockStatement`
        // ```javascript
        // if (test) {
        //
        // } /* comment */ else if (b) {
        //     test
        // }
        // /* comment */ else if(c) {
        //
        // } /*comment */ else {
        //
        // }
        // ```
        if consequent.kind() == JsSyntaxKind::JS_BLOCK_STATEMENT {
            return CommentPlacement::Trailing {
                node: consequent,
                comment,
            };
        }

        // Handle end of line comments that aren't stretching over multiple lines.
        // Make them dangling comments of the consequent expression
        //
        // ```javascript
        // if (cond1) expr1; // comment A
        // else if (cond2) expr2; // comment A
        // else expr3;
        //
        // if (cond1) expr1; /* comment */ else  expr2;
        //
        // if (cond1) expr1; /* b */
        // else if (cond2) expr2; /* b */
        // else expr3; /* b*/
        // ```
        if !comment.kind().is_block() && !comment.position().is_own_line() {
            return CommentPlacement::Dangling {
                node: if_statement,
                comment,
            };
        }

        // ```javascript
        // if (cond1) expr1;
        //
        // /* comment */ else  expr2;
        //
        // if (cond) expr; /*
        // a multiline comment */
        // else b;
        // ```
        CommentPlacement::Dangling {
            node: if_statement,
            comment,
        }
    };

    match (comment.enclosing_node().kind(), comment.following_node()) {
        (JsSyntaxKind::JS_IF_STATEMENT, Some(following)) => {
            let if_statement = JsIfStatement::unwrap_cast(comment.enclosing_node().clone());

            if let Some(preceding) = comment.preceding_node() {
                // Test if this is a comment right before the condition's `)`
                if comment.following_token().kind() == JsSyntaxKind::R_PAREN {
                    return Ok(CommentPlacement::Trailing {
                        node: preceding.clone(),
                        comment,
                    });
                }

                // Handle comments before `else`
                if following.kind() == JsSyntaxKind::JS_ELSE_CLAUSE {
                    let consequent = preceding.clone();
                    let if_statement = comment.enclosing_node().clone();
                    return Ok(handle_else_clause(comment, consequent, if_statement));
                }
            }

            // Move comments coming before the `{` inside of the block
            //
            // ```javascript
            // if (cond) /* test */ {
            // }
            // ```
            if let Some(block_statement) = JsBlockStatement::cast_ref(following) {
                return Ok(place_block_statement_comment(block_statement, comment));
            }

            // Move comments coming before an if chain inside the body of the first non chain if.
            //
            // ```javascript
            // if (cond1)  /* test */ if (other) { a }
            // ```
            if let Some(if_statement) = JsIfStatement::cast_ref(following) {
                if let Ok(nested_consequent) = if_statement.consequent() {
                    return Ok(place_leading_statement_comment(nested_consequent, comment));
                }
            }

            // Make all comments after the condition's `)` leading comments
            // ```javascript
            // if (5) // comment
            // true
            //
            // ```
            if (if_statement.consequent().map(AstNode::into_syntax).as_ref()) == Ok(following) {
                return Ok(CommentPlacement::Leading {
                    node: following.clone(),
                    comment,
                });
            }
        }
        (JsSyntaxKind::JS_ELSE_CLAUSE, _) => {
            if let Some(if_statement) = comment
                .enclosing_node()
                .parent()
                .and_then(JsIfStatement::cast)
            {
                if let Ok(consequent) = if_statement.consequent() {
                    return Ok(handle_else_clause(
                        comment,
                        consequent.into_syntax(),
                        if_statement.into_syntax(),
                    ));
                }
            }
        }
        _ => {
            // fall through
        }
    }

    Err(comment)
}

fn place_leading_statement_comment(
    statement: JsAnyStatement,
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    match statement {
        JsAnyStatement::JsBlockStatement(block) => place_block_statement_comment(block, comment),
        statement => CommentPlacement::Leading {
            node: statement.into_syntax(),
            comment,
        },
    }
}

fn place_block_statement_comment(
    block_statement: JsBlockStatement,
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    match block_statement.statements().first() {
        Some(JsAnyStatement::JsEmptyStatement(_)) | None => CommentPlacement::Dangling {
            node: block_statement.into_syntax(),
            comment,
        },
        Some(statement) => CommentPlacement::Leading {
            node: statement.into_syntax(),
            comment,
        },
    }
}

/// Returns `true` if `comment` is a [Closure type comment](https://github.com/google/closure-compiler/wiki/Types-in-the-Closure-Type-System)
/// or [TypeScript type comment](https://www.typescriptlang.org/docs/handbook/jsdoc-supported-types.html#type)
pub(crate) fn is_type_comment(comment: &SyntaxTriviaPieceComments<JsLanguage>) -> bool {
    let text = comment.text();

    // Must be a `/**` comment
    if !text.starts_with("/**") {
        return false;
    }

    text.trim_start_matches("/**")
        .trim_end_matches("*/")
        .split_whitespace()
        .any(|word| match word.strip_prefix("@type") {
            Some(after) => after.is_empty() || after.starts_with('{'),
            None => false,
        })
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
