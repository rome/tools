use crate::{FormatLanguage, TextSize};
use rome_rowan::syntax::SyntaxTriviaPieceSkipped;
use rome_rowan::{
    Direction, Language, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken,
    SyntaxTriviaPieceComments, WalkEvent,
};
#[cfg(debug_assertions)]
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CommentKind {
    /// An inline comment that can appear between any two tokens and doesn't contain any line breaks.
    ///
    /// ## Examples
    ///
    /// ### JavaScript:
    ///
    /// ```javascript
    /// a /* test */
    /// ```
    InlineBlock,

    /// A block comment that can appear between any two tokens and contains at least one line break.
    ///
    /// ## Examples
    ///
    /// ### JavaScript
    ///
    /// ```javascript
    /// /* first line
    ///  * more content on the second line
    ///  */
    /// ```
    Block,

    /// A line comment that appears at the end of the line.
    ///
    /// ## Examples
    ///
    /// ### JavaScript
    ///
    /// ```javascript
    /// a // test
    /// ```
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

    /// Returns `true` for comments that can appear inline between any two tokens.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use rome_formatter::CommentKind;
    ///
    /// // Block and InlineBlock comments can appear inline
    /// assert!(CommentKind::Block.is_inline());
    /// assert!(CommentKind::InlineBlock.is_inline());
    ///
    /// // But not line comments
    /// assert!(!CommentKind::Line.is_inline())
    /// ```
    pub const fn is_inline(&self) -> bool {
        matches!(self, CommentKind::InlineBlock | CommentKind::Block)
    }
}

#[derive(Debug, Clone)]
pub struct SourceComment<L: Language> {
    /// The number of lines appearing before this comment
    lines_before: u32,

    lines_after: u32,

    /// The comment piece
    piece: SyntaxTriviaPieceComments<L>,

    kind: CommentKind,
}

impl<L: Language> SourceComment<L> {
    /// Creates a new trailing comment. A trailing comment always has 0 lines before.
    pub fn trailing(piece: SyntaxTriviaPieceComments<L>) -> Self {
        Self {
            lines_before: 0,
            piece,

            // FIXME
            kind: CommentKind::InlineBlock,
            lines_after: 0,
        }
    }

