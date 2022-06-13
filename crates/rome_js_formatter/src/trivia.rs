use crate::prelude::*;
use rome_formatter::{format_args, write, CommentKind, CommentStyle};
use rome_js_syntax::{JsLanguage, JsSyntaxKind, JsSyntaxToken};
use rome_rowan::{Language, SyntaxTriviaPiece, SyntaxTriviaPieceComments};

/// Formats the leading trivia (comments, skipped token trivia) of a token
pub fn format_leading_trivia(
    token: &JsSyntaxToken,
    trim_mode: TriviaPrintMode,
) -> FormatLeadingTrivia {
    FormatLeadingTrivia { trim_mode, token }
}

/// Determines if the whitespace separating comment trivias
/// from their associated tokens should be printed or trimmed
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriviaPrintMode {
    Full,
    Trim,
}

impl Default for TriviaPrintMode {
    fn default() -> Self {
        Self::Full
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatLeadingTrivia<'a> {
    pub(crate) trim_mode: TriviaPrintMode,
    pub(crate) token: &'a JsSyntaxToken,
}

impl Format<JsFormatContext> for FormatLeadingTrivia<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let leading_comments = write_leading_trivia(
            self.token.leading_trivia().pieces(),
            self.token,
            LeadingTriviaOptions {
                trim_mode: self.trim_mode,
                ..Default::default()
            },
            f,
        )?;

        let last_trailing_comment_kind = f.state_mut().take_last_trailing_comment_kind();
        if needs_space_between_comments_and_token(
            &leading_comments,
            self.token.kind(),
            last_trailing_comment_kind,
        ) {
            comment(&space_token()).fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LeadingTriviaOptions {
    pub(crate) trim_mode: TriviaPrintMode,
}

pub(crate) fn write_leading_trivia<I>(
    pieces: I,
    token: &JsSyntaxToken,
    options: LeadingTriviaOptions,
    f: &mut JsFormatter,
) -> FormatResult<Vec<rome_formatter::Comment<JsLanguage>>>
where
    I: IntoIterator<Item = SyntaxTriviaPiece<JsLanguage>>,
{
    let mut lines_before = 0;
    let mut comments = Vec::new();
    let mut pieces = pieces.into_iter();

    while let Some(piece) = pieces.next() {
        if let Some(comment) = piece.as_comments() {
            comments.push(rome_formatter::Comment::leading(comment, lines_before));
            lines_before = 0;
        } else if piece.is_newline() {
            lines_before += 1;
        } else if piece.is_skipped() {
            // Special handling for tokens that have skipped trivia:
            //
            // ```
            // class {
            //   // comment
            //   @test(/* inner */) // trailing
            //   /* token leading */
            //   method() {}
            // }
            // ```
            // If `@test(/*inner)` are skipped trivia that are part of the `method` tokens leading trivia, then the
            // following code splits the trivia into for parts:
            // 1. The first skipped trivia's leading comments: Comments that come before the first skipped trivia `@`: The `// comment`
            // 2. Skipped trivia: All trivia pieces between the first and last skipped trivia: `@test(/* inner *)`. Gets formatted as verbatim
            // 3. Trailing comments of the last skipped token: All comments that are on the same line as the last skipped trivia. The `// trailing` comment
            // 4. The token's leading trivia: All comments that are not on the same line as the last skipped token trivia: `/* token leading */`

            // Format the 1. part, the skipped trivia's leading comments
            FormatLeadingComments {
                comments: &comments,
                trim_mode: TriviaPrintMode::Full,
                lines_before_token: lines_before,
            }
            .fmt(f)?;

            comments.clear();
            lines_before = 0;

            // Count the whitespace between the last skipped token trivia and the token
            let mut spaces = 0;
            // The range that covers from the first to the last skipped token trivia
            let mut skipped_trivia_range = piece.text_range();

            for piece in pieces {
                if piece.is_whitespace() {
                    spaces += 1;
                    continue;
                }

                spaces = 0;

                // If this is another skipped trivia, then extend the skipped range and
                // clear all accumulated comments because they are formatted as verbatim as part of the
                // skipped token trivia
                if piece.is_skipped() {
                    skipped_trivia_range = skipped_trivia_range.cover(piece.text_range());
                    comments.clear();
                    lines_before = 0;
                } else if let Some(comment) = piece.as_comments() {
                    comments.push(rome_formatter::Comment::leading(comment, lines_before));
                    lines_before = 0;
                } else if piece.is_newline() {
                    lines_before += 1;
                }
            }

            // Format the skipped token trivia range
            syntax_token_text_slice(token, skipped_trivia_range).fmt(f)?;

            // Find the start position of the next token's leading comments.
            // The start is the first comment that is preceded by a line break.
            let first_token_leading_comment = comments
                .iter()
                .position(|comment| comment.lines_before() > 0)
                .unwrap_or(comments.len());

            // Everything before the start position are trailing comments of the last skipped token
            let token_leading_comments = comments.split_off(first_token_leading_comment);
            let skipped_trailing_comments = comments;

            // Format the trailing comments of the last skipped token trivia
            FormatTrailingTrivia::new(skipped_trailing_comments.into_iter(), None).fmt(f)?;

            // Ensure that there's some whitespace between the last skipped token trivia and the
            // next token except if there was no whitespace present in the source.
            if lines_before > 0 {
                write!(f, [hard_line_break()])?;
            } else if spaces > 0 {
                write!(f, [space_token()])?;
            };

            // Write  leading comments of the next token
            FormatLeadingComments {
                comments: &token_leading_comments,
                lines_before_token: lines_before,
                trim_mode: options.trim_mode,
            }
            .fmt(f)?;

            return Ok(token_leading_comments);
        }
    }

    FormatLeadingComments {
        comments: &comments,
        trim_mode: options.trim_mode,
        lines_before_token: lines_before,
    }
    .fmt(f)?;

    Ok(comments)
}

struct FormatLeadingComments<'a> {
    comments: &'a [rome_formatter::Comment<JsLanguage>],
    trim_mode: TriviaPrintMode,
    lines_before_token: u32,
}

impl Format<JsFormatContext> for FormatLeadingComments<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        for (index, comment) in self.comments.iter().enumerate() {
            let is_line_comment = JsCommentStyle.get_comment_kind(comment.piece()).is_line();
            let lines_after = self
                .comments
                .get(index + 1)
                .map(|comment| comment.lines_before())
                .unwrap_or_else(|| match self.trim_mode {
                    TriviaPrintMode::Full => self.lines_before_token,
                    TriviaPrintMode::Trim => 0,
                });

