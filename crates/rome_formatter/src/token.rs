use crate::comments::CommentStyle;
use crate::prelude::*;
use crate::{
    format_args, write, Argument, Arguments, CommentKind, CstFormatContext, FormatRefWithRule,
    GroupId, LastTokenKind, SourceComment,
};
use rome_rowan::{Language, SyntaxToken, SyntaxTriviaPiece};

///! Provides builders for working with tokens and the tokens trivia

/// Formats a token without its leading or trailing trivia
///
/// ## Warning
/// It's your responsibility to format leading or trailing comments and skipped trivia.
pub const fn format_trimmed_token<L: Language>(token: &SyntaxToken<L>) -> FormatTrimmedToken<L> {
    FormatTrimmedToken { token }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct FormatTrimmedToken<'a, L: Language> {
    token: &'a SyntaxToken<L>,
}

impl<L: Language + 'static, C> Format<C> for FormatTrimmedToken<'_, L>
where
    C: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        write_space_between_comment_and_token(self.token.kind(), f)?;

        f.state_mut().set_last_token_kind(self.token.kind());

        let trimmed_range = self.token.text_trimmed_range();
        syntax_token_text_slice(self.token, trimmed_range).fmt(f)
    }
}

/// Formats a token that has been inserted by the formatter and isn't present in the source text.
/// Takes care of correctly handling spacing to the previous token's trailing trivia.
pub struct FormatInserted<L>
where
    L: Language,
{
    kind: L::Kind,
    text: &'static str,
}

impl<L> FormatInserted<L>
where
    L: Language,
{
    pub fn new(kind: L::Kind, text: &'static str) -> Self {
        Self { kind, text }
    }
}

impl<L, C> Format<C> for FormatInserted<L>
where
    L: Language + 'static,
    C: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        write_space_between_comment_and_token(self.kind, f)?;

        f.state_mut().set_last_token_kind(self.kind);
        text(self.text).fmt(f)
    }
}

fn write_space_between_comment_and_token<L: Language, Context>(
    token_kind: <L as Language>::Kind,
    f: &mut Formatter<Context>,
) -> FormatResult<()>
where
    Context: CstFormatContext<Language = L>,
{
    let is_last_content_inline_content = f.state().is_last_content_inline_comment();

    // Insert a space if the previous token has any trailing comments and this is not a group
    // end token
    #[allow(deprecated)]
    if is_last_content_inline_content && !f.context().comment_style().is_group_end_token(token_kind)
    {
        space().fmt(f)?;
    }

    f.state_mut().set_last_content_inline_comment(false);

    Ok(())
}

/// Inserts a open parentheses before the specified token and ensures
/// that any leading trivia of the token is formatted **before** the inserted parentheses.
///
/// # Example
///
/// ```javascript
/// /* leading */ "string";
/// ```
///
/// Becomes
///
/// ```javascript
/// /* leading */ ("string";
/// ```
///
/// when inserting the "(" before the "string" token.
#[derive(Clone, Debug)]
pub struct FormatInsertedOpenParen<'a, L>
where
    L: Language,
{
    /// The token before which the open paren must be inserted
    before_token: Option<&'a SyntaxToken<L>>,

    /// The token text of the open paren
    text: &'static str,

    /// The kind of the open paren
    kind: L::Kind,
}

impl<'a, L> FormatInsertedOpenParen<'a, L>
where
    L: Language,
{
    pub fn new(
        before_token: Option<&'a SyntaxToken<L>>,
        kind: L::Kind,
        text: &'static str,
    ) -> Self {
        Self {
            before_token,
            kind,
            text,
        }
    }
}