    /// Creates a leading comment with the specified lines before
    pub fn leading(piece: SyntaxTriviaPieceComments<L>, lines_before: u32) -> Self {
        Self {
            lines_before,
            piece,
            // FIXME
            kind: CommentKind::InlineBlock,
            lines_after: 0,
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

    pub fn lines_after(&self) -> u32 {
        self.lines_after
    }

    /// The kind of the comment
    pub fn kind(&self) -> CommentKind {
        self.kind
    }
}

#[derive(Debug, Clone)]
pub struct SkippedTokenTrivia<L: Language> {
    lines_before: u32,
    piece: SyntaxTriviaPieceSkipped<L>,
}

#[derive(Debug, Clone)]
pub enum DanglingTrivia<L: Language> {
    Comment(SourceComment<L>),
    SkippedToken(SkippedTokenTrivia<L>),
}

impl<L: Language> DanglingTrivia<L> {
    pub const fn is_comment(&self) -> bool {
        matches!(self, DanglingTrivia::Comment(_))
    }
}

#[derive(Debug, Clone)]
pub struct DecoratedComment<L: Language> {
    preceding: Option<SyntaxNode<L>>,
    following: Option<SyntaxNode<L>>,
    following_token: SyntaxToken<L>,
    lines_before: u32,
    lines_after: u32,
    trailing_token_comment: bool,
    comment: SyntaxTriviaPieceComments<L>,
    kind: CommentKind,
}

impl<L: Language> DecoratedComment<L> {
    /// The node that fully encloses the comment (the comment's start and end position are fully in the
    /// node's bounds).
    pub fn enclosing_node(&self) -> SyntaxNode<L> {
        // SAFETY: Guaranteed by the fact that comments are extracted from a root node.
        self.comment
            .as_piece()
            .token()
            .parent()
            .expect("Expected token to have a parent node.")
    }

    pub fn piece(&self) -> &SyntaxTriviaPieceComments<L> {
        &self.comment
    }

    pub fn enclosing_token(&self) -> SyntaxToken<L> {
        self.comment.as_piece().token()
    }

    /// The node directly preceding the comment or [None] if the comment is preceded by a token or is the first
    /// token in the program.
    pub fn preceding_node(&self) -> Option<&SyntaxNode<L>> {
        self.preceding.as_ref()
    }

    fn take_preceding_node(&mut self) -> Option<SyntaxNode<L>> {
        self.preceding.take()
    }

    /// The node directly following the comment or [None] if the comment is followed by a token or is the last token in the program.
    pub fn following_node(&self) -> Option<&SyntaxNode<L>> {
        self.following.as_ref()
    }

    fn take_following_node(&mut self) -> Option<SyntaxNode<L>> {
        self.following.take()
    }

    /// The number of lines between this comment and the **previous** token, comment or skipped trivia.
    pub fn lines_before(&self) -> u32 {
        self.lines_before
    }

    pub fn lines_after(&self) -> u32 {
        self.lines_after
    }

    /// `true` if the comment is part of the tokens [trailing trivia](SyntaxToken::trailing_trivia)
    pub fn is_trailing_token_trivia(&self) -> bool {
        self.trailing_token_comment
    }

    /// Returns the [kind](CommentKind) of the comment.
    pub fn kind(&self) -> CommentKind {
        self.kind
    }

    pub fn following_token(&self) -> &SyntaxToken<L> {
        &self.following_token
    }
}

impl<L: Language> From<DecoratedComment<L>> for SourceComment<L> {
    fn from(decorated: DecoratedComment<L>) -> Self {
        Self {
            lines_before: decorated.lines_before,
            lines_after: decorated.lines_after,
            piece: decorated.comment,
            kind: decorated.kind,
        }
    }
}

#[derive(Debug)]
pub enum CommentPosition<L: Language> {
    /// Overrides the positioning of the comment to be a leading node comment.
    Leading {
        node: SyntaxNode<L>,
        comment: DecoratedComment<L>,
    },
    /// Overrides the positioning of the comment to be a trailing node comment.
    Trailing {
        node: SyntaxNode<L>,
        comment: DecoratedComment<L>,
    },

    Dangling {
        token: SyntaxToken<L>,
        comment: DecoratedComment<L>,
    },

    /// Uses the default positioning rules for the comment.
    /// TODO document rules
    Default(DecoratedComment<L>),
}

/// Defines how to format comments for a specific [Language].
pub trait CommentStyle {
    type Language: Language;

    /// Returns `true` if a comment with the given `text` is a `rome-ignore format:` suppression comment.
    fn is_suppression(text: &str) -> bool;

    fn is_open_parentheses(kind: <Self::Language as Language>::Kind) -> bool;

    /// Returns the (kind)[CommentKind] of the comment
    fn get_comment_kind(comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind;

    fn position_comment(
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPosition<Self::Language>;
}

/// Type that stores the comments of a tree and gives access to:
///
/// * whether a node should be formatted as is because it has a leading suppression comment.
/// * a node's leading and trailing comments
/// * the dangling comments of a token
///
/// Cloning `comments` is cheap as it only involves bumping a reference counter.
#[derive(Debug, Clone, Default)]
pub struct Comments<L: Language> {
    /// The use of a [Rc] is necessary to achieve that [Comments] has a lifetime that is independent from the [crate::Formatter].
    /// Having independent lifetimes is necessary to support the use case where a (formattable object)[crate::Format]
    /// iterates over all comments, and writes them into the [crate::Formatter] (mutably borrowing the [crate::Formatter] and in turn its context).
    ///
    /// ```block
    /// for leading in f.context().comments().leading_comments(node) {
    ///     ^
    ///     |- Borrows comments
    ///   write!(f, [comment(leading.piece.text())])?;
    ///          ^
    ///          |- Mutably borrows the formatter, state, context, and comments (if comments aren't cloned)
    /// }
    /// ```
    ///
    /// Using an `Rc` here allows to cheaply clone [Comments] for these use cases.
    data: Rc<CommentsData<L>>,
}

impl<L: Language> Comments<L> {
    /// Extracts all the suppressions from `root` and its child nodes.
    pub fn from_node<FormatLanguage>(root: &SyntaxNode<L>) -> Self
    where
        FormatLanguage: crate::FormatLanguage<SyntaxLanguage = L>,
    {
        let mut builder = CommentsBuilderVisitor::<FormatLanguage>::new();

        for event in root.preorder_with_tokens(Direction::Next) {
            match event {
                WalkEvent::Enter(SyntaxElement::Node(node)) => {
                    builder.visit_node(WalkEvent::Enter(node))
                }

                WalkEvent::Leave(SyntaxElement::Node(node)) => {
                    builder.visit_node(WalkEvent::Leave(node))
                }

                WalkEvent::Enter(SyntaxElement::Token(token)) => builder.visit_token(token),
                WalkEvent::Leave(SyntaxElement::Token(_)) => {
                    // Handled as part of enter
                }
            }
        }

        Self {
            data: Rc::new(dbg!(builder.finish())),
        }
    }

    /// Returns `true` if the given [node] has
    /// * any leading comments
    /// * any trailing comments
    /// * if any direct child token has any dangling trivia (either a skipped token trivia or a comment)
    pub fn has_trivia(&self, node: &SyntaxNode<L>) -> bool {
        self.has_leading_comments(node)
            || self.has_trailing_comments(node)
            || self.has_node_dangling_trivia(node)
    }

    /// Returns `true` if the given `node` has any leading or trailing comments.
    #[inline]
    pub fn has_comments(&self, node: &SyntaxNode<L>) -> bool {
        self.has_leading_comments(node)
            || self.has_trailing_comments(node)
            || self.has_node_dangling_trivia(node)
    }

    /// Returns `true` if the given [node] has any leading comments.
    /// By default, a comment is a node's leading comment if:
    /// * the previous sibling is a token
    /// * there's a line break before the commend ending before this comment and the comment.
    #[inline]
    pub fn has_leading_comments(&self, node: &SyntaxNode<L>) -> bool {
        !self.leading_comments(node).is_empty()
    }

    /// Tests if the node has any leading comment that will be placed on its own line.
    pub fn has_leading_own_line_comment(&self, node: &SyntaxNode<L>) -> bool {
        self.leading_comments(node)
            .iter()
            .any(|comment| comment.lines_after() > 0)
    }

    /// Returns `true` if the given [node] has any trailing comments.
    /// By default, a comment is a node's trailing comment if:
    /// * the next sibling is a token
    /// * there's **no** line break between the node and this comment.
    #[inline]
    pub fn has_trailing_comments(&self, node: &SyntaxNode<L>) -> bool {
        !self.trailing_comments(node).is_empty()
    }

    /// Returns `true` if any direct child token of [node] has any dangling trivia.
    pub fn has_node_dangling_trivia(&self, node: &SyntaxNode<L>) -> bool {
        node.tokens().any(|token| self.has_dangling_trivia(&token))
    }

    pub fn node_dangling_comments<'a>(
        &'a self,
        node: &'a SyntaxNode<L>,
    ) -> impl Iterator<Item = &SourceComment<L>> + 'a {
        node.tokens()
            .flat_map(|token| self.dangling_comments(&token))
    }

    /// Returns the [node]'s leading comments.
    #[inline]
    pub fn leading_comments(&self, node: &SyntaxNode<L>) -> &[SourceComment<L>] {
        self.data.leading_comments.get(node)
    }

    /// Returns the [node]'s trailing comments.
    #[inline]
    pub fn trailing_comments(&self, node: &SyntaxNode<L>) -> &[SourceComment<L>] {
        self.data.trailing_comments.get(node)
    }

    pub fn leading_trailing_comments(
        &self,
        node: &SyntaxNode<L>,
    ) -> impl Iterator<Item = &SourceComment<L>> {
        self.leading_comments(node)
            .iter()
            .chain(self.trailing_comments(node).iter())
    }

    pub fn node_comments<'a>(
        &'a self,
        node: &'a SyntaxNode<L>,
    ) -> impl Iterator<Item = &SourceComment<L>> + 'a {
        self.leading_comments(node)
            .iter()
            .chain(self.node_dangling_comments(node))
            .chain(self.trailing_comments(node).iter())
    }

