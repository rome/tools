use crate::prelude::*;
use rome_formatter::write;
use rome_formatter::{CommentKind, CommentStyle, SourceComment};
use rome_js_syntax::suppression::{parse_suppression_comment, SuppressionCategory};
use rome_js_syntax::{JsLanguage, JsSyntaxKind};
use rome_rowan::{SyntaxTriviaPieceComments, TextLen};

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
            write!(f, [comment.piece()])
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

impl CommentStyle<JsLanguage> for JsCommentStyle {
    fn is_suppression(&self, text: &str) -> bool {
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

    fn is_group_start_token(&self, kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JsSyntaxKind::L_PAREN
                | JsSyntaxKind::L_BRACK
                | JsSyntaxKind::L_CURLY
                | JsSyntaxKind::DOLLAR_CURLY
        )
    }

    fn is_group_end_token(&self, kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JsSyntaxKind::R_BRACK
                | JsSyntaxKind::R_CURLY
                | JsSyntaxKind::R_PAREN
                | JsSyntaxKind::COMMA
                | JsSyntaxKind::SEMICOLON
                | JsSyntaxKind::DOT
                | JsSyntaxKind::EOF
        )
    }
}
