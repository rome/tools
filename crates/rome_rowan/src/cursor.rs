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

use std::{
    cell::Cell,
    mem::{self, ManuallyDrop},
    ptr,
};

use countme::Count;
pub(crate) use trivia::{SyntaxTrivia, SyntaxTriviaPiecesIterator};

use crate::cursor::node::Siblings;
pub(crate) use crate::cursor::token::SyntaxToken;
use crate::{
    green::{GreenElementRef, GreenNodeData, GreenTokenData, RawSyntaxKind},
    sll,
    utility_types::Delta,
    GreenNode, GreenToken, NodeOrToken, TextRange, TextSize,
};
pub(crate) use element::SyntaxElement;
pub(crate) use node::{
    Preorder, PreorderWithTokens, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren,
    SyntaxSlot, SyntaxSlots,
};

enum GreenElement {
    Node {
        ptr: Cell<ptr::NonNull<GreenNodeData>>,
    },
    Token {
        ptr: ptr::NonNull<GreenTokenData>,
    },
}

struct _SyntaxElement;

struct NodeData {
    _c: Count<_SyntaxElement>,

    rc: Cell<u32>,
    parent: Cell<Option<ptr::NonNull<NodeData>>>,
    slot: Cell<u32>,
    green: GreenElement,

    /// Invariant: never changes after NodeData is created.
    mutable: bool,
    /// Absolute offset for immutable nodes, unused for mutable nodes.
    offset: TextSize,
    // The following links only have meaning when `mutable` is true.
    first: Cell<*const NodeData>,
    /// Invariant: never null if mutable.
    next: Cell<*const NodeData>,
    /// Invariant: never null if mutable.
    prev: Cell<*const NodeData>,
}

unsafe impl sll::Elem for NodeData {
    fn prev(&self) -> &Cell<*const Self> {
        &self.prev
    }
    fn next(&self) -> &Cell<*const Self> {
        &self.next
    }
    fn key(&self) -> &Cell<u32> {
        &self.slot
    }
}

#[inline(never)]
unsafe fn free(mut data: ptr::NonNull<NodeData>) {
    loop {
        debug_assert_eq!(data.as_ref().rc.get(), 0);
        debug_assert!(data.as_ref().first.get().is_null());
        let node = Box::from_raw(data.as_ptr());
        match node.parent.take() {
            Some(parent) => {
                debug_assert!(parent.as_ref().rc.get() > 0);
                if node.mutable {
                    sll::unlink(&parent.as_ref().first, &*node)
                }
                if parent.as_ref().dec_rc() {
                    data = parent;
                } else {
                    break;
                }
            }
            None => {
                match &node.green {
                    GreenElement::Node { ptr } => {
                        let _ = GreenNode::from_raw(ptr.get());
                    }
                    GreenElement::Token { ptr } => {
                        let _ = GreenToken::from_raw(*ptr);
                    }
                }
                break;
            }
        }
    }
}

impl NodeData {
    #[inline]
    fn new(
        parent: Option<SyntaxNode>,
        slot: u32,
        offset: TextSize,
        green: GreenElement,
        mutable: bool,
    ) -> ptr::NonNull<NodeData> {
        let parent = ManuallyDrop::new(parent);
        let res = NodeData {
            _c: Count::new(),
            rc: Cell::new(1),
            parent: Cell::new(parent.as_ref().map(|it| it.ptr)),
            slot: Cell::new(slot),
            green,

            mutable,
            offset,
            first: Cell::new(ptr::null()),
            next: Cell::new(ptr::null()),
            prev: Cell::new(ptr::null()),
        };
        unsafe {
            if mutable {
                let res_ptr: *const NodeData = &res;
                match sll::init(
                    (*res_ptr).parent().map(|it| &it.first),
                    res_ptr.as_ref().unwrap(),
                ) {
                    sll::AddToSllResult::AlreadyInSll(node) => {
                        if cfg!(debug_assertions) {
                            assert_eq!((*node).slot(), (*res_ptr).slot());
                            match ((*node).green(), (*res_ptr).green()) {
                                (NodeOrToken::Node(lhs), NodeOrToken::Node(rhs)) => {
                                    assert!(ptr::eq(lhs, rhs))
                                }
                                (NodeOrToken::Token(lhs), NodeOrToken::Token(rhs)) => {
                                    assert!(ptr::eq(lhs, rhs))
                                }
                                it => {
                                    panic!("node/token confusion: {:?}", it)
                                }
                            }
                        }

                        ManuallyDrop::into_inner(parent);
                        let res = node as *mut NodeData;
                        (*res).inc_rc();
                        return ptr::NonNull::new_unchecked(res);
                    }
                    it => {
                        let res = Box::into_raw(Box::new(res));
                        it.add_to_sll(res);
                        return ptr::NonNull::new_unchecked(res);
                    }
                }
            }
            ptr::NonNull::new_unchecked(Box::into_raw(Box::new(res)))
        }
    }

    #[inline]
    fn inc_rc(&self) {
        let rc = match self.rc.get().checked_add(1) {
            Some(it) => it,
            None => std::process::abort(),
        };
        self.rc.set(rc)
    }