    /// Skipped token trivia or comment trivia that
    #[inline]
    pub fn dangling_trivia(&self, token: &SyntaxToken<L>) -> &[DanglingTrivia<L>] {
        self.data.dangling_trivia.get(token)
    }

    pub fn dangling_comments(
        &self,
        token: &SyntaxToken<L>,
    ) -> impl Iterator<Item = &SourceComment<L>> {
        self.dangling_trivia(token)
            .iter()
            .filter_map(|trivia| match trivia {
                DanglingTrivia::Comment(comment) => Some(comment),
                DanglingTrivia::SkippedToken(_) => None,
            })
    }

    #[inline]
    pub fn has_dangling_trivia(&self, token: &SyntaxToken<L>) -> bool {
        !self.dangling_trivia(token).is_empty()
    }

    pub fn has_dangling_comments(&self, token: &SyntaxToken<L>) -> bool {
        let dangling_trivia = self.dangling_trivia(token);

        !dangling_trivia.is_empty() && dangling_trivia.iter().all(|trivia| trivia.is_comment())
    }

    /// Returns `true` if the passed `node` has a leading suppression comment.
    ///
    /// Suppression comments only apply if they at the start of a node and they suppress the most
    /// outer node.
    ///
    /// # Examples
    ///
    /// ```javascript
    /// // rome-ignore format: Reason
    /// console.log("Test");
    /// ```
    ///
    /// Returns `true` for the expression statement but `false` for the call expression because the
    /// call expression is nested inside of the expression statement.
    pub fn is_suppressed(&self, node: &SyntaxNode<L>) -> bool {
        self.mark_suppression_checked(node);
        let is_suppression = self.data.is_suppression;

        self.leading_comments(node)
            .iter()
            .any(|comment| is_suppression(comment.piece().text()))
    }

