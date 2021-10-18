use hashbrown::hash_map::RawEntryMut;
use rustc_hash::FxHasher;
use std::hash::{BuildHasherDefault, Hash, Hasher};

use crate::{
	green::GreenElementRef, GreenNode, GreenNodeData, GreenToken, GreenTokenData, NodeOrToken,
	SyntaxKind,
};

use super::element::GreenElement;

type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<FxHasher>>;

#[derive(Debug)]
struct NoHash<T>(T);

/// Interner for GreenTokens and GreenNodes
// XXX: the impl is a bit tricky. As usual when writing interners, we want to
// store all values in one HashSet.
//
// However, hashing trees is fun: hash of the tree is recursively defined. We
// maintain an invariant -- if the tree is interned, then all of its children
// are interned as well.
//
// That means that computing the hash naively is wasteful -- we just *know*
// hashes of children, and we can re-use those.
//
// So here we use *raw* API of hashbrown and provide the hashes manually,
// instead of going via a `Hash` impl. Our manual `Hash` and the
// `#[derive(Hash)]` are actually different! At some point we had a fun bug,
// where we accidentally mixed the two hashes, which made the cache much less
// efficient.
//
// To fix that, we additionally wrap the data in `NoHash` wrapper, to make sure
// we don't accidentally use the wrong hash!
#[derive(Default, Debug)]
pub struct NodeCache {
	nodes: HashMap<NoHash<GreenNode>, ()>,
	tokens: HashMap<NoHash<GreenToken>, ()>,
}

fn token_hash(token: &GreenTokenData) -> u64 {
	let mut h = FxHasher::default();
	token.kind().hash(&mut h);
	token.text().hash(&mut h);
	h.finish()
}

fn node_hash(node: &GreenNodeData) -> u64 {
	let mut h = FxHasher::default();
	node.kind().hash(&mut h);
	for child in node.children() {
		match child {
			NodeOrToken::Node(it) => node_hash(it),
			NodeOrToken::Token(it) => token_hash(it),
		}
		.hash(&mut h)
	}
	h.finish()
}

fn element_id(elem: GreenElementRef<'_>) -> *const () {
	match elem {
		NodeOrToken::Node(it) => it as *const GreenNodeData as *const (),
		NodeOrToken::Token(it) => it as *const GreenTokenData as *const (),
	}
}

impl NodeCache {
	pub(crate) fn node(
		&mut self,
		kind: SyntaxKind,
		children: &mut Vec<(u64, GreenElement)>,
		first_child: usize,
	) -> (u64, GreenNode) {
		let build_node = move |children: &mut Vec<(u64, GreenElement)>| {
			GreenNode::new(kind, children.drain(first_child..).map(|(_, it)| it))
		};

		let children_ref = &children[first_child..];
		if children_ref.len() > 3 {
			let node = build_node(children);
			return (0, node);
		}

		let hash = {
			let mut h = FxHasher::default();
			kind.hash(&mut h);
			for &(hash, _) in children_ref {
				if hash == 0 {
					let node = build_node(children);
					return (0, node);
				}
				hash.hash(&mut h);
			}
			h.finish()
		};

		// Green nodes are fully immutable, so it's ok to deduplicate them.
		// This is the same optimization that Roslyn does
		// https://github.com/KirillOsenkov/Bliki/wiki/Roslyn-Immutable-Trees
		//
		// For example, all `#[inline]` in this file share the same green node!
		// For `libsyntax/parse/parser.rs`, measurements show that deduping saves
		// 17% of the memory for green nodes!
		let entry = self.nodes.raw_entry_mut().from_hash(hash, |node| {
			node.0.kind() == kind && node.0.children().len() == children_ref.len() && {
				let lhs = node.0.children();
				let rhs = children_ref.iter().map(|(_, it)| it.as_deref());

				let lhs = lhs.map(element_id);
				let rhs = rhs.map(element_id);

				lhs.eq(rhs)
			}
		});

		let node = match entry {
			RawEntryMut::Occupied(entry) => {
				drop(children.drain(first_child..));
				entry.key().0.clone()
			}
			RawEntryMut::Vacant(entry) => {
				let node = build_node(children);
				entry.insert_with_hasher(hash, NoHash(node.clone()), (), |n| node_hash(&n.0));
				node
			}
		};

		(hash, node)
	}

	pub(crate) fn token(&mut self, kind: SyntaxKind, text: &str) -> (u64, GreenToken) {
		let hash = {
			let mut h = FxHasher::default();
			kind.hash(&mut h);
			text.hash(&mut h);
			h.finish()
		};

		let entry = self.tokens.raw_entry_mut().from_hash(hash, |token| {
			token.0.kind() == kind && token.0.text() == text
		});

		let token = match entry {
			RawEntryMut::Occupied(entry) => entry.key().0.clone(),
			RawEntryMut::Vacant(entry) => {
				let token = GreenToken::new(kind, text);
				entry.insert_with_hasher(hash, NoHash(token.clone()), (), |t| token_hash(&t.0));
				token
			}
		};

		(hash, token)
	}
}
