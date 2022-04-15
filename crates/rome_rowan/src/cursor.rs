//! Implementation of the cursors -- API for convenient access to syntax trees.
//!
//! Functional programmers will recognize that this module implements a zipper
//! for a purely functional (green) tree.
//!
//! A cursor node (`SyntaxNode`) points to a `GreenNode` and a parent
//! `SyntaxNode`. This allows cursor to provide iteration over both ancestors
//! and descendants, as well as a cheep access to absolute offset of the node in
//! file.
//!
//! By default `SyntaxNode`s are immutable, but you can get a mutable copy of
//! the tree by calling `clone_for_update`. Mutation is based on interior
//! mutability and doesn't need `&mut`. You can have two `SyntaxNode`s pointing
//! at different parts of the same tree; mutations via the first node will be
//! reflected in the other.

// Implementation notes:
//
// The implementation is utterly and horribly unsafe. This whole module is an
// unsafety boundary. It is believed that the API here is, in principle, sound,
// but the implementation might have bugs.
//
// The core type is `NodeData` -- a heap-allocated reference counted object,
// which points to a green node or a green token, and to the parent `NodeData`.
// Publicly-exposed `SyntaxNode` and `SyntaxToken` own a reference to
// `NodeData`.
//
// `NodeData`s are transient, and are created and destroyed during tree
// traversals. In general, only currently referenced nodes and their ancestors
// are alive at any given moment.
//
// More specifically, `NodeData`'s ref count is equal to the number of
// outstanding `SyntaxNode` and `SyntaxToken` plus the number of children with
// non-zero ref counts. For example, if the user has only a single `SyntaxNode`
// pointing somewhere in the middle of the tree, then all `NodeData` on the path
// from that point towards the root have ref count equal to one.
//
// `NodeData` which doesn't have a parent (is a root) owns the corresponding
// green node or token, and is responsible for freeing it.
//
// That's mostly it for the immutable subset of the API. Mutation is fun though,
// you'll like it!
//
// Mutability is a run-time property of a tree of `NodeData`. The whole tree is
// either mutable or immutable. `clone_for_update` clones the whole tree of
// `NodeData`s, making it mutable (note that the green tree is re-used).
//
// If the tree is mutable, then all live `NodeData` are additionally liked to
// each other via intrusive liked lists. Specifically, there are two pointers to
// siblings, as well as a pointer to the first child. Note that only live nodes
// are considered. If the user only has `SyntaxNode`s for  the first and last
// children of some particular node, then their `NodeData` will point at each
// other.
//
// The links are used to propagate mutations across the tree. Specifically, each
// `NodeData` remembers it's index in parent. When the node is detached from or
// attached to the tree, we need to adjust the indices of all subsequent
// siblings. That's what makes the `for c in node.children() { c.detach() }`
// pattern work despite the apparent iterator invalidation.
//
// This code is encapsulated into the sorted linked list (`sll`) module.
//
// The actual mutation consist of functionally "mutating" (creating a
// structurally shared copy) the green node, and then re-spinning the tree. This
// is a delicate process: `NodeData` point directly to the green nodes, so we
// must make sure that those nodes don't move. Additionally, during mutation a
// node might become or might stop being a root, so we must take care to not
// double free / leak its green node.
//
// Because we can change green nodes using only shared references, handing out
// references into green nodes in the public API would be unsound. We don't do
// that, but we do use such references internally a lot. Additionally, for
// tokens the underlying green token actually is immutable, so we can, and do
// return `&str`.
//
// Invariants [must not leak outside of the module]:
//    - Mutability is the property of the whole tree. Intermixing elements that
//      differ in mutability is not allowed.
//    - Mutability property is persistent.
//    - References to the green elements' data are not exposed into public API
//      when the tree is mutable.
//    - TBD

mod element;
mod node;
mod token;
mod trivia;

use std::{iter, ops};
use std::{ptr, rc::Rc};

use countme::Count;
pub(crate) use trivia::{SyntaxTrivia, SyntaxTriviaPiecesIterator};

pub(crate) use crate::cursor::token::SyntaxToken;
use crate::{cursor::node::Siblings, green::GreenElement};
use crate::{
    green::{GreenElementRef, RawSyntaxKind},
    NodeOrToken, TextRange, TextSize,
};
pub(crate) use element::SyntaxElement;
pub(crate) use node::{
    Preorder, PreorderWithTokens, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren,
    SyntaxSlot, SyntaxSlots,
};

#[derive(Debug)]
struct _SyntaxElement;

pub(crate) fn has_live() -> bool {
    countme::get::<_SyntaxElement>().live > 0
}

#[derive(Debug)]
struct NodeData {
    _c: Count<_SyntaxElement>,

    parent: Option<Rc<NodeData>>,
    slot: u32,
    green: GreenElement,

    /// Absolute offset for immutable nodes, unused for mutable nodes.
    offset: TextSize,
}

impl NodeData {
    #[inline]
    fn new(
        parent: Option<Rc<NodeData>>,
        slot: u32,
        offset: TextSize,
        green: GreenElement,
    ) -> Rc<NodeData> {
        let res = NodeData {
            _c: Count::new(),
            parent,
            slot,
            green,
            offset,
        };

        Rc::new(res)
    }