    /// Marks that it isn't necessary for the given node to check if it has been suppressed or not.
    #[inline]
    pub fn mark_suppression_checked(&self, node: &SyntaxNode<L>) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                let mut checked_nodes = self.data.checked_suppressions.borrow_mut();
                checked_nodes.insert(node.clone());
            } else {
                let _ = node;
            }
        }
    }

    /// Verifies that [NodeSuppressions::is_suppressed] has been called for every node of `root`.
    /// This is a no-op in builds that have the feature `debug_assertions` disabled.
    ///
    /// # Panics
    /// If theres any node for which the formatting didn't very if it has a suppression comment.
    #[inline]
    pub(crate) fn assert_checked_all_suppressions(&self, root: &SyntaxNode<L>) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                let checked_nodes = self.data.checked_suppressions.borrow();
                for node in root.descendants() {
                    if node.kind().is_list() || node.kind().is_root() {
                        continue;
                    }

                    if !checked_nodes.contains(&node) {
                        panic!(r#"
The following node has been formatted without checking if it has suppression comments.
Ensure that the formatter calls into the node's formatting rule by using `node.format()` or
manually test if the node has a suppression comment using `f.context().comments().is_suppressed(node.syntax())`
if using the node's format rule isn't an option."

Node:
{node:#?}"#
                        );
                    }
                }
            } else {
                let _ = root;
            }
        }
    }
}

struct CommentsData<L: Language> {
    is_suppression: fn(&str) -> bool,

    /// Stores all leading node comments by node
    leading_comments: AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,

    /// Stores the trailing node comments by node
    trailing_comments: AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,