impl<Context, L> Format<Context> for FormatInsertedOpenParen<'_, L>
where
    L: Language + 'static,
    Context: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let mut comments = Vec::new();

        if let Some(before_token) = &self.before_token {
            // Format the leading trivia of the next token as the leading trivia of the open paren.
            let leading_pieces = before_token.leading_trivia().pieces();

            let mut lines_before = 0;

            for piece in leading_pieces {
                if let Some(comment) = piece.as_comments() {
                    comments.push(SourceComment::leading(comment, lines_before));
                    lines_before = 0;
                } else if piece.is_newline() {
                    lines_before += 1;
                } else if piece.is_skipped() {
                    // Keep the skipped trivia inside of the parens. Handled by the
                    // formatting of the `before_token`.
                    break;
                }
            }

            write!(
                f,
                [FormatLeadingComments {
                    comments: &comments,
                    trim_mode: TriviaPrintMode::Full,
                    lines_before_token: lines_before,
                }]
            )?;
        }

        write!(
            f,
            [FormatInserted {
                kind: self.kind,
                text: self.text,
            }]
        )?;

        // Prevent that the comments get formatted again when formatting the
        // `before_token`
        for comment in comments {
            f.state_mut().mark_comment_as_formatted(comment.piece());
        }

        Ok(())
    }
}

/// Inserts a closing parentheses before another token and moves that tokens
/// trailing trivia after the closing parentheses.
///
/// # Example
///
/// ```javascript
/// "string" /* trailing */;
/// ```
///
/// Becomes
///
/// ```javascript
/// "string") /* trailing */
/// ```
#[derive(Clone, Debug)]
pub struct FormatInsertedCloseParen<L>
where
    L: Language,
{
    /// The token after which the close paren must be inserted
    comments: Vec<SourceComment<L>>,

    /// The token text of the close paren
    text: &'static str,

    /// The kind of the close paren
    kind: L::Kind,
}

impl<L> FormatInsertedCloseParen<L>
where
    L: Language,
{
    pub fn after_token<Context>(
        after_token: Option<&SyntaxToken<L>>,
        kind: L::Kind,
        text: &'static str,
        f: &mut Formatter<Context>,
    ) -> Self {
        let mut comments = Vec::new();

        if let Some(after_token) = after_token {
            // "Steal" the trailing comments and mark them as handled.
            // Must be done eagerly before formatting because the `after_token`
            // gets formatted **before** formatting the inserted paren.
            for piece in after_token.trailing_trivia().pieces() {
                if let Some(comment) = piece.as_comments() {
                    f.state_mut().mark_comment_as_formatted(&comment);
                    comments.push(SourceComment::trailing(comment));
                }
            }
        }

        Self {
            comments,
            kind,
            text,
        }
    }
}

impl<Context, L> Format<Context> for FormatInsertedCloseParen<L>
where
    L: Language + 'static,
    Context: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        write!(
            f,
            [
                FormatInserted {
                    kind: self.kind,
                    text: self.text,
                },
                FormatTrailingTrivia::new(self.comments.iter().cloned(), self.kind,)
                    .skip_formatted_check()
            ]
        )
    }
}

/// Formats the leading and trailing trivia of a removed token.
///
/// Formats all leading and trailing comments up to the first line break or skipped token trivia as a trailing
/// comment of the previous token. The remaining trivia is then printed as leading trivia of the next token.
pub const fn format_removed<L>(token: &SyntaxToken<L>) -> FormatRemoved<L>
where
    L: Language,
{
    FormatRemoved { token }
}

/// Formats the trivia of a token that is present in the source text but should be omitted in the
/// formatted output.
pub struct FormatRemoved<'a, L>
where
    L: Language,
{
    token: &'a SyntaxToken<L>,
}

impl<C, L> Format<C> for FormatRemoved<'_, L>
where
    L: Language + 'static,
    C: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        f.state_mut().track_token(self.token);

        let last = f.state().last_token_kind();

        write_removed_token_trivia(self.token, last, f)
    }
}

