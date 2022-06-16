use crate::prelude::*;
use crate::{
    format_args, write, Argument, Arguments, CommentStyle, FormatContext, GroupId, LastTokenKind,
    SourceComment,
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
    C: FormatContext,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        f.state_mut().set_last_token_kind(self.token.kind());

        let trimmed_range = self.token.text_trimmed_range();

        syntax_token_text_slice(self.token, trimmed_range).fmt(f)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct InsertedToken<Kind> {
    kind: Kind,
    text: &'static str,
}

/// Formats a token that has been inserted by the formatter and isn't present in the source text.
/// Takes care of correctly handling spacing to the previous token's trailing trivia.
pub struct FormatInserted<S>
where
    S: CommentStyle,
{
    token: InsertedToken<<S::Language as Language>::Kind>,
    style: S,
}

impl<S> FormatInserted<S>
where
    S: CommentStyle,
{
    pub fn new(kind: <S::Language as Language>::Kind, text: &'static str, style: S) -> Self {
        Self {
            token: InsertedToken { kind, text },
            style,
        }
    }
}

impl<S, C> Format<C> for FormatInserted<S>
where
    S: CommentStyle + 'static,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        let is_last_content_inline_content = f.state().is_last_content_inline_comment();

        // Insert a space if the previous token has any trailing comments and this is not a group
        // end token
        if is_last_content_inline_content && !self.style.is_group_end_token(self.token.kind) {
            space_token().fmt(f)?;
        }

        f.state_mut().set_last_token_kind(self.token.kind);

        token(self.token.text).fmt(f)
    }
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
#[derive(Copy, Clone, Debug)]
pub struct FormatInsertedOpenParen<'a, S>
where
    S: CommentStyle,
{
    /// The token before which the open paren must be inserted
    before_token: &'a SyntaxToken<S::Language>,

    /// The token text of the open paren
    text: &'static str,

    /// The kind of the open paren
    kind: <S::Language as Language>::Kind,

    style: S,
}

impl<'a, S> FormatInsertedOpenParen<'a, S>
where
    S: CommentStyle,
{
    pub fn new(
        before_token: &'a SyntaxToken<S::Language>,
        kind: <S::Language as Language>::Kind,
        text: &'static str,
        style: S,
    ) -> Self {
        Self {
            before_token,
            kind,
            text,
            style,
        }
    }
}

impl<Context, S> Format<Context> for FormatInsertedOpenParen<'_, S>
where
    S: CommentStyle + 'static,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        // Format the leading trivia of the next token as the leading trivia of the open paren.
        let leading_pieces = self.before_token.leading_trivia().pieces();

        let mut lines_before = 0;
        let mut comments = Vec::new();

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
                options: LeadingTriviaOptions {
                    style: self.style,
                    trim_mode: TriviaPrintMode::Full,
                },
                lines_before_token: lines_before,
            }]
        )?;

        let is_last_content_inline_comment = f.state().is_last_content_inline_comment();

        if needs_space_between_comments_and_token(
            &comments,
            self.kind,
            is_last_content_inline_comment,
            self.style,
        ) {
            space_token().fmt(f)?;
        }

        f.state_mut().set_last_content_is_inline_comment(false);

        write!(
            f,
            [FormatInserted {
                token: InsertedToken {
                    kind: self.kind,
                    text: self.text,
                },
                style: self.style,
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
pub struct FormatInsertedCloseParen<S>
where
    S: CommentStyle,
{
    /// The token after which the close paren must be inserted
    comments: Vec<SourceComment<S::Language>>,

    /// The token text of the close paren
    text: &'static str,

    /// The kind of the close paren
    kind: <S::Language as Language>::Kind,

    style: S,
}

impl<S> FormatInsertedCloseParen<S>
where
    S: CommentStyle,
{
    pub fn new<Context>(
        after_token: &SyntaxToken<S::Language>,
        kind: <S::Language as Language>::Kind,
        text: &'static str,
        style: S,
        f: &mut Formatter<Context>,
    ) -> Self {
        let mut comments = Vec::new();
        // "Steal" the trailing comments and mark them as handled.
        // Must be done eagerly before formatting because the `after_token`
        // gets formatted **before** formatting the inserted paren.
        for piece in after_token.trailing_trivia().pieces() {
            if let Some(comment) = piece.as_comments() {
                f.state_mut().mark_comment_as_formatted(&comment);
                comments.push(SourceComment::trailing(comment));
            }
        }

        Self {
            comments,
            kind,
            text,
            style,
        }
    }
}

impl<Context, S> Format<Context> for FormatInsertedCloseParen<S>
where
    S: CommentStyle + 'static,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        write!(
            f,
            [
                FormatInserted {
                    token: InsertedToken {
                        kind: self.kind,
                        text: self.text,
                    },
                    style: self.style,
                },
                FormatTrailingTrivia::new(self.comments.iter().cloned(), self.kind, self.style,)
                    .skip_formatted_check()
            ]
        )
    }
}

