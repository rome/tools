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