/// Writes the trivia of a removed token
fn write_removed_token_trivia<C, L>(
    token: &SyntaxToken<L>,
    last_token: Option<LastTokenKind>,
    f: &mut Formatter<C>,
) -> FormatResult<()>
where
    L: Language + 'static,
    C: CstFormatContext<Language = L>,
{
    let mut pieces = token
        .leading_trivia()
        .pieces()
        .chain(token.trailing_trivia().pieces())
        .peekable();

    let last_token = last_token.and_then(|token| token.as_language::<L>());

    // If this isn't the first token than format all comments that are before the first skipped token
    // trivia or line break as the trailing trivia of the previous token (which these comments will
    // become if the document gets formatted a second time).
    if let Some(last_token) = last_token {
        let mut trailing_comments = vec![];

        while let Some(piece) = pieces.peek() {
            if let Some(comment) = piece.as_comments() {
                if !f.state().is_comment_formatted(&comment) {
                    trailing_comments.push(SourceComment::trailing(comment));
                }
            } else if piece.is_newline() || piece.is_skipped() {
                break;
            }

            pieces.next();
        }

        FormatTrailingTrivia::new(trailing_comments.into_iter(), last_token).fmt(f)?;
    }

    write_leading_trivia(pieces, token, TriviaPrintMode::Full, f)?;

    Ok(())
}

/// Print out a `token` from the original source with a different `content`.
///
/// This will print the trivia that belong to `token` to `content`;
/// `token` is then marked as consumed by the formatter.
pub fn format_replaced<'a, 'content, L, Context>(
    token: &'a SyntaxToken<L>,
    content: &'content impl Format<Context>,
) -> FormatReplaced<'a, 'content, L, Context>
where
    L: Language,
{
    FormatReplaced {
        token,
        content: Argument::new(content),
    }
}

/// Formats a token's leading and trailing trivia but uses the provided content instead
/// of the token in the formatted output.
#[derive(Copy, Clone)]
pub struct FormatReplaced<'a, 'content, L, C>
where
    L: Language,
{
    token: &'a SyntaxToken<L>,
    content: Argument<'content, C>,
}

impl<L, C> Format<C> for FormatReplaced<'_, '_, L, C>
where
    L: Language + 'static,
    C: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        f.state_mut().track_token(self.token);

        format_leading_trivia(self.token).fmt(f)?;

        write_space_between_comment_and_token(self.token.kind(), f)?;

        f.state_mut().set_last_token_kind(self.token.kind());

        f.write_fmt(Arguments::from(&self.content))?;
        format_trailing_trivia(self.token).fmt(f)
    }
}

/// Formats the given token only if the group does break and otherwise retains the token's trivia.
pub fn format_only_if_breaks<'a, 'content, L, Content, Context>(
    token: &'a SyntaxToken<L>,
    content: &'content Content,
) -> FormatOnlyIfBreaks<'a, 'content, L, Context>
where
    L: Language,
    Content: Format<Context>,
{
    FormatOnlyIfBreaks {
        token,
        content: Argument::new(content),
        group_id: None,
    }
}

/// Formats a token with its leading and trailing trivia that only gets printed if its enclosing
/// group does break but otherwise gets omitted from the formatted output.
pub struct FormatOnlyIfBreaks<'a, 'content, L, C>
where
    L: Language,
{
    token: &'a SyntaxToken<L>,
    content: Argument<'content, C>,
    group_id: Option<GroupId>,
}

impl<'a, 'content, L, C> FormatOnlyIfBreaks<'a, 'content, L, C>
where
    L: Language,
{
    pub fn with_group_id(mut self, group_id: Option<GroupId>) -> Self {
        self.group_id = group_id;
        self
    }
}

impl<L, C> Format<C> for FormatOnlyIfBreaks<'_, '_, L, C>
where
    L: Language + 'static,
    C: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        // Store the last token and last trailing comment before formatting the content which will override the state.
        // Is it safe to set `last_trailing_comment` only in the format removed because format removed may set it to true
        // but it's false for the "break" case. Ignorable, because it's after a new line break in that case?
        let last_token = f.state().last_token_kind();
        let is_last_content_inline_comment = f.state().is_last_content_inline_comment();

        write!(
            f,
            [
                if_group_breaks(&Arguments::from(&self.content)).with_group_id(self.group_id),
                // Print the trivia otherwise
                if_group_fits_on_line(&format_with(|f| {
                    // Restore state to how it was before formatting the "breaks" variant
                    f.state_mut().set_last_token_kind_raw(last_token);
                    f.state_mut()
                        .set_last_content_inline_comment(is_last_content_inline_comment);

                    write_removed_token_trivia(self.token, last_token, f)
                }))
                .with_group_id(self.group_id),
            ]
        )
    }
}