/// Formats the trivia of a token that is present in the source text but should be omitted in the
/// formatted output.
pub struct FormatRemoved<'a, S>
where
    S: CommentStyle,
{
    token: &'a SyntaxToken<S::Language>,
    style: S,
}

impl<'a, S> FormatRemoved<'a, S>
where
    S: CommentStyle,
{
    pub const fn new(token: &'a SyntaxToken<S::Language>, style: S) -> Self {
        Self { token, style }
    }
}

impl<C, S> Format<C> for FormatRemoved<'_, S>
where
    S: CommentStyle + 'static,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        f.state_mut().track_token(self.token);

        let last = f.state().last_token_kind();

        let is_last_content_inline_comment = f.state().is_last_content_inline_comment();
        write_removed_token_trivia(
            self.token,
            last,
            is_last_content_inline_comment,
            self.style,
            f,
        )
    }
}

/// Writes the trivia of a removed token
fn write_removed_token_trivia<C, S>(
    token: &SyntaxToken<S::Language>,
    last_token: Option<LastTokenKind>,
    is_last_content_inline_comment: bool,
    style: S,
    f: &mut Formatter<C>,
) -> FormatResult<()>
where
    S: CommentStyle + 'static,
{
    let mut pieces = token
        .leading_trivia()
        .pieces()
        .chain(token.trailing_trivia().pieces())
        .peekable();

    let last_token = last_token.and_then(|token| token.as_language::<S::Language>());

    // If this isn't the first token than format all comments that are before the first skipped token
    // trivia or line break as the trailing trivia of the previous token (which these comments will
    // become if the document gets formatted a second time).
    let has_trailing_inline_comment = if let Some(last_token) = last_token {
        let mut trailing_comments = vec![];
        let mut is_last_inline = is_last_content_inline_comment;

        while let Some(piece) = pieces.peek() {
            if let Some(comment) = piece.as_comments() {
                if !f.state().is_comment_formatted(&comment) {
                    is_last_inline = style.get_comment_kind(&comment).is_inline();
                    trailing_comments.push(SourceComment::trailing(comment));
                }
            } else if piece.is_newline() || piece.is_skipped() {
                break;
            }

            pieces.next();
        }

        FormatTrailingTrivia::new(trailing_comments.into_iter(), last_token, style).fmt(f)?;

        is_last_inline
    } else {
        is_last_content_inline_comment
    };

    let next_token_leading_comments = write_leading_trivia(
        pieces,
        token,
        LeadingTriviaOptions {
            style,
            trim_mode: TriviaPrintMode::Full,
        },
        f,
    )?;

    // There's a trailing inline comment if:
    // * the last comment in the leading trivia is an inline comment
    // * there's no leading comment and the last trailing comment is an inline comment
    // * there's neither leading nor trailing comment, in which case the last comment written
    //   is the comment from the previous token
    f.state_mut().set_last_content_is_inline_comment(
        next_token_leading_comments
            .last()
            .map(|c| style.get_comment_kind(c.piece()).is_inline())
            .unwrap_or(has_trailing_inline_comment),
    );

    Ok(())
}

/// Formats a token's leading and trailing trivia but uses the provided content instead
/// of the token in the formatted output.
#[derive(Copy, Clone)]
pub struct FormatReplaced<'a, 'content, S, C>
where
    S: CommentStyle,
{
    token: &'a SyntaxToken<S::Language>,
    content: Argument<'content, C>,
    style: S,
}

impl<'a, 'content, S, C> FormatReplaced<'a, 'content, S, C>
where
    S: CommentStyle,
{
    pub fn new<Content>(
        token: &'a SyntaxToken<S::Language>,
        content: &'content Content,
        style: S,
    ) -> Self
    where
        Content: Format<C>,
    {
        FormatReplaced {
            token,
            content: Argument::new(content),
            style,
        }
    }
}

