use crate::{
	cow_mut::CowMut,
	green::{GreenElement, NodeCache, SyntaxKind},
	Language, NodeOrToken, SyntaxNode,
};

/// A checkpoint for maybe wrapping a node. See `GreenNodeBuilder::checkpoint` for details.
#[derive(Clone, Copy, Debug)]
pub struct Checkpoint(usize);

/// A builder for a syntax tree.
#[derive(Default, Debug)]
pub struct TreeBuilder<'cache> {
	cache: CowMut<'cache, NodeCache>,
	parents: Vec<(SyntaxKind, usize)>,
	children: Vec<(u64, GreenElement)>,
}

impl TreeBuilder<'_> {
	/// Creates new builder.
	pub fn new() -> TreeBuilder<'static> {
		TreeBuilder::default()
	}

	/// Reusing `NodeCache` between different [TreeBuilder]`s saves memory.
	/// It allows to structurally share underlying trees.
	pub fn with_cache(cache: &mut NodeCache) -> TreeBuilder<'_> {
		TreeBuilder {
			cache: CowMut::Borrowed(cache),
			parents: Vec::new(),
			children: Vec::new(),
		}
	}

	/// Adds new token to the current branch.
	#[inline]
	pub fn token(&mut self, kind: SyntaxKind, text: &str) {
		let (hash, token) = self.cache.token(kind, text);
		self.children.push((hash, token.into()));
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
		let (hash, node) = self.cache.node(kind, &mut self.children, first_child);
		self.children.push((hash, node.into()));
	}

	/// Prepare for maybe wrapping the next node.
	/// The way wrapping works is that you first of all get a checkpoint,
	/// then you place all tokens you want to wrap, and then *maybe* call
	/// `start_node_at`.
	/// Example:
	/// ```rust
	/// # use rome_rowan::{TreeBuilder, SyntaxKind};
	/// # const PLUS: SyntaxKind = SyntaxKind(0);
	/// # const OPERATION: SyntaxKind = SyntaxKind(1);
	/// # struct Parser;
	/// # impl Parser {
	/// #     fn peek(&self) -> Option<SyntaxKind> { None }
	/// #     fn parse_expr(&mut self) {}
	/// # }
	/// # let mut builder = TreeBuilder::new();
	/// # let mut parser = Parser;
	/// let checkpoint = builder.checkpoint();
	/// parser.parse_expr();
	/// if parser.peek() == Some(PLUS) {
	///   // 1 + 2 = Add(1, 2)
	///   builder.start_node_at(checkpoint, OPERATION);
	///   parser.parse_expr();
	///   builder.finish_node();
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
	pub fn finish<L: Language>(mut self) -> SyntaxNode<L> {
		assert_eq!(self.children.len(), 1);
		match self.children.pop().unwrap().1 {
			NodeOrToken::Node(node) => SyntaxNode::new_root(node),
			NodeOrToken::Token(_) => panic!(),
		}
	}
}
