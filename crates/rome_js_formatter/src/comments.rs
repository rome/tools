use crate::prelude::*;
use rome_formatter::{write, CommentPlacement, CommentPosition, Comments, DecoratedComment};
use rome_formatter::{CommentKind, CommentStyle, SourceComment};
use rome_js_syntax::suppression::{parse_suppression_comment, SuppressionCategory};
use rome_js_syntax::{
    JsAnyName, JsAnyRoot, JsAnyStatement, JsArrayHole, JsBlockStatement, JsCatchClause,
    JsFinallyClause, JsIdentifierExpression, JsIfStatement, JsLanguage, JsSyntaxKind, JsSyntaxNode,
    JsVariableDeclarator, JsWhileStatement,
};
use rome_rowan::{AstNode, SyntaxTriviaPieceComments, TextLen};

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
        match comment.position() {
            CommentPosition::EndOfLine => handle_typecast_comment(comment)
                .or_else(handle_if_statement_comment)
                .or_else(handle_while_comment)
                .or_else(handle_try_comment)
                .or_else(handle_root_comments)
                .or_else(handle_array_hole_comment)
                .or_else(handle_variable_declarator),
            CommentPosition::OwnLine => handle_member_expression_comment(comment)
                .or_else(handle_if_statement_comment)
                .or_else(handle_while_comment)
                .or_else(handle_try_comment)
                .or_else(handle_root_comments)
                .or_else(handle_array_hole_comment),
            CommentPosition::SameLine => handle_if_statement_comment(comment)
                .or_else(handle_while_comment)
                .or_else(handle_root_comments)
                .or_else(handle_array_hole_comment),
        }
    }
}

/// Force end of line type cast comments to remain leading comments of the next node, if any
fn handle_typecast_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    match comment.following_node() {
        Some(following_node) if is_type_comment(comment.piece()) => CommentPlacement::Leading {
            node: following_node.clone(),
            comment,
        },
        _ => CommentPlacement::Default(comment),
    }
}

/// Handles array hole comments. Array holes have no token so all comments
/// become trailing comments by default. Override it that all comments are leading comments.
fn handle_array_hole_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    if let Some(array_hole) = comment.preceding_node().and_then(JsArrayHole::cast_ref) {
        CommentPlacement::Leading {
            node: array_hole.into_syntax(),
            comment,
        }
    } else {
        CommentPlacement::Default(comment)
    }
}

/// Handle a all comments document.
/// See `blank.js`
fn handle_root_comments(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
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
            return CommentPlacement::Leading {
                node: root.into_syntax(),
                comment,
            };
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_member_expression_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    let following = match comment.following_node() {
        Some(following)
            if matches!(
                comment.enclosing_node().kind(),
                JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                    | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
            ) =>
        {
            following
        }
        _ => return CommentPlacement::Default(comment),
    };

    // ```javascript
    // a
    // /* comment */.b
    // a
    // /* comment */[b]
    // ```
    if JsAnyName::can_cast(following.kind()) || JsIdentifierExpression::can_cast(following.kind()) {
        CommentPlacement::Leading {
            node: comment.enclosing_node().clone(),
            comment,
        }
    } else {
        CommentPlacement::Default(comment)
    }
}

