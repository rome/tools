use hashbrown::hash_map::{RawEntryMut, RawOccupiedEntryMut, RawVacantEntryMut};
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
			Slot::Empty { .. } => {}
			Slot::Node { node, .. } => node_hash(node).hash(&mut h),
			Slot::Token { token, .. } => token_hash(token).hash(&mut h),
		}
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

	/// Creates a new node or retrieves it from the cache.
	/// * `kind`: The type of the node
	/// * `children`: The slots of this node are all the elements in `children` starting from `first_child`
	/// * `build_node`: Function that constructs a node in case it wasn't part of the cache.
	pub(crate) fn node(
		&mut self,
		kind: RawSyntaxKind,
		slots: &[(u64, GreenElement)],
	) -> NodeCacheNodeEntryMut {
		if slots.len() > 3 {
			return NodeCacheNodeEntryMut::NoCache(Self::UNCACHED_NODE_HASH);
		}

		let hash = {
			let mut h = FxHasher::default();
			kind.hash(&mut h);
			for &(hash, _) in slots {
				if hash == Self::UNCACHED_NODE_HASH {
					return NodeCacheNodeEntryMut::NoCache(Self::UNCACHED_NODE_HASH);
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
			node.0.kind() == kind && node.0.slots().len() == slots.len() && {
				let lhs = node.0.slots();
				let lhs = lhs.filter_map(|slot| match slot {
					Slot::Empty { .. } => None,
					Slot::Node { node, .. } => Some(element_id(NodeOrToken::Node(node))),
					Slot::Token { token, .. } => Some(element_id(NodeOrToken::Token(token))),
				});

				let rhs = slots
					.iter()
					.map(|(_, element)| element_id(element.as_deref()));

				lhs.eq(rhs)
			}
		});

		match entry {
			RawEntryMut::Occupied(entry) => NodeCacheNodeEntryMut::Cached(CachedNodeEntry {
				hash,
				raw_entry: entry,
			}),
			RawEntryMut::Vacant(entry) => NodeCacheNodeEntryMut::Vacant(VacantNodeEntry {
				raw_entry: entry,
				hash,
			}),
		}
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

pub(crate) enum NodeCacheNodeEntryMut<'a> {
	Cached(CachedNodeEntry<'a>),
	NoCache(u64),
	Vacant(VacantNodeEntry<'a>),
}

pub(crate) struct VacantNodeEntry<'a> {
	hash: u64,
	raw_entry: RawVacantEntryMut<'a, NoHash<GreenNode>, (), BuildHasherDefault<FxHasher>>,
}

pub(crate) struct CachedNodeEntry<'a> {
	hash: u64,
	raw_entry: RawOccupiedEntryMut<'a, NoHash<GreenNode>, (), BuildHasherDefault<FxHasher>>,
}

impl<'a> CachedNodeEntry<'a> {
	pub fn node(&self) -> &GreenNode {
		&self.raw_entry.key().0
	}

	pub fn hash(&self) -> u64 {
		self.hash
	}
}

impl<'a> VacantNodeEntry<'a> {
	pub fn hash(&self) -> u64 {
		self.hash
	}
	pub fn insert(self, node: GreenNode) {
		self.raw_entry
			.insert_with_hasher(self.hash, NoHash(node), (), |n| node_hash(&n.0));
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