    #[inline]
    fn key(&self) -> (ptr::NonNull<()>, TextSize) {
        let ptr = match &self.green {
            GreenElement::Node(ptr) => ptr::NonNull::from(&**ptr).cast(),
            GreenElement::Token(ptr) => ptr::NonNull::from(&**ptr).cast(),
        };
        (ptr, self.offset())
    }

    #[inline]
    fn parent_node(&self) -> Option<SyntaxNode> {
        debug_assert!(matches!(self.parent()?.green, GreenElement::Node { .. }));
        Some(SyntaxNode {
            ptr: self.parent.as_ref()?.clone(),
        })
    }

    #[inline]
    fn parent(&self) -> Option<&NodeData> {
        self.parent.as_deref()
    }

    #[inline]
    fn green(&self) -> GreenElementRef<'_> {
        match &self.green {
            GreenElement::Node(ptr) => GreenElementRef::Node(&*ptr),
            GreenElement::Token(ptr) => GreenElementRef::Token(&*ptr),
        }
    }

    /// Returns an iterator over the siblings of this node. The iterator is positioned at the current node.
    #[inline]
    fn green_siblings(&self) -> Option<Siblings> {
        match &self.parent()?.green {
            GreenElement::Node(ptr) => Some(Siblings::new(&*ptr, self.slot())),
            GreenElement::Token(_) => {
                debug_assert!(
                    false,
                    "A token should never be a parent of a token or node."
                );
                None
            }
        }
    }
    #[inline]
    fn slot(&self) -> u32 {
        self.slot
    }

    #[inline]
    fn offset(&self) -> TextSize {
        self.offset
    }

    #[inline]
    fn text_range(&self) -> TextRange {
        let offset = self.offset();
        let len = self.green().text_len();
        TextRange::at(offset, len)
    }

    #[inline]
    fn kind(&self) -> RawSyntaxKind {
        self.green().kind()
    }

    fn next_sibling(&self) -> Option<SyntaxNode> {
        let siblings = self.green_siblings()?;
        siblings.following().find_map(|child| {
            child.element().into_node().and_then(|green| {
                let parent = self.parent_node()?;
                let offset = parent.offset() + child.rel_offset();
                Some(SyntaxNode::new_child(green, parent, child.slot(), offset))
            })
        })
    }
    fn prev_sibling(&self) -> Option<SyntaxNode> {
        let siblings = self.green_siblings()?;
        siblings.previous().find_map(|child| {
            child.element().into_node().and_then(|green| {
                let parent = self.parent_node()?;
                let offset = parent.offset() + child.rel_offset();
                Some(SyntaxNode::new_child(green, parent, child.slot(), offset))
            })
        })
    }

    fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        let siblings = self.green_siblings()?;

        siblings.following().next().and_then(|child| {
            let parent = self.parent_node()?;
            let offset = parent.offset() + child.rel_offset();
            Some(SyntaxElement::new(
                child.element(),
                parent,
                child.slot(),
                offset,
            ))
        })
    }
    fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        let siblings = self.green_siblings()?;

        siblings.previous().next().and_then(|child| {
            let parent = self.parent_node()?;
            let offset = parent.offset() + child.rel_offset();
            Some(SyntaxElement::new(
                child.element(),
                parent,
                child.slot(),
                offset,
            ))
        })
    }

    /// Return a clone of this subtree detached from its parent
    #[must_use]
    fn detach(self: Rc<Self>) -> Rc<Self> {
        match self.parent.is_some() {
            true => Self::new(None, 0, 0.into(), self.green().to_owned()),
            // If this node is already detached, increment the reference count and return a clone
            false => self.clone(),
        }
    }

    /// Return a clone of this node with the specified range of slots replaced
    /// with the elements of the provided iterator
    #[must_use]
    fn splice_slots<R, I>(self: Rc<Self>, range: R, replace_with: I) -> Rc<Self>
    where
        R: ops::RangeBounds<usize>,
        I: Iterator<Item = Option<GreenElement>>,
    {
        let new_green = match self.green() {
            NodeOrToken::Node(green) => GreenElement::Node(green.splice_slots(range, replace_with)),
            NodeOrToken::Token(_) => unreachable!(),
        };

        // If the reference count of self is 1, recycle the NodeData in place,
        // otherwise create a new clone of the data
        //
        // This is similar to Rc::make_mut, but that function can't be called
        // directly since NodeData doesn't implement Clone
        let mut node = match Rc::try_unwrap(self) {
            Ok(mut node) => {
                node.green = new_green.clone();
                node
            }
            Err(ptr) => NodeData {
                _c: Count::new(),
                parent: ptr.parent.clone(),
                slot: ptr.slot,
                green: new_green.clone(),
                offset: ptr.offset,
            },
        };

        node.parent = match node.parent {
            Some(parent) => {
                // SAFETY: This conversion can only fail on 16-bits systems for nodes with more than 65 535 children
                let index = usize::try_from(node.slot).expect("integer overflow");

                let range = index..=index;
                let replace_with = iter::once(Some(new_green));
                let parent = parent.splice_slots(range, replace_with);

                Some(parent)
            }
            None => None,
        };

        Rc::new(node)
    }
}