/// Determines if the whitespace separating comment trivia
/// from their associated tokens should be printed or trimmed
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum TriviaPrintMode {
    #[default]
    Full,
    Trim,
}

/// Formats the leading trivia (comments, skipped token trivia) of a token
pub fn format_leading_trivia<L: Language>(token: &SyntaxToken<L>) -> FormatLeadingTrivia<L> {
    FormatLeadingTrivia {
        trim_mode: TriviaPrintMode::Full,
        token,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatLeadingTrivia<'a, L>
where
    L: Language,
{
    trim_mode: TriviaPrintMode,
    token: &'a SyntaxToken<L>,
}

impl<'a, L> FormatLeadingTrivia<'a, L>
where
    L: Language,
{
    pub fn with_trim_mode(mut self, mode: TriviaPrintMode) -> Self {
        self.trim_mode = mode;
        self
    }
}

impl<L, C> Format<C> for FormatLeadingTrivia<'_, L>
where
    L: Language,
    C: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        write_leading_trivia(
            self.token.leading_trivia().pieces(),
            self.token,
            self.trim_mode,
            f,
        )?;

        Ok(())
    }
}

fn write_leading_trivia<I, L, C>(
    pieces: I,
    token: &SyntaxToken<L>,
    trim_mode: TriviaPrintMode,
    f: &mut Formatter<C>,
) -> FormatResult<()>
where
    I: IntoIterator<Item = SyntaxTriviaPiece<L>>,
    L: Language,
    C: CstFormatContext<Language = L>,
{
    let mut lines_before = 0;
    let mut comments = Vec::new();
    let mut pieces = pieces.into_iter();

    while let Some(piece) = pieces.next() {
        if let Some(comment) = piece.as_comments() {
            if !f.state().is_comment_formatted(&comment) {
                comments.push(SourceComment::leading(comment, lines_before));
            }

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
                    comments.push(SourceComment::leading(comment, lines_before));
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
            FormatTrailingTrivia::skipped(skipped_trailing_comments.into_iter()).fmt(f)?;

            // Ensure that there's some whitespace between the last skipped token trivia and the
            // next token except if there was no whitespace present in the source.
            if lines_before > 0 {
                write!(f, [hard_line_break()])?;
            } else if spaces > 0 {
                write!(f, [space()])?;
            };

            // Write  leading comments of the next token
            FormatLeadingComments {
                comments: &token_leading_comments,
                lines_before_token: lines_before,
                trim_mode,
            }
            .fmt(f)?;

            return Ok(());
        }
    }

    FormatLeadingComments {
        comments: &comments,
        trim_mode,
        lines_before_token: lines_before,
    }
    .fmt(f)
}

struct FormatLeadingComments<'a, L>
where
    L: Language,
{
    comments: &'a [SourceComment<L>],
    trim_mode: TriviaPrintMode,
    lines_before_token: u32,
}