    #[inline]
    fn dec_rc(&self) -> bool {
        let rc = self.rc.get() - 1;
        self.rc.set(rc);
        rc == 0
    }

    #[inline]
    fn key(&self) -> (ptr::NonNull<()>, TextSize) {
        let ptr = match &self.green {
            GreenElement::Node { ptr } => ptr.get().cast(),
            GreenElement::Token { ptr } => ptr.cast(),
        };
        (ptr, self.offset())
    }

    #[inline]
    fn parent_node(&self) -> Option<SyntaxNode> {
        let parent = self.parent()?;
        debug_assert!(matches!(parent.green, GreenElement::Node { .. }));
        parent.inc_rc();
        Some(SyntaxNode {
            ptr: ptr::NonNull::from(parent),
        })
    }

    #[inline]
    fn parent(&self) -> Option<&NodeData> {
        self.parent.get().map(|it| unsafe { &*it.as_ptr() })
    }

    #[inline]
    fn green(&self) -> GreenElementRef<'_> {
        match &self.green {
            GreenElement::Node { ptr } => GreenElementRef::Node(unsafe { &*ptr.get().as_ptr() }),
            GreenElement::Token { ptr } => GreenElementRef::Token(unsafe { &*ptr.as_ref() }),
        }
    }

    /// Returns an iterator over the siblings of this node. The iterator is positioned at the current node.
    #[inline]
    fn green_siblings(&self) -> Option<Siblings> {
        match &self.parent()?.green {
            GreenElement::Node { ptr } => {
                let parent = unsafe { &*ptr.get().as_ptr() };

                Some(Siblings::new(parent, self.slot()))
            }
            GreenElement::Token { .. } => {
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
        self.slot.get()
    }

    #[inline]
    fn offset(&self) -> TextSize {
        if self.mutable {
            self.offset_mut()
        } else {
            self.offset
        }
    }

    #[cold]
    fn offset_mut(&self) -> TextSize {
        let mut res = TextSize::from(0);

        let mut node = self;
        while let Some(parent) = node.parent() {
            let green = parent.green().into_node().unwrap();
            res += green
                .slots()
                .nth(node.slot() as usize)
                .unwrap()
                .rel_offset();
            node = parent;
        }

        res
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

    fn detach(&self) {
        assert!(self.mutable);
        assert!(self.rc.get() > 0);
        let parent_ptr = match self.parent.take() {
            Some(parent) => parent,
            None => return,
        };

        unsafe {
            sll::adjust(self, self.slot() + 1, Delta::Sub(1));
            let parent = parent_ptr.as_ref();
            sll::unlink(&parent.first, self);

            // Add strong ref to green
            match self.green().to_owned() {
                NodeOrToken::Node(it) => {
                    GreenNode::into_raw(it);
                }
                NodeOrToken::Token(it) => {
                    GreenToken::into_raw(it);
                }
            }

            match parent.green() {
                NodeOrToken::Node(green) => {
                    let green = green.remove_slot(self.slot() as usize);
                    parent.respine(green)
                }
                NodeOrToken::Token(_) => unreachable!(),
            }

            if parent.dec_rc() {
                free(parent_ptr)
            }
        }
    }
    fn attach_child(&self, index: usize, child: &NodeData) {
        assert!(self.mutable && child.mutable && child.parent().is_none());
        assert!(self.rc.get() > 0 && child.rc.get() > 0);

        unsafe {
            child.slot.set(index as u32);
            child.parent.set(Some(self.into()));
            self.inc_rc();

            if !self.first.get().is_null() {
                sll::adjust(&*self.first.get(), index as u32, Delta::Add(1));
            }

            match sll::link(&self.first, child) {
                sll::AddToSllResult::AlreadyInSll(_) => {
                    panic!("Child already in sorted linked list")
                }
                it => it.add_to_sll(child),
            }

            match self.green() {
                NodeOrToken::Node(green) => {
                    // Child is root, so it owns the green node. Steal it!
                    let child_green = match &child.green {
                        GreenElement::Node { ptr } => GreenNode::from_raw(ptr.get()).into(),
                        GreenElement::Token { ptr } => GreenToken::from_raw(*ptr).into(),
                    };

                    let green = green.insert_slot(index, Some(child_green));
                    self.respine(green);
                }
                NodeOrToken::Token(_) => unreachable!(),
            }
        }
    }
    unsafe fn respine(&self, mut new_green: GreenNode) {
        let mut node = self;
        loop {
            let old_green = match &node.green {
                GreenElement::Node { ptr } => ptr.replace(ptr::NonNull::from(&*new_green)),
                GreenElement::Token { .. } => unreachable!(),
            };
            match node.parent() {
                Some(parent) => match parent.green() {
                    NodeOrToken::Node(parent_green) => {
                        new_green = parent_green
                            .replace_child(node.slot() as usize, Some(new_green.into()));
                        node = parent;
                    }
                    _ => unreachable!(),
                },
                None => {
                    mem::forget(new_green);
                    let _ = GreenNode::from_raw(old_green);
                    break;
                }
            }
        }
    }
}