impl<S, C> Format<C> for FormatReplaced<'_, '_, S, C>
where
    S: CommentStyle,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        f.state_mut().track_token(self.token);

        FormatLeadingTrivia::new(self.token, self.style).fmt(f)?;
        f.write_fmt(Arguments::from(&self.content))?;
        format_token_trailing_trivia(self.token, self.style).fmt(f)
    }
}

/// Formats a token with its leading and trailing trivia that only gets printed if its enclosing
/// group does break but otherwise gets omitted from the formatted output.
pub struct FormatOnlyIfBreaks<'a, 'content, S, C>
where
    S: CommentStyle,
{
    token: &'a SyntaxToken<S::Language>,
    content: Argument<'content, C>,
    style: S,
    group_id: Option<GroupId>,
}

impl<'a, 'content, S, C> FormatOnlyIfBreaks<'a, 'content, S, C>
where
    S: CommentStyle,
{
    pub fn new<Content>(
        token: &'a SyntaxToken<S::Language>,
        content: &'content Content,
        style: S,
    ) -> Self
    where
        S: CommentStyle,
        Content: Format<C>,
    {
        Self {
            token,
            content: Argument::new(content),
            style,
            group_id: None,
        }
    }

    pub fn with_group_id(mut self, group_id: Option<GroupId>) -> Self {
        self.group_id = group_id;
        self
    }
}

impl<S, C> Format<C> for FormatOnlyIfBreaks<'_, '_, S, C>
where
    S: CommentStyle + 'static,
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
                    write_removed_token_trivia(
                        self.token,
                        last_token,
                        is_last_content_inline_comment,
                        self.style,
                        f,
                    )
                }))
                .with_group_id(self.group_id)
            ]
        )
    }
}

/// Determines if the whitespace separating comment trivia
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
pub struct FormatLeadingTrivia<'a, S>
where
    S: CommentStyle,
{
    trim_mode: TriviaPrintMode,
    token: &'a SyntaxToken<S::Language>,
    style: S,
}

impl<'a, S> FormatLeadingTrivia<'a, S>
where
    S: CommentStyle,
{
    pub fn new(token: &'a SyntaxToken<S::Language>, style: S) -> Self {
        Self {
            token,
            style,
            trim_mode: TriviaPrintMode::Full,
        }
    }

    pub fn with_trim_mode(mut self, mode: TriviaPrintMode) -> Self {
        self.trim_mode = mode;
        self
    }
}

impl<S, C> Format<C> for FormatLeadingTrivia<'_, S>
where
    S: CommentStyle,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        let leading_comments = write_leading_trivia(
            self.token.leading_trivia().pieces(),
            self.token,
            LeadingTriviaOptions {
                trim_mode: self.trim_mode,
                style: self.style,
            },
            f,
        )?;

        let is_last_content_inline_comment = f.state().is_last_content_inline_comment();

        if needs_space_between_comments_and_token(
            &leading_comments,
            self.token.kind(),
            is_last_content_inline_comment,
            self.style,
        ) {
            space_token().fmt(f)?;
        }

        f.state_mut().set_last_content_is_inline_comment(false);

        Ok(())
    }
}

fn needs_space_between_comments_and_token<S: CommentStyle>(
    comments: &[SourceComment<S::Language>],
    current_token_kind: <S::Language as Language>::Kind,
    has_trailing_inline_comment: bool,
    style: S,
) -> bool {
    let is_last_comment_inline_comment = comments
        .last()
        .map(|comment| style.get_comment_kind(comment.piece()).is_inline())
        .unwrap_or(has_trailing_inline_comment);

    if is_last_comment_inline_comment {
        // Don't insert a space if the current token is a group end token
        !style.is_group_end_token(current_token_kind)
    } else {
        false
    }
}

#[derive(Debug, Clone, Default)]
struct LeadingTriviaOptions<S> {
    trim_mode: TriviaPrintMode,
    style: S,
}

fn write_leading_trivia<I, S, C>(
    pieces: I,
    token: &SyntaxToken<S::Language>,
    options: LeadingTriviaOptions<S>,
    f: &mut Formatter<C>,
) -> FormatResult<Vec<SourceComment<S::Language>>>
where
    I: IntoIterator<Item = SyntaxTriviaPiece<S::Language>>,
    S: CommentStyle,
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
                options: LeadingTriviaOptions {
                    trim_mode: TriviaPrintMode::Full,
                    ..options
                },
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
            FormatTrailingTrivia::skipped(skipped_trailing_comments.into_iter(), options.style)
                .fmt(f)?;

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
                options,
            }
            .fmt(f)?;

            return Ok(token_leading_comments);
        }
    }

    FormatLeadingComments {
        comments: &comments,
        options,
        lines_before_token: lines_before,
    }
    .fmt(f)?;

    Ok(comments)
}

