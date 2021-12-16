use crate::api::SyntaxKind;
use crate::{
	api::TriviaPiece,
	cow_mut::CowMut,
	green::{GreenElement, NodeCache},
	AstTreeShape, GreenNode, NodeOrToken, SyntaxNode,
};

/// A checkpoint for maybe wrapping a node. See `GreenNodeBuilder::checkpoint` for details.
#[derive(Clone, Copy, Debug)]
pub struct Checkpoint(usize);

/// A builder for a syntax tree.
#[derive(Debug)]
pub struct TreeBuilder<'cache, L: AstTreeShape> {
	cache: CowMut<'cache, NodeCache>,
	parents: Vec<(L::Kind, usize)>,
	children: Vec<(u64, Option<GreenElement>)>,
}

impl<L: AstTreeShape> Default for TreeBuilder<'_, L> {
	fn default() -> Self {
		Self {
			cache: CowMut::default(),
			parents: Vec::default(),
			children: Vec::default(),
		}
	}
}

impl<L: AstTreeShape> TreeBuilder<'_, L> {
	/// Creates new builder.
	pub fn new() -> TreeBuilder<'static, L> {
		TreeBuilder::default()
	}

	/// Reusing `NodeCache` between different [TreeBuilder]`s saves memory.
	/// It allows to structurally share underlying trees.
	pub fn with_cache(cache: &mut NodeCache) -> TreeBuilder<'_, L> {
		TreeBuilder {
			cache: CowMut::Borrowed(cache),
			parents: Vec::new(),
			children: Vec::new(),
		}
	}

	/// Method to quickly wrap a tree with a node.
	///
	/// TreeBuilder::<RawLanguage>::wrap_with_node(RawSyntaxKind(0), |builder| {
	///     builder.token(RawSyntaxKind(1), "let");
	/// });
	pub fn wrap_with_node<F>(kind: L::Kind, build: F) -> SyntaxNode<L>
	where
		F: Fn(&mut Self),
	{
		let mut builder = TreeBuilder::<L>::new();
		builder.start_node(kind);
		build(&mut builder);
		builder.finish_node();
		builder.finish()
	}

	/// Adds new token to the current branch.
	#[inline]
	pub fn token(&mut self, kind: L::Kind, text: &str) {
		let (hash, token) = self.cache.token(L::kind_to_raw(kind), text);
		self.children.push((hash, Some(token.into())));
	}

	/// Adds new token to the current branch.
	#[inline]
	pub fn token_with_trivia(
		&mut self,
		kind: L::Kind,
		text: &str,
		leading: Vec<TriviaPiece>,
		trailing: Vec<TriviaPiece>,
	) {
		let (hash, token) =
			self.cache
				.token_with_trivia(L::kind_to_raw(kind), text, leading, trailing);
		self.children.push((hash, Some(token.into())));
	}

	/// Inserts a placeholder for a child that is missing in a parent node either because
	/// it's an optional node that isn't present or it's a mandatory child that is missing
	/// because of a syntax error.
	#[inline]
	pub fn missing(&mut self) {
		self.children.push(NodeCache::empty());
	}

	/// Start new node and make it current.
	#[inline]
	pub fn start_node(&mut self, kind: L::Kind) {
		let len = self.children.len();
		self.parents.push((kind, len));
	}

	/// Finish current branch and restore previous
	/// branch as current.
	#[inline]
	pub fn finish_node(&mut self) {
		let (kind, first_child) = self.parents.pop().unwrap();

		let raw_kind = L::kind_to_raw(kind);
		let (hash, node) =
			self.cache
				.node(raw_kind, &mut self.children, first_child, |all_children| {
					let children = &all_children[first_child..];
					let child_kinds = children
						.iter()
						.map(|(_, element)| element.as_ref().map(|e| L::kind_from_raw(e.kind())));

					let raw_kind = if L::fits_shape_of(&kind, children.len(), child_kinds) {
						raw_kind
					} else {
						L::kind_to_raw(kind.to_unknown())
					};

					GreenNode::new(
						raw_kind,
						all_children.drain(first_child..).map(|(_, it)| it),
					)
				});

		self.children.push((hash, Some(node.into())));
	}

	/// Prepare for maybe wrapping the next node.
	/// The way wrapping works is that you first of all get a checkpoint,
	/// then you place all tokens you want to wrap, and then *maybe* call
	/// `start_node_at`.
	/// Example:
	/// ```rust
	/// # use rome_rowan::{TreeBuilder, RawSyntaxKind};
	/// # use rome_rowan::api::{RawLanguage, RawLanguageKind};
	/// # const PLUS: RawLanguageKind = RawLanguageKind(0);
	/// # const OPERATION: RawLanguageKind = RawLanguageKind(1);
	/// # struct Parser;
	/// # impl Parser {
	/// #     fn peek(&self) -> Option<RawLanguageKind> { None }
	/// #     fn parse_expr(&mut self) {}
	/// # }
	/// # let mut builder = TreeBuilder::<'_, RawLanguage>::new();
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
	pub fn start_node_at(&mut self, checkpoint: Checkpoint, kind: L::Kind) {
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
	#[must_use]
	pub fn finish(self) -> SyntaxNode<L> {
		SyntaxNode::new_root(self.finish_green())
	}

	// For tests
	#[must_use]
	pub(crate) fn finish_green(mut self) -> GreenNode {
		assert_eq!(self.children.len(), 1);
		match self.children.pop().unwrap().1 {
			Some(NodeOrToken::Node(node)) => node,
			_ => panic!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::api::{RawLanguage, RawLanguageKind};
	use crate::green::GreenElementRef;
	use crate::{GreenNodeData, GreenTokenData, NodeOrToken, TreeBuilder};

	// Builds a "Condition" like structure where the closing ) is missing
	fn build_condition_with_missing_closing_parenthesis(builder: &mut TreeBuilder<RawLanguage>) {
		builder.start_node(RawLanguageKind(2));

		builder.token(RawLanguageKind(3), "(");

		builder.start_node(RawLanguageKind(4));
		builder.token(RawLanguageKind(5), "a");
		builder.finish_node();

		// missing )
		builder.missing();

		builder.finish_node();
	}

	#[test]
	fn caches_identical_nodes_with_empty_slots() {
		let mut builder: TreeBuilder<RawLanguage> = TreeBuilder::new();

		builder.start_node(RawLanguageKind(1)); // Root
		build_condition_with_missing_closing_parenthesis(&mut builder);
		build_condition_with_missing_closing_parenthesis(&mut builder);
		builder.finish_node();

		let root = builder.finish_green();

		let first = root.children().next().unwrap();
		let last = root.children().last().unwrap();

		assert_eq!(first.element(), last.element());
		assert_same_elements(first.element(), last.element());
	}

	#[test]
	fn doesnt_cache_node_if_empty_slots_differ() {
		let mut builder: TreeBuilder<RawLanguage> = TreeBuilder::new();

		builder.start_node(RawLanguageKind(1)); // Root
		build_condition_with_missing_closing_parenthesis(&mut builder); // misses the ')'

		// Create a well formed condition
		builder.start_node(RawLanguageKind(2));

		builder.token(RawLanguageKind(3), "(");

		builder.start_node(RawLanguageKind(4));
		builder.token(RawLanguageKind(5), "a");
		builder.finish_node();

		// missing )
		builder.token(RawLanguageKind(5), ")");

		builder.finish_node();

		// finish root
		builder.finish_node();

		let root = builder.finish_green();
		let first_condition = root.children().next().unwrap();
		let last_condition = root.children().last().unwrap();

		assert_ne!(first_condition.element(), last_condition.element());
	}

	fn assert_same_elements(left: GreenElementRef<'_>, right: GreenElementRef<'_>) {
		fn element_id(element: GreenElementRef<'_>) -> *const () {
			match element {
				NodeOrToken::Node(node) => node as *const GreenNodeData as *const (),
				NodeOrToken::Token(token) => token as *const GreenTokenData as *const (),
			}
		}

		assert_eq!(element_id(left), element_id(right),);
	}
}