    /// Stores the dangling trivia by token
    dangling_trivia: AppendOnlyMultiMap<SyntaxToken<L>, DanglingTrivia<L>>,

    /// Stores all nodes for which [Comments::is_suppressed] has been called.
    /// This index of nodes that have been checked if they have a suppression comments is used to
    /// detect format implementations that manually format a child node without previously checking if
    /// the child has a suppression comment.
    ///
    /// The implementation refrains from snapshotting the checked nodes because a node gets formatted
    /// as verbatim if its formatting fails which has the same result as formatting it as suppressed node
    /// (thus, guarantees that the formatting isn't changed).
    #[cfg(debug_assertions)]
    checked_suppressions: RefCell<HashSet<SyntaxNode<L>>>,
}

impl<L> Default for CommentsData<L>
where
    L: Language,
{
    fn default() -> Self {
        Self {
            is_suppression: |_| false,
            leading_comments: Default::default(),
            trailing_comments: Default::default(),
            dangling_trivia: Default::default(),
            #[cfg(debug_assertions)]
            checked_suppressions: Default::default(),
        }
    }
}

impl<L: Language> std::fmt::Debug for CommentsData<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_comments: Vec<DebugComment<'_, L>> = Vec::new();

        for node in self.leading_comments.keys() {
            debug_comments.extend(
                self.leading_comments
                    .get(node)
                    .iter()
                    .map(|comment| DebugComment::Leading { node, comment }),
            );
        }

        for node in self.trailing_comments.keys() {
            debug_comments.extend(
                self.trailing_comments
                    .get(node)
                    .iter()
                    .map(|comment| DebugComment::Trailing { node, comment }),
            );
        }

        for token in self.dangling_trivia.keys() {
            debug_comments.extend(
                self.dangling_trivia
                    .get(token)
                    .iter()
                    .map(|trivia| DebugComment::Dangling { token, trivia }),
            );
        }

        debug_comments.sort_unstable_by_key(|comment| comment.start());

        f.debug_list().entries(debug_comments).finish()
    }
}

/// Helper for printing a comment of [Comments]
enum DebugComment<'a, L: Language> {
    Leading {
        comment: &'a SourceComment<L>,
        node: &'a SyntaxNode<L>,
    },
    Trailing {
        comment: &'a SourceComment<L>,
        node: &'a SyntaxNode<L>,
    },
    Dangling {
        trivia: &'a DanglingTrivia<L>,
        token: &'a SyntaxToken<L>,
    },
}

impl<L: Language> DebugComment<'_, L> {
    fn start(&self) -> TextSize {
        match self {
            DebugComment::Leading { comment, .. } | DebugComment::Trailing { comment, .. } => {
                comment.piece.text_range().start()
            }
            DebugComment::Dangling { trivia, .. } => match trivia {
                DanglingTrivia::Comment(comment) => comment.piece.text_range().start(),
                DanglingTrivia::SkippedToken(skipped) => skipped.piece.text_range().start(),
            },
        }
    }
}

impl<L: Language> std::fmt::Debug for DebugComment<'_, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebugComment::Leading { node, comment } => f
                .debug_struct("Leading")
                .field("node", node)
                .field("comment", comment)
                .finish(),
            DebugComment::Trailing { node, comment } => f
                .debug_struct("Trailing")
                .field("node", node)
                .field("comment", comment)
                .finish(),
            DebugComment::Dangling { trivia, token } => f
                .debug_struct("Dangling")
                .field("token", token)
                .field("trivia", trivia)
                .finish(),
        }
    }
}

#[derive(Debug, Default)]
struct CommentsBuilderVisitor<Language: FormatLanguage> {
    node_comments: NodeCommentsBuilder<Language::SyntaxLanguage>,
    dangling_trivia: DanglingTriviaBuilder<Language::SyntaxLanguage>,
    preceding_node: Option<SyntaxNode<Language::SyntaxLanguage>>,
    following_node: Option<SyntaxNode<Language::SyntaxLanguage>>,
    last_token: Option<SyntaxToken<Language::SyntaxLanguage>>,
}

