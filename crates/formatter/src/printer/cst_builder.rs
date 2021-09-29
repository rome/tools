use rslint_parser::{GreenNode, NodeOrToken, SyntaxNode};
use rslint_rowan::GreenToken;

/// Id of the parent node into which a child node should be inserted
/// 0 -> Insert to root
/// 1 -> First inserted node
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct ParentNodeId(usize);

impl ParentNodeId {
	pub fn root() -> Self {
		ParentNodeId::default()
	}
}

pub(crate) struct CSTBuilderSnapshot {
	parents_pos: usize,
	children_pos: usize,
}

/// Helps building a Concrete Syntax Tree in the [Printer]. The builder
/// stores the currently started nodes in the [parents] collection and the children of all
/// started nodes in the [children] collection.
#[derive(Debug, Default)]
pub(crate) struct CSTBuilder {
	/// Stores all started nodes together with the index where in the [children] array the
	/// children of this node start.  
	parents: Vec<(GreenNode, usize)>,

	/// Stores the children of the started nodes.
	children: Vec<NodeOrToken<GreenNode, GreenToken>>,
}

impl CSTBuilder {
	/// Appends a node to the parent specified by the parent id. Finishes all nodes from the
	/// node at the top of [parents] along to the node with the [parent] id.
	pub fn append_node(&mut self, parent: ParentNodeId, node: GreenNode) -> ParentNodeId {
		self.finish_children(parent);

		self.parents.push((node, self.children.len()));
		ParentNodeId(self.parents.len())
	}

	/// Appends a node that should be left as is, without updating its children.
	pub fn append_raw_node(&mut self, parent: ParentNodeId, node: GreenNode) {
		self.finish_children(parent);

		self.children.push(NodeOrToken::Node(node));
	}

	/// Appends a parent as a child to the specified in parent
	pub fn append_token(&mut self, parent: ParentNodeId, token: GreenToken) {
		self.finish_children(parent);
		self.children.push(NodeOrToken::Token(token));
	}

	/// Finishes all currently open nodes and returns the root node
	pub fn root_node(mut self) -> SyntaxNode {
		while let Some((node, first_child)) = self.parents.pop() {
			finish_node(node, first_child, &mut self.children);
		}

		assert_eq!(
			1,
			self.children.len(),
			"The children should have been reduced to the root node only"
		);

		let root = self.children.pop().unwrap();

		match root {
			NodeOrToken::Node(node) => SyntaxNode::new_root(node),
			_ => panic!("The root element must be a node"),
		}
	}

	/// Finishes all the started nodes until we reach the passed parent.
	/// This is needed because the printer only does a pre-order traversal of the tokens, meaning
	/// it can't explicitly call finish node.
	fn finish_children(&mut self, parent: ParentNodeId) {
		assert!(
			parent.0 <= self.parents.len(),
			"Id no longer valid, was finish_node called before?"
		);

		for (node, first_child) in self.parents.drain(parent.0..).rev() {
			finish_node(node, first_child, &mut self.children);
		}
	}

	/// Creates a snapshot that allows "rewinding" the builder to a previous state.
	/// Rewinding is only guaranteed to work if no nodes started before the snapshot
	/// are finished after creating the snapshot.
	pub fn snapshot(&self) -> CSTBuilderSnapshot {
		CSTBuilderSnapshot {
			parents_pos: self.parents.len(),
			children_pos: self.children.len(),
		}
	}

	/// Restores the builder back to the state when the snapshot was taken.
	pub fn restore(&mut self, snapshot: CSTBuilderSnapshot) {
		assert!(self.children.len() >= snapshot.children_pos, "snapshot no longer valid, was it restored before or was it used past the point where the snapshot was taken?");
		assert!(self.parents.len() >= snapshot.parents_pos, "snapshot no longer valid, was it restored before or was it used past the point where the snapshot was taken?");

		self.children.truncate(snapshot.children_pos);
		self.parents.truncate(snapshot.parents_pos);
	}
}

fn finish_node(
	node: GreenNode,
	first_child: usize,
	children: &mut Vec<NodeOrToken<GreenNode, GreenToken>>,
) {
	assert!(children.len() > first_child);

	let merged = with_merged_children(node, children, first_child);
	children.push(NodeOrToken::Node(merged));
}

fn with_merged_children(
	node: GreenNode,
	children: &mut Vec<NodeOrToken<GreenNode, GreenToken>>,
	first_child: usize,
) -> GreenNode {
	let existing_children = node.children();
	let new_children = &children[first_child..];

	// Let's try to reuse the existing node whenever possible. The assumption is that most of the
	// file is properly formatted and, therefore, most nodes haven't changed.
	if shallow_eq_children(existing_children, new_children) {
		children.truncate(first_child);
		node
	} else {
		GreenNode::new(node.kind(), children.drain(first_child..))
	}
}

