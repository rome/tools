use fxhash::FxHashMap;
use smallvec::SmallVec;

use crate::{
	green::{GreenElement, GreenNode, GreenToken, SyntaxKind},
	NodeOrToken, SmolStr,
};

use super::{node::GreenNodeHead, token::GreenTokenData};

#[derive(Default, Debug)]
pub struct NodeCache {
	nodes: FxHashMap<GreenNodeHead, GreenNode>,
	tokens: FxHashMap<GreenTokenData, GreenToken>,
}

impl NodeCache {
	fn node<I>(&mut self, kind: SyntaxKind, children: I) -> GreenNode
	where
		I: IntoIterator<Item = GreenElement>,
		I::IntoIter: ExactSizeIterator,
	{
		let children = children.into_iter();
		// Green nodes are fully immutable, so it's ok to deduplicate them.
		// This is the same optimization that Roslyn does
		// https://github.com/KirillOsenkov/Bliki/wiki/Roslyn-Immutable-Trees
		//
		// For example, all `#[inline]` in this file share the same green node!
		// For `libsyntax/parse/parser.rs`, measurements show that deduping saves
		// 17% of the memory for green nodes!
		if children.len() <= 3 {
			let children: SmallVec<[_; 3]> = children.collect();
			let head = GreenNodeHead::from_child_slice(kind, children.as_ref());
			self.nodes
				.entry(head.clone())
				.or_insert_with(|| GreenNode::from_head_and_children(head, children))
				.clone()
		} else {
			GreenNode::new(kind, children)
		}
	}

	fn token(&mut self, kind: SyntaxKind, text: SmolStr) -> GreenToken {
		let data = GreenTokenData { kind, text };
		self.tokens
			.entry(data.clone())
			.or_insert_with(|| {
				let GreenTokenData { kind, text } = data;
				GreenToken::new(kind, text)
			})
			.clone()
	}
}

#[derive(Debug)]
enum MaybeOwned<'a, T> {
	Owned(T),
	Borrowed(&'a mut T),
}

impl<T> std::ops::Deref for MaybeOwned<'_, T> {
	type Target = T;

	fn deref(&self) -> &T {
		match self {
			MaybeOwned::Owned(it) => it,
			MaybeOwned::Borrowed(it) => *it,
		}
	}
}

impl<T> std::ops::DerefMut for MaybeOwned<'_, T> {
	fn deref_mut(&mut self) -> &mut T {
		match self {
			MaybeOwned::Owned(it) => it,
			MaybeOwned::Borrowed(it) => *it,
		}
	}
}

impl<T: Default> Default for MaybeOwned<'_, T> {
	fn default() -> Self {
		MaybeOwned::Owned(T::default())
	}
}

/// A checkpoint for maybe wrapping a node. See `GreenNodeBuilder::checkpoint` for details.
#[derive(Clone, Copy, Debug)]
pub struct Checkpoint(usize);

/// A builder for a green tree.
#[derive(Default, Debug)]
pub struct GreenNodeBuilder<'cache> {
	cache: MaybeOwned<'cache, NodeCache>,
	parents: Vec<(SyntaxKind, usize)>,
	children: Vec<GreenElement>,
}

impl GreenNodeBuilder<'_> {
	/// Creates new builder.
	pub fn new() -> GreenNodeBuilder<'static> {
		GreenNodeBuilder::default()
	}

	/// Reusing `NodeCache` between different `GreenNodeBuilder`s saves memory.
	/// It allows to structurally share underlying trees.
	pub fn with_cache(cache: &mut NodeCache) -> GreenNodeBuilder<'_> {
		GreenNodeBuilder {
			cache: MaybeOwned::Borrowed(cache),
			parents: Vec::new(),
			children: Vec::new(),
		}
	}

	/// Adds new token to the current branch.
	#[inline]
	pub fn token(&mut self, kind: SyntaxKind, text: SmolStr) {
		let token = self.cache.token(kind, text);
		self.children.push(token.into());
	}

	/// Start new node and make it current.
	#[inline]
	pub fn start_node(&mut self, kind: SyntaxKind) {
		let len = self.children.len();
		self.parents.push((kind, len));
	}

	/// Finish current branch and restore previous
	/// branch as current.
	#[inline]
	pub fn finish_node(&mut self) {
		let (kind, first_child) = self.parents.pop().unwrap();
		let children = self.children.drain(first_child..);
		let node = self.cache.node(kind, children);
		self.children.push(node.into());
	}

	/// Prepare for maybe wrapping the next node.
	/// The way wrapping works is that you first of all get a checkpoint,
	/// then you place all tokens you want to wrap, and then *maybe* call
	/// `start_node_at`.
	/// Example:
	/// ```rust
	/// # use rslint_rowan::{GreenNodeBuilder, SyntaxKind};
	/// # const PLUS: SyntaxKind = SyntaxKind(0);
	/// # const OPERATION: SyntaxKind = SyntaxKind(1);
	/// # struct Parser;
	/// # impl Parser {
	/// #     fn peek(&self) -> Option<SyntaxKind> { None }
	/// #     fn parse_expr(&mut self) {}
	/// # }
	/// # let mut builder = GreenNodeBuilder::new();
	/// # let mut parser = Parser;
	/// let checkpoint = builder.checkpoint();
	/// parser.parse_expr();
	/// if parser.peek() == Some(PLUS) {
	///     // 1 + 2 = Add(1, 2)
	///     builder.start_node_at(checkpoint, OPERATION);
	///     parser.parse_expr();
	///     builder.finish_node();
	/// }
	/// ```
	#[inline]
	pub fn checkpoint(&self) -> Checkpoint {
		Checkpoint(self.children.len())
	}

	/// Wrap the previous branch marked by `checkpoint` in a new branch and
	/// make it current.
	#[inline]
	pub fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
		let Checkpoint(checkpoint) = checkpoint;
		assert!(
			checkpoint <= self.children.len(),
			"checkpoint no longer valid, was finish_node called early?"
		);

		if let Some(&(_, first_child)) = self.parents.last() {
			assert!(
				checkpoint >= first_child,
				"checkpoint no longer valid, was an unmatched start_node_at called?"
			);
		}

		self.parents.push((kind, checkpoint));
	}

	/// Complete tree building. Make sure that
	/// `start_node_at` and `finish_node` calls
	/// are paired!
	#[inline]
	pub fn finish(mut self) -> GreenNode {
		assert_eq!(self.children.len(), 1);
		match self.children.pop().unwrap() {
			NodeOrToken::Node(node) => node,
			NodeOrToken::Token(_) => panic!(),
		}
	}
}