impl<Language> CommentsBuilderVisitor<Language>
where
    Language: FormatLanguage,
{
    fn new() -> Self {
        Self {
            node_comments: NodeCommentsBuilder::default(),
            dangling_trivia: DanglingTriviaBuilder::default(),
            preceding_node: None,
            following_node: None,
            last_token: None,
        }
    }

    fn visit_node(&mut self, event: WalkEvent<SyntaxNode<Language::SyntaxLanguage>>) {
        match event {
            WalkEvent::Enter(node) => {
                // Lists cannot have comments attached. They either belong to the entire parent or to
                // the first child.
                if node.kind().is_list() {
                    return;
                }

                // Associate comments with the most outer node
                if self.following_node.is_none() {
                    self.following_node = Some(node);
                }
            }

            WalkEvent::Leave(node) => {
                if self.following_node.as_ref() == Some(&node) {
                    self.following_node = None;
                }
                if !node.kind().is_list() {
                    self.preceding_node = Some(node);
                }
            }
        }
    }

    fn visit_token(&mut self, token: SyntaxToken<Language::SyntaxLanguage>) {
        // Store the last processed comment so that we can set `line_break_after`
        let mut last_comment = None;

        if let Some(last_token) = self.last_token.take() {
            for piece in last_token
                .trailing_trivia()
                .pieces()
                .filter_map(|piece| piece.as_comments())
            {
                if let Some(last_comment) = last_comment.take() {
                    self.handle_comment(last_comment, &token);
                }

                last_comment = Some(DecoratedComment {
                    preceding: self.preceding_node.clone(),
                    following: self.following_node.clone(),
                    following_token: token.clone(),
                    lines_before: 0,
                    lines_after: 0,
                    trailing_token_comment: true,
                    kind: Language::CommentStyle::get_comment_kind(&piece),
                    comment: piece,
                });
            }
        }

        let mut lines_before = 0;
        let mut has_skipped = false;

        for leading in token.leading_trivia().pieces() {
            if leading.is_newline() {
                lines_before += 1;
            } else if let Some(skipped) = leading.as_skipped() {
                if let Some(mut last_comment) = last_comment.take() {
                    last_comment.lines_after = lines_before;
                    self.handle_comment(last_comment, &token);
                }

                self.dangling_trivia.insert_trivia(
                    token.clone(),
                    DanglingTrivia::SkippedToken(SkippedTokenTrivia {
                        lines_before,
                        piece: skipped,
                    }),
                );

                lines_before = 0;
                has_skipped = true;
            } else if let Some(comment) = leading.as_comments() {
                if let Some(mut last_comment) = last_comment.take() {
                    last_comment.lines_after = lines_before;
                    self.handle_comment(last_comment, &token);
                }

                let kind = Language::CommentStyle::get_comment_kind(&comment);
                if has_skipped {
                    self.dangling_trivia.insert_trivia(
                        token.clone(),
                        DanglingTrivia::Comment(SourceComment {
                            lines_before,
                            // FIXME
                            lines_after: 0,
                            piece: comment,
                            kind,
                        }),
                    );
                } else {
                    last_comment = Some(DecoratedComment {
                        preceding: self.preceding_node.clone(),
                        following: self.following_node.clone(),
                        following_token: token.clone(),
                        lines_before,
                        lines_after: 0,
                        trailing_token_comment: false,
                        kind,
                        comment,
                    });
                }
                lines_before = 0;
            }
        }

        if let Some(mut last_comment) = last_comment.take() {
            last_comment.lines_after = lines_before;
            self.handle_comment(last_comment, &token);
        }

        // Any comment following now is preceded by 'token' and not a node.

        // TODO: Difference to prettier:
        // - Prettier keeps the preceding node around, even if there has been a token in between
        // - Prettier keeps the following node around, even if there has been a token in between
        // - a = b; a is still the preceding even if positioned at b;
        // - same with following. It takes the first node that follows (and belongs to the same parent)
        // They then use a breakTie in situations where there are preceding and following nodes set.
        // Emphasis the importance of nodes even more. Reduces the places where dangling comments can appear
        // Has mainly become relevant for trailing comments. Is there also a noticable difference for leading?
        self.preceding_node = None;
        self.following_node = None;
        self.last_token = Some(token);
    }

    fn handle_comment(
        &mut self,
        comment: DecoratedComment<Language::SyntaxLanguage>,
        token: &SyntaxToken<Language::SyntaxLanguage>,
    ) {
        match Language::CommentStyle::position_comment(comment) {
            CommentPosition::Leading { node, comment } => {
                self.node_comments
                    .insert_leading_comment(node, comment.into());
            }
            CommentPosition::Trailing { node, comment } => {
                self.node_comments
                    .insert_trailing_comment(node, comment.into());
            }
            CommentPosition::Dangling { token, comment } => self
                .dangling_trivia
                .insert_trivia(token, DanglingTrivia::Comment(comment.into())),
            CommentPosition::Default(mut comment) => {
                if comment.is_trailing_token_trivia() {
                    let enclosing = comment.enclosing_node();

                    // The enclosing can only ever be a list if the comment is a leading or trailing comment of a
                    // separator token in a separated list.
                    // Example:
                    // ```js
                    // [
                    //   a, // test
                    //   b
                    // ]
                    // ```
                    // The default algorithm would make `// test` a leading comment of the node `b` but
                    // it should be a trailing comment of `a` because that's most likely what the user intended.
                    if enclosing.kind().is_list() && comment.lines_after() > 0 {
                        if let Some(SyntaxElement::Node(node)) =
                            comment.comment.as_piece().token().prev_sibling_or_token()
                        {
                            self.node_comments
                                .insert_trailing_comment(node, comment.into());
                            return;
                        }
                    }

                    match (comment.take_preceding_node(), comment.take_following_node()) {
                        (Some(preceding), Some(following)) => {
                            // Always attach suppression with the next node.
                            if Language::CommentStyle::is_suppression(comment.comment.text()) {
                                self.node_comments
                                    .insert_leading_comment(following, comment.into());
                            } else {
                                // Attach comments with both preceding and following node to the preceding
                                // because there's a line break separating it from the following node.
                                // ```javascript
                                // a; // comment
                                // b
                                // ```
                                self.node_comments
                                    .insert_trailing_comment(preceding, comment.into());
                            }
                        }
                        (Some(preceding), None) => {
                            self.node_comments
                                .insert_trailing_comment(preceding, comment.into());
                        }
                        (None, Some(following)) => {
                            self.node_comments
                                .insert_leading_comment(following, comment.into());
                        }
                        (None, None) => {
                            self.dangling_trivia.insert_trivia(
                                token.clone(),
                                DanglingTrivia::Comment(comment.into()),
                            );
                        }
                    }
                } else {
                    match (comment.take_following_node(), comment.take_preceding_node()) {
                        // Following always wins for a leading comment
                        // ```javascript
                        // a;
                        // // comment
                        // b
                        // ```
                        // attach the comment to the `b` expression statement
                        (Some(following), _) => {
                            self.node_comments
                                .insert_leading_comment(following, comment.into());
                        }
                        (None, Some(preceding)) => {
                            self.node_comments
                                .insert_trailing_comment(preceding, comment.into());
                        }
                        (None, None) => {
                            self.dangling_trivia.insert_trivia(
                                token.clone(),
                                DanglingTrivia::Comment(comment.into()),
                            );
                        }
                    }
                }
            }
        }
    }

    fn finish(self) -> CommentsData<Language::SyntaxLanguage> {
        let (leading_comments, trailing_comments) = self.node_comments.finish();
        let dangling_trivia = self.dangling_trivia.finish();

        CommentsData {
            is_suppression: Language::CommentStyle::is_suppression,
            leading_comments,
            trailing_comments,
            dangling_trivia,

            #[cfg(debug_assertions)]
            checked_suppressions: RefCell::default(),
        }
    }
}

