use rome_rowan::{Language, SyntaxTriviaPieceComments};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CommentKind {
    /// An inline comment that can appear between any two tokens and doesn't contain any line breaks.
    /// For example, a `/* test */` comment in JavaScript.
    InlineBlock,

    /// A block comment that can appear between any two tokens and contains at least one line break.
    /// For example, a `/* first line\nmore content on the second line */` comment in JavaScript.
    Block,

    /// A line comment that appears at the end of the line. For example the `// test` comment in JavaScript.
    Line,
}

#[derive(Debug, Clone)]
pub struct Comment<L: Language> {
    /// The number of lines appearing before this comment
    lines_before: u32,

    /// The comment piece
    piece: SyntaxTriviaPieceComments<L>,
}

impl<L: Language> Comment<L> {
    /// Creates a new trailing comment. A trailing comment always has 0 lines before.
    pub fn trailing(piece: SyntaxTriviaPieceComments<L>) -> Self {
        Self {
            lines_before: 0,
            piece,
        }
    }

    /// Creates a leading comment with the specified lines before
    pub fn leading(piece: SyntaxTriviaPieceComments<L>, lines_before: u32) -> Self {
        Self {
            lines_before,
            piece,
        }
    }

    /// Returns the underlining comment trivia piece
    pub fn piece(&self) -> &SyntaxTriviaPieceComments<L> {
        &self.piece
    }

    /// Returns the number of lines before directly before this comment
    pub fn lines_before(&self) -> u32 {
        self.lines_before
    }
}

impl CommentKind {
    pub const fn is_line(&self) -> bool {
        matches!(self, CommentKind::Line)
    }

    pub const fn is_block(&self) -> bool {
        matches!(self, CommentKind::Block)
    }

    pub const fn is_inline_block(&self) -> bool {
        matches!(self, CommentKind::InlineBlock)
    }
}

pub trait CommentStyle {
    type Language: Language;

    /// Returns the kind of the comment
    fn get_comment_kind(&self, comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind;

    /// Returns `true` if a token with the passed `kind` marks the start of a group. Common group tokens are
    /// * left parentheses: `(`, `[`, `{`
    fn is_group_start_token(&self, kind: <Self::Language as Language>::Kind) -> bool;

    /// Returns `true` if a token with the passed `kind` marks the end of a group. Common group end tokens are:
    /// * right parentheses: `)`, `]`, `}`
    /// * end of statement token: `;`
    /// * element separator: `,` or `.`.
    /// * end of file token: `EOF`
    fn is_group_end_token(&self, kind: <Self::Language as Language>::Kind) -> bool;
}