            let format_content = format_with(|f| {
                if comment.lines_before() > 0 && index == 0 {
                    write!(f, [hard_line_break()])?;
                } else {
                    write!(f, [space_token()])?;
                };

                write!(f, [comment.piece()])?;

                if is_line_comment {
                    match lines_after {
                        0 | 1 => write!(f, [hard_line_break()])?,
                        _ => write!(f, [empty_line()])?,
                    }
                } else {
                    match lines_after {
                        0 => {
                            // space between last comment and token handled at the end.
                            // space between comments is inserted before each comment
                        }
                        1 => write!(f, [hard_line_break()])?,
                        _ => write!(f, [empty_line()])?,
                    }
                };

                Ok(())
            });

            write!(f, [rome_formatter::comment(&format_content)])?;
        }

        Ok(())
    }
}

pub(crate) fn needs_space_between_comments_and_token(
    comments: &[rome_formatter::Comment<JsLanguage>],
    token: JsSyntaxKind,
    last_trailing_kind: Option<CommentKind>,
) -> bool {
    let last_comment_kind = comments
        .last()
        .map(|comment| JsCommentStyle.get_comment_kind(comment.piece()))
        .or(last_trailing_kind);

    if let Some(last) = last_comment_kind {
        // Line comments always insert a trailing line break
        if last.is_line() {
            false
        } else {
            match token {
                // Don't insert a space if the current token is a group end token
                JsSyntaxKind::R_BRACK
                | JsSyntaxKind::R_CURLY
                | JsSyntaxKind::R_PAREN
                | JsSyntaxKind::COMMA
                | JsSyntaxKind::SEMICOLON
                | JsSyntaxKind::DOT
                | JsSyntaxKind::EOF => false,
                _ => true,
            }
        }
    } else {
        false
    }
}

/// Formats the trailing trivia (comments) of a token
pub fn format_trailing_trivia(token: &JsSyntaxToken) -> impl Format<JsFormatContext> {
    let comments = token
        .trailing_trivia()
        .pieces()
        .filter_map(|piece| piece.as_comments().map(rome_formatter::Comment::trailing));
    FormatTrailingTrivia::new(comments, Some(token.kind()))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FormatTrailingTrivia<I> {
    comments: I,
    token_kind: Option<JsSyntaxKind>,
}

impl<I> FormatTrailingTrivia<I> {
    pub fn new(comments: I, token_kind: Option<JsSyntaxKind>) -> Self {
        Self {
            comments,
            token_kind,
        }
    }
}

impl<I> Format<JsFormatContext> for FormatTrailingTrivia<I>
where
    I: Iterator<Item = rome_formatter::Comment<JsLanguage>> + Clone,
{
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let comments = self.comments.clone();
        let mut last_kind = None;

        for (index, comment) in comments.enumerate() {
            let kind = JsCommentStyle.get_comment_kind(comment.piece());
            last_kind = Some(kind);
            let is_single_line = kind.is_line();

            let content = format_with(|f| {
                if !is_single_line {
                    match self.token_kind {
                        Some(token) if JsCommentStyle.is_group_start_token(token) && index == 0 => {
                            (
                            // Don't write a space if this is a group start token and it isn't the first trailing comment
                            )
                        }
                        //  Write a space for all other cases
                        _ => space_token().fmt(f)?,
                    }
                    comment.piece().fmt(f)?;
                } else {
                    write![
                        f,
                        [
                            line_suffix(&format_args![space_token(), comment.piece()]),
                            expand_parent()
                        ]
                    ]?;
                }

                Ok(())
            });

            rome_formatter::comment(&content).fmt(f)?;
        }

        f.state_mut().set_last_trailing_comment(last_kind);

        Ok(())
    }
}

pub struct JsCommentStyle;

impl CommentStyle for JsCommentStyle {
    type Language = JsLanguage;

    fn get_comment_kind(&self, comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
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

    fn is_group_start_token(&self, kind: <Self::Language as Language>::Kind) -> bool {
        matches!(
            kind,
            JsSyntaxKind::L_PAREN | JsSyntaxKind::L_BRACK | JsSyntaxKind::L_CURLY
        )
    }

    fn is_group_end_token(&self, kind: <Self::Language as Language>::Kind) -> bool {
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