// TODO necessary?
#[derive(Debug)]
struct NodeCommentsBuilder<L: Language> {
    leading_comments: AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
    trailing_comments: AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
}

impl<L: Language> NodeCommentsBuilder<L> {
    fn insert_leading_comment(&mut self, node: SyntaxNode<L>, comment: SourceComment<L>) {
        self.leading_comments.append(node, comment);
    }

    fn insert_trailing_comment(&mut self, node: SyntaxNode<L>, comment: SourceComment<L>) {
        self.trailing_comments.append(node, comment);
    }

    fn finish(
        self,
    ) -> (
        AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
        AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
    ) {
        (self.leading_comments, self.trailing_comments)
    }
}

impl<L: Language> Default for NodeCommentsBuilder<L> {
    fn default() -> Self {
        Self {
            leading_comments: AppendOnlyMultiMap::new(),
            trailing_comments: AppendOnlyMultiMap::new(),
        }
    }
}

#[derive(Debug)]
struct DanglingTriviaBuilder<L: Language> {
    trivia: AppendOnlyMultiMap<SyntaxToken<L>, DanglingTrivia<L>>,
}

impl<L: Language> DanglingTriviaBuilder<L> {
    fn insert_trivia(&mut self, token: SyntaxToken<L>, trivia: DanglingTrivia<L>) {
        self.trivia.append(token, trivia)
    }