impl<L, C> Format<C> for FormatLeadingComments<'_, L>
where
    L: Language,
    C: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        let mut first = true;
        let mut last_inline_comment = f.state().is_last_content_inline_comment();

        for (index, comment) in self.comments.iter().enumerate() {
            if f.state().is_comment_formatted(comment.piece()) {
                continue;
            }

            let lines_after = self
                .comments
                .get(index + 1)
                .map(|comment| comment.lines_before())
                .unwrap_or_else(|| match self.trim_mode {
                    TriviaPrintMode::Full => self.lines_before_token,
                    TriviaPrintMode::Trim => 0,
                });

            #[allow(deprecated)]
            let comment_kind = f
                .context()
                .comment_style()
                .get_comment_kind(comment.piece());

            last_inline_comment = comment_kind.is_inline() && lines_after == 0;

            let format_content = format_with(|f| {
                if comment.lines_before() > 0 && first {
                    write!(f, [hard_line_break()])?;
                } else if !first {
                    write!(f, [space()])?;
                };

                let format_comment =
                    FormatRefWithRule::new(comment, C::LeadingCommentRule::default());

                write!(f, [format_comment])?;

                match comment_kind {
                    CommentKind::Line => match lines_after {
                        0 | 1 => write!(f, [hard_line_break()])?,
                        _ => write!(f, [empty_line()])?,
                    },
                    CommentKind::InlineBlock | CommentKind::Block => {
                        match lines_after {
                            0 => {
                                // space between last comment and token handled at the end.
                                // space between comments is inserted before each comment
                            }
                            1 => write!(f, [hard_line_break()])?,
                            _ => write!(f, [empty_line()])?,
                        }
                    }
                }

                Ok(())
            });

            write!(f, [crate::comment(&format_content)])?;
            first = false;
        }

        f.state_mut()
            .set_last_content_inline_comment(last_inline_comment);

        Ok(())
    }
}

/// Formats the trailing trivia (comments) of a token
pub fn format_trailing_trivia<L: Language>(
    token: &SyntaxToken<L>,
) -> FormatTrailingTrivia<impl Iterator<Item = SourceComment<L>> + Clone, L> {
    let comments = token
        .trailing_trivia()
        .pieces()
        .filter_map(|piece| piece.as_comments().map(SourceComment::trailing));

    FormatTrailingTrivia::new(comments, token.kind())
}

#[derive(Debug, Copy, Clone)]
pub struct FormatTrailingTrivia<I, L: Language>
where
    I: Iterator<Item = SourceComment<L>> + Clone,
{
    /// The comments to format
    comments: I,

    /// The kind of the token of which the comments are the trailing trivia.
    /// `Some(kind)` for a regular token. `None` for a skipped token trivia OR
    token_kind: Option<<L as Language>::Kind>,

    skip_formatted_check: bool,
}

impl<I, L: Language> FormatTrailingTrivia<I, L>
where
    I: Iterator<Item = SourceComment<L>> + Clone,
{
    pub fn new(comments: I, token_kind: L::Kind) -> Self {
        Self {
            comments,
            token_kind: Some(token_kind),
            skip_formatted_check: false,
        }
    }

    pub fn skipped(comments: I) -> Self {
        Self {
            comments,
            token_kind: None,
            skip_formatted_check: false,
        }
    }

    pub fn skip_formatted_check(mut self) -> Self {
        self.skip_formatted_check = true;
        self
    }
}

impl<I, L: Language, C> Format<C> for FormatTrailingTrivia<I, L>
where
    I: Iterator<Item = SourceComment<L>> + Clone,
    C: CstFormatContext<Language = L>,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        let comments = self.comments.clone();
        let mut last_inline_comment = f.state().is_last_content_inline_comment();

        for (index, comment) in comments.enumerate() {
            if !self.skip_formatted_check && f.state().is_comment_formatted(comment.piece()) {
                continue;
            }

            #[allow(deprecated)]
            let kind = f
                .context()
                .comment_style()
                .get_comment_kind(comment.piece());
            last_inline_comment = kind.is_inline();
            let is_single_line = kind.is_line();

            let content = format_with(|f: &mut Formatter<C>| {
                if !is_single_line {
                    match self.token_kind {
                        // Don't write a space if this is a group start token and it isn't the first trailing comment
                        #[allow(deprecated)]
                        Some(token)
                            if f.context().comment_style().is_group_start_token(token)
                                && index == 0 => {}
                        //  Write a space for all other cases
                        _ => space().fmt(f)?,
                    }
                    comment.piece().fmt(f)
                } else {
                    write![
                        f,
                        [
                            line_suffix(&format_args![space(), comment.piece()]),
                            expand_parent()
                        ]
                    ]
                }
            });

            crate::comment(&content).fmt(f)?;
        }

        f.state_mut()
            .set_last_content_inline_comment(last_inline_comment);

        Ok(())
    }
}
