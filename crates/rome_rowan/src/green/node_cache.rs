use hashbrown::hash_map::RawEntryMut;
use rustc_hash::FxHasher;
use std::hash::{BuildHasherDefault, Hash, Hasher};

use crate::api::TriviaPiece;
use crate::green::Slot;
use crate::{
	green::GreenElementRef, GreenNode, GreenNodeData, GreenToken, GreenTokenData, NodeOrToken,
	RawSyntaxKind,
};

use super::element::GreenElement;
use super::token::GreenTokenTrivia;

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

fn token_hash_of(kind: RawSyntaxKind, text: &str) -> u64 {
	let mut h = FxHasher::default();
	kind.hash(&mut h);
	text.hash(&mut h);
	h.finish()
}

fn token_hash(token: &GreenTokenData) -> u64 {
	token_hash_of(token.kind(), token.text())
}

fn node_hash(node: &GreenNodeData) -> u64 {
	let mut h = FxHasher::default();
	node.kind().hash(&mut h);
	for slot in node.slots() {
		match slot {
			Slot::Empty { .. } => NodeCache::EMPTY_SLOT_HASH,
			Slot::Node { node, .. } => node_hash(node),
			Slot::Token { token, .. } => token_hash(token),
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
	/// Hash used for nodes that haven't been cached because it has too many slots or
	/// one of its children wasn't cached.
	const UNCACHED_NODE_HASH: u64 = 0;

	/// Hash for an empty slot
	const EMPTY_SLOT_HASH: u64 = 1;

	pub(crate) const fn empty() -> (u64, Option<GreenElement>) {
		(Self::EMPTY_SLOT_HASH, None)
	}

	pub(crate) fn node<F>(
		&mut self,
		kind: RawSyntaxKind,
		// u64 is the hash of the slot
		slots: &mut Vec<(u64, Option<GreenElement>)>,
		first_child: usize,
		build_node: F,
	) -> (u64, GreenNode)
	where
		F: FnOnce(&mut Vec<(u64, Option<GreenElement>)>) -> GreenNode,
	{
		let slots_ref = &slots[first_child..];
		if slots_ref.len() > 3 {
			let node = build_node(slots);
			return (Self::UNCACHED_NODE_HASH, node);
		}

		let hash = {
			let mut h = FxHasher::default();
			kind.hash(&mut h);
			for &(hash, _) in slots_ref {
				if hash == Self::UNCACHED_NODE_HASH {
					let node = build_node(slots);
					return (Self::UNCACHED_NODE_HASH, node);
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
			node.0.kind() == kind && node.0.slots().len() == slots_ref.len() && {
				let lhs = node.0.slots();
				let lhs = lhs.map(|slot| slot.as_ref().map(element_id));

				let rhs = slots_ref.iter().map(|(_, element)| {
					element
						.as_ref()
						.map(|element| element_id(element.as_deref()))
				});

				lhs.eq(rhs)
			}
		});

		let node = match entry {
			RawEntryMut::Occupied(entry) => {
				slots.truncate(first_child);
				entry.key().0.clone()
			}
			RawEntryMut::Vacant(entry) => {
				let node = build_node(slots);
				entry.insert_with_hasher(hash, NoHash(node.clone()), (), |n| node_hash(&n.0));
				node
			}
		};

		(hash, node)
	}

	pub(crate) fn token(&mut self, kind: RawSyntaxKind, text: &str) -> (u64, GreenToken) {
		self.token_with_trivia(kind, text, Vec::new(), Vec::new())
	}

	pub(crate) fn token_with_trivia(
		&mut self,
		kind: RawSyntaxKind,
		text: &str,
		leading: Vec<TriviaPiece>,
		trailing: Vec<TriviaPiece>,
	) -> (u64, GreenToken) {
		let hash = token_hash_of(kind, text);

		let leading = GreenTokenTrivia::from(leading);
		let trailing = GreenTokenTrivia::from(trailing);

		let entry = self.tokens.raw_entry_mut().from_hash(hash, |token| {
			token.0.kind() == kind && token.0.text() == text
		});

		let token = match entry {
			RawEntryMut::Occupied(entry) => entry.key().0.clone(),
			RawEntryMut::Vacant(entry) => {
				let token = GreenToken::with_trivia(kind, text, leading, trailing);
				entry.insert_with_hasher(hash, NoHash(token.clone()), (), |t| token_hash(&t.0));
				token
			}
		};

		(hash, token)
	}
}

#[test]
fn green_token_hash() {
	let kind = RawSyntaxKind(0);
	let text = " let ";
	let t1 = GreenToken::with_trivia(
		kind,
		text,
		GreenTokenTrivia::Whitespace(1),
		GreenTokenTrivia::Whitespace(1),
	);
	let t2 = GreenToken::with_trivia(
		kind,
		text,
		GreenTokenTrivia::Whitespace(1),
		GreenTokenTrivia::Whitespace(1),
	);

	assert_eq!(token_hash(&t1), token_hash(&t2));

	let t3 = GreenToken::new(kind, "let");
	assert_ne!(token_hash(&t1), token_hash(&t3));

	let t4 = GreenToken::with_trivia(
		kind,
		"\tlet ",
		GreenTokenTrivia::Whitespace(1),
		GreenTokenTrivia::Whitespace(1),
	);
	assert_ne!(token_hash(&t1), token_hash(&t4));
}