struct FormatLeadingComments<'a, S>
where
    S: CommentStyle,
{
    comments: &'a [SourceComment<S::Language>],
    options: LeadingTriviaOptions<S>,
    lines_before_token: u32,
}

impl<S, C> Format<C> for FormatLeadingComments<'_, S>
where
    S: CommentStyle,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        if self.comments.is_empty() {
            return Ok(());
        }

        let format_comments = format_with(|f| {
            for (index, comment) in self.comments.iter().enumerate() {
                if f.state().is_comment_formatted(comment.piece()) {
                    continue;
                }

                let is_line_comment = self
                    .options
                    .style
                    .get_comment_kind(comment.piece())
                    .is_line();

                let lines_after = self
                    .comments
                    .get(index + 1)
                    .map(|comment| comment.lines_before())
                    .unwrap_or_else(|| match self.options.trim_mode {
                        TriviaPrintMode::Full => self.lines_before_token,
                        TriviaPrintMode::Trim => 0,
                    });

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
            }
            Ok(())
        });

        write!(f, [comments(&format_comments, CommentPosition::Leading)])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FormatTrailingTrivia<I, S: CommentStyle>
where
    I: Iterator<Item = SourceComment<S::Language>> + Clone,
{
    /// The comments to format
    comments: I,

    /// The language specific style on how to format comments
    style: S,

    /// The kind of the token of which the comments are the trailing trivia.
    /// `Some(kind)` for a regular token. `None` for a skipped token trivia OR
    token_kind: Option<<S::Language as Language>::Kind>,

    skip_formatted_check: bool,
}

pub fn format_token_trailing_trivia<S>(
    token: &SyntaxToken<S::Language>,
    style: S,
) -> FormatTrailingTrivia<impl Iterator<Item = SourceComment<S::Language>> + Clone, S>
where
    S: CommentStyle,
{
    let comments = token
        .trailing_trivia()
        .pieces()
        .filter_map(|piece| piece.as_comments().map(SourceComment::trailing));

    FormatTrailingTrivia::new(comments, token.kind(), style)
}

impl<I, S: CommentStyle> FormatTrailingTrivia<I, S>
where
    I: Iterator<Item = SourceComment<S::Language>> + Clone,
{
    pub fn new(comments: I, token_kind: <S::Language as Language>::Kind, style: S) -> Self {
        Self {
            comments,
            style,
            token_kind: Some(token_kind),
            skip_formatted_check: false,
        }
    }

    pub fn skipped(comments: I, style: S) -> Self {
        Self {
            comments,
            style,
            token_kind: None,
            skip_formatted_check: false,
        }
    }

    pub fn skip_formatted_check(mut self) -> Self {
        self.skip_formatted_check = true;
        self
    }
}

impl<I, S: CommentStyle, C> Format<C> for FormatTrailingTrivia<I, S>
where
    I: Iterator<Item = SourceComment<S::Language>> + Clone,
{
    fn fmt(&self, f: &mut Formatter<C>) -> FormatResult<()> {
        let comments = self.comments.clone();
        let mut last_inline_comment = false;

        let mut comments = comments.enumerate().peekable();
        let is_empty = comments.peek().is_none();

        if is_empty {
            return Ok(());
        }

        let format_comments = format_once(|f| {
            for (index, comment) in comments {
                if !self.skip_formatted_check && f.state().is_comment_formatted(comment.piece()) {
                    continue;
                }

                let kind = self.style.get_comment_kind(comment.piece());
                last_inline_comment = kind.is_inline();
                let is_single_line = kind.is_line();

                if !is_single_line {
                    match self.token_kind {
                        // Don't write a space if this is a group start token and it isn't the first trailing comment
                        Some(token) if self.style.is_group_start_token(token) && index == 0 => {}
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
            }

            Ok(())
        });

        crate::comments(&format_comments, CommentPosition::Trailing).fmt(f)?;
        f.state_mut()
            .set_last_content_is_inline_comment(last_inline_comment);
        Ok(())
    }
}