/// Does a shallow comparison of the children:
/// * Nodes: test if the nodes point to the same underlying data structure
/// * Tokens: Equal comparison because the [FormatValue] implementations create new tokens for
///  `,`, `;` but we want to avoid creating new nodes just because of that.
fn shallow_eq_children<'a, L, R>(lhs: L, rhs: R) -> bool
where
	L: IntoIterator<Item = NodeOrToken<&'a GreenNode, &'a GreenToken>>,
	L::IntoIter: ExactSizeIterator,
	R: IntoIterator<Item = &'a NodeOrToken<GreenNode, GreenToken>>,
	R::IntoIter: ExactSizeIterator,
{
	let lhs = lhs.into_iter();
	let rhs = rhs.into_iter();

	if lhs.len() != rhs.len() {
		false
	} else {
		lhs.zip(rhs).all(|(existing, new)| match existing {
			NodeOrToken::Node(existing_node) => match new {
				NodeOrToken::Node(new_node) => existing_node.shallow_eq(new_node),
				_ => false,
			},
			NodeOrToken::Token(existing_token) => match new {
				NodeOrToken::Token(new_token) => existing_token == new_token,
				_ => false,
			},
		})
	}
}

#[cfg(test)]
mod tests {
	// use super::nodes_shallow_eq;
	use crate::printer::cst_builder::{shallow_eq_children, CSTBuilder, ParentNodeId};
	use crate::Tokens;
	use rslint_parser::{GreenNode, NodeOrToken, SyntaxKind, SyntaxNode};
	use rslint_rowan::GreenToken;

	fn create_node(kind: SyntaxKind) -> GreenNode {
		GreenNode::new(rslint_rowan::SyntaxKind(kind.into()), vec![])
	}

	fn create_node_with_children(
		kind: SyntaxKind,
		children: Vec<NodeOrToken<GreenNode, GreenToken>>,
	) -> GreenNode {
		GreenNode::new(rslint_rowan::SyntaxKind(kind.into()), children)
	}

	#[test]
	fn shallow_eq_children_returns_true_for_same_node() {
		let node = create_node(SyntaxKind::MODULE);

		assert!(shallow_eq_children(
			vec![NodeOrToken::Node(&node)],
			vec![&NodeOrToken::Node(node.clone())]
		));
	}

	#[test]
	fn shallow_eq_children_returns_true_for_same_tokens() {
		let mut tokens = Tokens::default();

		let token = tokens.double_quoted_string("test");

		assert!(shallow_eq_children(
			vec![NodeOrToken::Token(&token)],
			vec![&NodeOrToken::Token(token.clone())]
		));
	}

	#[test]
	fn shallow_eq_children_returns_true_for_equal_tokens() {
		let mut tokens = Tokens::default();

		let lhs = tokens.double_quoted_string("test");
		let rhs = tokens.double_quoted_string("test");

		assert!(shallow_eq_children(
			vec![NodeOrToken::Token(&lhs)],
			vec![&NodeOrToken::Token(rhs)]
		));
	}

	#[test]
	fn shallow_eq_children_returns_false_for_non_equal_tokens() {
		let mut tokens = Tokens::default();

		let hy = tokens.double_quoted_string("hy");
		let hello = tokens.double_quoted_string("hello");

		assert!(!shallow_eq_children(
			vec![NodeOrToken::Token(&hy)],
			vec![&NodeOrToken::Token(hello)]
		));
	}

	#[test]
	fn shallow_eq_children_returns_true_for_same_nodes() {
		let mut tokens = Tokens::default();

		let common_node = create_node_with_children(
			SyntaxKind::LITERAL,
			vec![NodeOrToken::Token(tokens.double_quoted_string("test"))],
		);

		assert!(shallow_eq_children(
			vec![NodeOrToken::Node(&common_node)],
			vec![&NodeOrToken::Node(common_node.clone())]
		));
	}

	#[test]
	fn shallow_eq_children_returns_true_for_different_nodes() {
		let lhs = create_node(SyntaxKind::SCRIPT);
		let rhs = create_node(SyntaxKind::SCRIPT);

		// These nodes have the same shape but point to different green nodes which is why they are not shallow equal
		assert!(!shallow_eq_children(
			vec![NodeOrToken::Node(&lhs)],
			vec![&NodeOrToken::Node(rhs)]
		));
	}