fn handle_if_statement_comment(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
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
    }

    match (comment.enclosing_node().kind(), comment.following_node()) {
        (JsSyntaxKind::JS_IF_STATEMENT, Some(following)) => {
            let if_statement = JsIfStatement::unwrap_cast(comment.enclosing_node().clone());

            if let Some(preceding) = comment.preceding_node() {
                // Test if this is a comment right before the condition's `)`
                if comment.following_token().kind() == JsSyntaxKind::R_PAREN {
                    return CommentPlacement::Trailing {
                        node: preceding.clone(),
                        comment,
                    };
                }

                // Handle comments before `else`
                if following.kind() == JsSyntaxKind::JS_ELSE_CLAUSE {
                    let consequent = preceding.clone();
                    let if_statement = comment.enclosing_node().clone();
                    return handle_else_clause(comment, consequent, if_statement);
                }
            }

            // Move comments coming before the `{` inside of the block
            //
            // ```javascript
            // if (cond) /* test */ {
            // }
            // ```
            if let Some(block_statement) = JsBlockStatement::cast_ref(following) {
                return place_block_statement_comment(block_statement, comment);
            }

            // Move comments coming before an if chain inside the body of the first non chain if.
            //
            // ```javascript
            // if (cond1)  /* test */ if (other) { a }
            // ```
            if let Some(if_statement) = JsIfStatement::cast_ref(following) {
                if let Ok(nested_consequent) = if_statement.consequent() {
                    return place_leading_statement_comment(nested_consequent, comment);
                }
            }

            // Make all comments after the condition's `)` leading comments
            // ```javascript
            // if (5) // comment
            // true
            //
            // ```
            if let Ok(consequent) = if_statement.consequent() {
                if consequent.syntax() == following {
                    return CommentPlacement::Leading {
                        node: following.clone(),
                        comment,
                    };
                }
            }
        }
        (JsSyntaxKind::JS_ELSE_CLAUSE, _) => {
            if let Some(if_statement) = comment
                .enclosing_node()
                .parent()
                .and_then(JsIfStatement::cast)
            {
                if let Ok(consequent) = if_statement.consequent() {
                    return handle_else_clause(
                        comment,
                        consequent.into_syntax(),
                        if_statement.into_syntax(),
                    );
                }
            }
        }
        _ => {
            // fall through
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_while_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    let (while_statement, following) = match (
        JsWhileStatement::cast_ref(comment.enclosing_node()),
        comment.following_node(),
    ) {
        (Some(while_statement), Some(following)) => (while_statement, following),
        _ => return CommentPlacement::Default(comment),
    };

    if let Some(preceding) = comment.preceding_node() {
        // Test if this is a comment right before the condition's `)`
        if comment.following_token().kind() == JsSyntaxKind::R_PAREN {
            return CommentPlacement::Trailing {
                node: preceding.clone(),
                comment,
            };
        }
    }

    // Move comments coming before the `{` inside of the block
    //
    // ```javascript
    // while (cond) /* test */ {
    // }
    // ```
    if let Some(block) = JsBlockStatement::cast_ref(following) {
        return place_block_statement_comment(block, comment);
    }

    // Make all comments after the condition's `)` leading comments
    // ```javascript
    // while (5) // comment
    // true
    //
    // ```
    if let Ok(body) = while_statement.body() {
        if body.syntax() == following {
            return CommentPlacement::Leading {
                node: body.into_syntax(),
                comment,
            };
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_try_comment(comment: DecoratedComment<JsLanguage>) -> CommentPlacement<JsLanguage> {
    let following = match comment.following_node() {
        Some(following)
            if matches!(
                comment.enclosing_node().kind(),
                JsSyntaxKind::JS_TRY_STATEMENT | JsSyntaxKind::JS_TRY_FINALLY_STATEMENT
            ) =>
        {
            // Move comments before the `catch` or `finally` inside of the body
            // ```javascript
            // try {
            // }
            //  catch(e) {
            // }
            // // Comment 7
            // finally {}
            // ```
            let body = if let Some(catch) = JsCatchClause::cast_ref(following) {
                catch.body()
            } else if let Some(finally) = JsFinallyClause::cast_ref(following) {
                finally.body()
            } else {
                // Use an err, so that the following code skips over it
                Err(rome_rowan::SyntaxError::MissingRequiredChild)
            };

            //
            // ```javascript
            // try {
            // } /* comment catch {
            // }
            // ```
            if let Ok(body) = body {
                return place_block_statement_comment(body, comment);
            }

            following
        }
        Some(following)
            if matches!(
                comment.enclosing_node().kind(),
                JsSyntaxKind::JS_CATCH_CLAUSE | JsSyntaxKind::JS_FINALLY_CLAUSE
            ) =>
        {
            following
        }
        _ => return CommentPlacement::Default(comment),
    };

    // Move comments coming before the `{` inside of the block
    //
    // ```javascript
    // try /* test */ {
    // }
    // ```
    if let Some(block) = JsBlockStatement::cast_ref(following) {
        return place_block_statement_comment(block, comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_variable_declarator(
    comment: DecoratedComment<JsLanguage>,
) -> CommentPlacement<JsLanguage> {
    let following = match comment.following_node() {
        Some(following) => following,
        None => return CommentPlacement::Default(comment),
    };

    fn is_complex_value(value: &JsSyntaxNode) -> bool {
        matches!(
            value.kind(),
            JsSyntaxKind::JS_OBJECT_EXPRESSION
                | JsSyntaxKind::JS_ARRAY_EXPRESSION
                | JsSyntaxKind::JS_TEMPLATE
                | JsSyntaxKind::TS_OBJECT_TYPE
        )
    }

    let enclosing = comment.enclosing_node();
    match enclosing.kind() {
        JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION | JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => {
            // Makes all comments preceding objects/arrays/templates leading comments of these nodes.
            // ```javascript
            // let a = // comment
            // { };
            // ```
            if is_complex_value(following) {
                return CommentPlacement::Leading {
                    node: following.clone(),
                    comment,
                };
            }
        }
        JsSyntaxKind::JS_VARIABLE_DECLARATOR => {
            let variable_declarator = JsVariableDeclarator::unwrap_cast(enclosing.clone());

            match variable_declarator.initializer() {
                // ```javascript
                // let obj2 // Comment
                // = {
                //   key: 'val'
                // }
                // ```
                Some(initializer) if initializer.syntax() == following => {
                    if let Ok(expression) = initializer.expression() {
                        return CommentPlacement::Leading {
                            node: expression.into_syntax(),
                            comment,
                        };
                    }
                }
                _ => {
                    // fall through
                }
            }
        }
        JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
            if let Some(variable_declarator) =
                enclosing.parent().and_then(JsVariableDeclarator::cast)
            {
                // Keep trailing comments with the id for variable declarators. Necessary because the value is wrapped
                // inside of an initializer clause.
                // ```javascript
                // let a = // comment
                //      b;
                // ```
                if !is_complex_value(following) {
                    if let Ok(id) = variable_declarator.id() {
                        return CommentPlacement::Trailing {
                            node: id.into_syntax(),
                            comment,
                        };
                    }
                }
            }
        }
        _ => {
            // fall through
        }
    }

    CommentPlacement::Default(comment)
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