    fn finish(self) -> AppendOnlyMultiMap<SyntaxToken<L>, DanglingTrivia<L>> {
        self.trivia
    }
}

impl<L: Language> Default for DanglingTriviaBuilder<L> {
    fn default() -> Self {
        Self {
            trivia: AppendOnlyMultiMap::new(),
        }
    }
}

/// Multimap implementation that uses a shared vector to store the values for each key.
///
/// The map uses a single vector to store the values of all keys together with a map
/// that stores the the value range for each key. The upside of using a single vector for all
/// values is that it avoids allocating a new vector for every element. The downside is that the values
/// for a key must all be appended in order.
#[derive(Clone)]
struct AppendOnlyMultiMap<K, V> {
    index: HashMap<K, ValueRange>,
    values: Vec<V>,
}

impl<K: std::hash::Hash + Eq, V> AppendOnlyMultiMap<K, V> {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            values: Vec::new(),
        }
    }

    /// Appends the `value` to the `key`'s values.
    ///
    /// # Panics
    /// If `key` is already present in the map but other keys have been inserted since it was initially inserted.
    pub fn append(&mut self, key: K, value: V) {
        if let Some(range) = self.index.get_mut(&key) {
            assert_eq!(self.values.len(), range.end());

            self.values.push(value);
            range.increment_end();
        } else {
            let range = ValueRange::single(self.values.len());
            self.values.push(value);
            self.index.insert(key, range);
        }
    }

    /// Returns an iterator over all the keys
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.index.keys()
    }

    /// Returns a slice of the values associated with `key`.
    pub fn get(&self, key: &K) -> &[V] {
        if let Some(range) = self.index.get(key) {
            &self.values[range.start()..range.end()]
        } else {
            &[]
        }
    }
}

impl<K, V> Default for AppendOnlyMultiMap<K, V> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            index: HashMap::new(),
        }
    }
}

impl<K, V> std::fmt::Debug for AppendOnlyMultiMap<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_map();

        for (key, range) in &self.index {
            builder.entry(&key, &&self.values[range.start()..range.end()]);
        }

        builder.finish()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct ValueRange {
    start: u32,
    end: u32,
}

impl ValueRange {
    fn single(position: usize) -> Self {
        Self {
            start: position as u32,
            end: (position + 1) as u32,
        }
    }

    fn start(&self) -> usize {
        self.start as usize
    }

    fn end(&self) -> usize {
        self.end as usize
    }

    fn increment_end(&mut self) {
        self.end += 1;
    }
}