	#[test]
	fn builder_constructs_a_tree_with_all_nodes_and_tokens() {
		let mut builder = CSTBuilder::default();
		let mut tokens = Tokens::default();

		let root_position =
			builder.append_node(ParentNodeId::default(), create_node(SyntaxKind::SCRIPT));
		let child_position = builder.append_node(root_position, create_node(SyntaxKind::LITERAL));
		builder.append_token(child_position, tokens.get(SyntaxKind::NUMBER, "5"));
		builder.append_token(root_position, tokens.whitespace("\n"));

		let root = builder.root_node();

		let expected = SyntaxNode::new_root(create_node_with_children(
			SyntaxKind::SCRIPT,
			vec![
				NodeOrToken::Node(create_node_with_children(
					SyntaxKind::LITERAL,
					vec![NodeOrToken::Token(tokens.get(SyntaxKind::NUMBER, "5"))],
				)),
				NodeOrToken::Token(tokens.whitespace("\n")),
			],
		));
		assert_eq!(
			expected.green(),
			root.green(),
			"Expected trees to match.\nleft: {:#?}\nright: {:#?}",
			expected,
			root
		);
	}

	#[test]
	fn builder_reuses_nodes_and_tokens() {
		// program(
		// 	[1],
		//  'abc'
		// )
		let mut tokens = Tokens::default();

		let number = create_node_with_children(
			SyntaxKind::LITERAL,
			vec![NodeOrToken::Token(tokens.get(SyntaxKind::NUMBER, "1"))],
		);

		let array = create_node_with_children(
			SyntaxKind::ARRAY_EXPR,
			vec![
				NodeOrToken::Token(tokens.left_bracket()),
				NodeOrToken::Node(number.clone()),
				NodeOrToken::Token(tokens.right_bracket()),
			],
		);

		let string = create_node_with_children(
			SyntaxKind::LITERAL,
			vec![NodeOrToken::Token(tokens.get(SyntaxKind::STRING, "'abc'"))],
		);

		let program = create_node_with_children(
			SyntaxKind::MODULE,
			vec![
				NodeOrToken::Node(array.clone()),
				NodeOrToken::Node(string.clone()),
			],
		);

		// program(
		// 	[1],
		//  "abc"
		// )
		let mut builder = CSTBuilder::default();

		let program_position = builder.append_node(ParentNodeId::default(), program);

		let array_position = builder.append_node(program_position, array.clone());
		builder.append_token(array_position, tokens.left_bracket());
		let num_position = builder.append_node(array_position, number);
		builder.append_token(num_position, tokens.get(SyntaxKind::NUMBER, "1"));
		builder.append_token(array_position, tokens.right_bracket());

		// convert quotes
		let string_position = builder.append_node(program_position, string);
		builder.append_token(string_position, tokens.double_quoted_string("abc"));

		let root = builder.root_node();

		let expected_str = create_node_with_children(
			SyntaxKind::LITERAL,
			vec![NodeOrToken::Token(tokens.double_quoted_string("abc"))],
		);

		let expected_program = create_node_with_children(
			SyntaxKind::MODULE,
			vec![
				NodeOrToken::Node(array.clone()),
				NodeOrToken::Node(expected_str),
			],
		);

		assert_eq!(&expected_program, root.green());

		let generated_array = root.first_child().unwrap();

		// The array should still be the same node as in the original program since its formatting hasn't change
		assert!(array.shallow_eq(generated_array.green()));
	}

	#[test]
	fn builder_rewinds_when_restoring_a_snapshot() {
		let mut builder = CSTBuilder::default();
		let mut tokens = Tokens::default();

		let program_pos =
			builder.append_node(ParentNodeId::root(), create_node(SyntaxKind::SCRIPT));
		let snapshot = builder.snapshot();

		let string_pos = builder.append_node(program_pos, create_node(SyntaxKind::LITERAL));
		builder.append_token(
			string_pos,
			tokens.double_quoted_string("a very long string that causes a line break"),
		);

		builder.restore(snapshot);

		let string_pos = builder.append_node(program_pos, create_node(SyntaxKind::LITERAL));
		builder.append_token(
			string_pos,
			tokens.double_quoted_string("a very long string that causes a line break"),
		);
		builder.append_token(string_pos, tokens.whitespace("\n"));

		let root = builder.root_node();

		let expected = SyntaxNode::new_root(create_node_with_children(
			SyntaxKind::SCRIPT,
			vec![NodeOrToken::Node(create_node_with_children(
				SyntaxKind::LITERAL,
				vec![
					NodeOrToken::Token(
						tokens.double_quoted_string("a very long string that causes a line break"),
					),
					NodeOrToken::Token(tokens.whitespace("\n")),
				],
			))],
		));

		assert_eq!(
			root.green(),
			expected.green(),
			"left: {:#?}\nright: {:#?}",
			root,
			expected
		);
	}
}
