use crate::CstFormatContext;
use rome_rowan::{
    Direction, Language, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxTriviaPieceComments,
    WalkEvent,
};
#[cfg(debug_assertions)]
use std::cell::RefCell;
use std::collections::HashSet;

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

#[derive(Debug, Clone)]
pub struct SourceComment<L: Language> {
    /// The number of lines appearing before this comment
    lines_before: u32,

    /// The comment piece
    piece: SyntaxTriviaPieceComments<L>,
}

impl<L: Language> SourceComment<L> {
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

/// Defines how to format comments for a specific [Language].
pub trait CommentStyle<L: Language> {
    /// Returns `true` if a comment with the given `text` is a `rome-ignore format:` suppression comment.
    fn is_suppression(&self, text: &str) -> bool;

    /// Returns the (kind)[CommentKind] of the comment
    fn get_comment_kind(&self, comment: &SyntaxTriviaPieceComments<L>) -> CommentKind;

    /// Returns `true` if a token with the passed `kind` marks the start of a group. Common group tokens are:
    /// * left parentheses: `(`, `[`, `{`
    fn is_group_start_token(&self, kind: L::Kind) -> bool;

    /// Returns `true` if a token with the passed `kind` marks the end of a group. Common group end tokens are:
    /// * right parentheses: `)`, `]`, `}`
    /// * end of statement token: `;`
    /// * element separator: `,` or `.`.
    /// * end of file token: `EOF`
    fn is_group_end_token(&self, kind: L::Kind) -> bool;
}

/// Type that stores the comments of a tree and gives access to:
///
/// * whether a node should be formatted as is because it has a leading suppression comment.
/// * a node's leading and trailing comments
/// * the dangling comments of a token
#[derive(Debug, Default, Clone)]
pub struct Comments<L: Language> {
    /// Stores the nodes that have at least one leading suppression comment.
    suppressed_nodes: HashSet<SyntaxNode<L>>,

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

impl<L: Language> Comments<L> {
    /// Extracts all the suppressions from `root` and its child nodes.
    pub fn from_node<Context>(root: &SyntaxNode<L>, context: &Context) -> Self
    where
        Context: CstFormatContext<Language = L>,
    {
        let mut suppressed_nodes = HashSet::new();
        let mut current_node = None;

        for event in root.preorder_with_tokens(Direction::Next) {
            match event {
                WalkEvent::Enter(SyntaxElement::Node(node)) => {
                    // Lists cannot have a suppression comment attached, it must
                    // belong to either the entire parent node or one of the children
                    if node.kind().is_root() || node.kind().is_list() {
                        continue;
                    }

                    if current_node.is_none() {
                        current_node = Some(node);
                    }
                }
                WalkEvent::Leave(SyntaxElement::Node(node)) => {
                    if current_node == Some(node) {
                        current_node = None;
                    }
                }
                WalkEvent::Enter(SyntaxElement::Token(token)) => {
                    if let Some(current_node) = current_node.take() {
                        for comment in token
                            .leading_trivia()
                            .pieces()
                            .filter_map(|piece| piece.as_comments())
                        {
                            if context.comment_style().is_suppression(comment.text()) {
                                suppressed_nodes.insert(current_node);
                                break;
                            }
                        }
                    }
                }
                WalkEvent::Leave(SyntaxElement::Token(_)) => {
                    // Token already handled as part of the enter event.
                }
            }
        }

        Self {
            suppressed_nodes,
            #[cfg(debug_assertions)]
            checked_suppressions: RefCell::default(),
        }
    }

    /// Returns `true` if the passed `node` has a leading suppression comment.
    ///
    /// Suppression comments only apply if they are at the start of a node and they suppress the most
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
        self.suppressed_nodes.contains(node)
    }

    /// Marks that it isn't necessary for the given node to check if it has been suppressed or not.
    #[inline]
    pub fn mark_suppression_checked(&self, node: &SyntaxNode<L>) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                let mut checked_nodes = self.checked_suppressions.borrow_mut();
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
                let checked_nodes = self.checked_suppressions.borrow();
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
