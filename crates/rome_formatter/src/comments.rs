mod builder;
mod map;

use self::{builder::CommentsBuilderVisitor, map::CommentsMap};
use rome_rowan::syntax::SyntaxElementKey;
use rome_rowan::{Language, SyntaxNode, SyntaxToken, SyntaxTriviaPieceComments};
use rustc_hash::FxHashSet;
#[cfg(debug_assertions)]
use std::cell::RefCell;
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
pub struct DecoratedComment<L: Language> {
    enclosing: SyntaxNode<L>,
    preceding: Option<SyntaxNode<L>>,
    following: Option<SyntaxNode<L>>,
    following_token: SyntaxToken<L>,
    position: CommentPosition,
    lines_before: u32,
    lines_after: u32,
    comment: SyntaxTriviaPieceComments<L>,
    kind: CommentKind,
}

impl<L: Language> DecoratedComment<L> {
    /// The node that fully encloses the comment (the comment's start and end position are fully in the
    /// node's bounds).
    pub fn enclosing_node(&self) -> &SyntaxNode<L> {
        &self.enclosing
    }

    pub fn piece(&self) -> &SyntaxTriviaPieceComments<L> {
        &self.comment
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

    /// Returns the [kind](CommentKind) of the comment.
    pub fn kind(&self) -> CommentKind {
        self.kind
    }

    pub fn position(&self) -> CommentPosition {
        self.position
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CommentPosition {
    /// A comment that is separated by at least one line break from the following token
    ///
    /// ```javascript
    /// a; /* this */ // or this
    /// b;
    EndOfLine,

    /// A Comment that is separated by at least one line break from the preceding token
    ///
    /// ```javascript
    /// a;
    /// /* comment */ /* or this */
    /// b;
    /// ```
    OwnLine,

    /// A comment that is placed on the same line as the preceding and following token.
    ///
    /// ```javascript
    /// a /* comment */ + b
    /// ```
    SameLine,
}

impl CommentPosition {
    pub const fn is_same_line(&self) -> bool {
        matches!(self, CommentPosition::SameLine)
    }

    pub const fn is_own_line(&self) -> bool {
        matches!(self, CommentPosition::OwnLine)
    }

    pub const fn is_end_of_line(&self) -> bool {
        matches!(self, CommentPosition::EndOfLine)
    }
}

#[derive(Debug)]
pub enum CommentPlacement<L: Language> {
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

    /// Makes this comment a dangling comment of `node`
    Dangling {
        node: SyntaxNode<L>,
        comment: DecoratedComment<L>,
    },

    /// Uses the default positioning rules for the comment.
    /// TODO document rules
    Default(DecoratedComment<L>),
}

impl<L: Language> CommentPlacement<L> {
    #[inline]
    pub fn or_else<F>(self, or_else: F) -> Self
    where
        F: FnOnce(DecoratedComment<L>) -> CommentPlacement<L>,
    {
        match self {
            CommentPlacement::Default(comment) => or_else(comment),
            placement => placement,
        }
    }
}

/// Defines how to format comments for a specific [Language].
pub trait CommentStyle: Default {
    type Language: Language;

    /// Returns `true` if a comment with the given `text` is a `rome-ignore format:` suppression comment.
    fn is_suppression(text: &str) -> bool;

    /// Returns the (kind)[CommentKind] of the comment
    fn get_comment_kind(&self, comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind;

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language>;
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
    pub fn from_node<Style>(root: &SyntaxNode<L>, style: &Style) -> Self
    where
        Style: CommentStyle<Language = L>,
    {
        let builder = CommentsBuilderVisitor::new(style);

        let (comments, skipped) = builder.visit(root);

        Self {
            data: Rc::new(CommentsData {
                root: Some(root.clone()),
                is_suppression: Style::is_suppression,

                comments,
                with_skipped: skipped,
                #[cfg(debug_assertions)]
                checked_suppressions: RefCell::new(Default::default()),
            }),
        }
    }

    /// Returns `true` if the given `node` has any leading or trailing comments.
    #[inline]
    pub fn has_comments(&self, node: &SyntaxNode<L>) -> bool {
        self.data.comments.has(&node.key())
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

    /// Returns the [node]'s leading comments.
    #[inline]
    pub fn leading_comments(&self, node: &SyntaxNode<L>) -> &[SourceComment<L>] {
        self.data.comments.leading(&node.key())
    }

    /// Returns `true` if node has any dangling comments.
    pub fn has_dangling_comments(&self, node: &SyntaxNode<L>) -> bool {
        !self.dangling_comments(node).is_empty()
    }

    /// Returns the dangling comments of `node`
    pub fn dangling_comments(&self, node: &SyntaxNode<L>) -> &[SourceComment<L>] {
        self.data.comments.dangling(&node.key())
    }

    /// Returns the [node]'s trailing comments.
    #[inline]
    pub fn trailing_comments(&self, node: &SyntaxNode<L>) -> &[SourceComment<L>] {
        self.data.comments.trailing(&node.key())
    }

    /// Returns `true` if the given [node] has any trailing comments.
    /// By default, a comment is a node's trailing comment if:
    /// * the next sibling is a token
    /// * there's **no** line break between the node and this comment.
    #[inline]
    pub fn has_trailing_comments(&self, node: &SyntaxNode<L>) -> bool {
        !self.trailing_comments(node).is_empty()
    }

    /// Returns an iterator over the leading and trailing comments of `node`.
    pub fn leading_trailing_comments(
        &self,
        node: &SyntaxNode<L>,
    ) -> impl Iterator<Item = &SourceComment<L>> {
        self.leading_comments(node)
            .iter()
            .chain(self.trailing_comments(node).iter())
    }

    /// Returns an iterator over the leading, dangling, and trailing comments of `node`.
    pub fn leading_dangling_trailing_comments<'a>(
        &'a self,
        node: &'a SyntaxNode<L>,
    ) -> impl Iterator<Item = &SourceComment<L>> + 'a {
        self.data.comments.parts(&node.key())
    }

    /// Returns `true` if that node has skipped token trivia attached.
    #[inline]
    pub fn has_skipped(&self, token: &SyntaxToken<L>) -> bool {
        self.data.with_skipped.contains(&token.key())
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

        self.leading_dangling_trailing_comments(node)
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
                use rome_rowan::SyntaxKind;

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
    root: Option<SyntaxNode<L>>,

    is_suppression: fn(&str) -> bool,

    /// Stores all leading node comments by node
    comments: CommentsMap<SyntaxElementKey, SourceComment<L>>,
    with_skipped: FxHashSet<SyntaxElementKey>,

    /// Stores all nodes for which [Comments::is_suppressed] has been called.
    /// This index of nodes that have been checked if they have a suppression comments is used to
    /// detect format implementations that manually format a child node without previously checking if
    /// the child has a suppression comment.
    ///
    /// The implementation refrains from snapshotting the checked nodes because a node gets formatted
    /// as verbatim if its formatting fails which has the same result as formatting it as suppressed node
    /// (thus, guarantees that the formatting isn't changed).
    #[cfg(debug_assertions)]
    checked_suppressions: RefCell<FxHashSet<SyntaxNode<L>>>,
}

impl<L: Language> Default for CommentsData<L> {
    fn default() -> Self {
        Self {
            root: None,
            is_suppression: |_| false,
            comments: Default::default(),
            with_skipped: Default::default(),
            #[cfg(debug_assertions)]
            checked_suppressions: Default::default(),
        }
    }
}

impl<L: Language> std::fmt::Debug for CommentsData<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();

        if let Some(root) = &self.root {
            for node in root.descendants() {
                for leading in self.comments.leading(&node.key()) {
                    list.entry(&DebugComment::Leading {
                        node: &node,
                        comment: leading,
                    });
                }

                for dangling in self.comments.dangling(&node.key()) {
                    list.entry(&DebugComment::Dangling {
                        node: &node,
                        comment: dangling,
                    });
                }

                for trailing in self.comments.trailing(&node.key()) {
                    list.entry(&DebugComment::Trailing {
                        node: &node,
                        comment: trailing,
                    });
                }
            }
        }

        list.finish()
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
        comment: &'a SourceComment<L>,
        node: &'a SyntaxNode<L>,
    },
}

impl<L: Language> std::fmt::Debug for DebugComment<'_, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebugComment::Leading { node, comment } => f
                .debug_struct("Leading")
                .field("node", node)
                .field("comment", comment)
                .finish(),
            DebugComment::Dangling { node, comment } => f
                .debug_struct("Dangling")
                .field("node", node)
                .field("comment", comment)
                .finish(),
            DebugComment::Trailing { node, comment } => f
                .debug_struct("Trailing")
                .field("node", node)
                .field("comment", comment)
                .finish(),
        }
    }
}
