mod element;
mod node;
mod node_cache;
mod token;

pub(crate) use self::{
	element::{GreenElement, GreenElementRef},
	node::{Child, Children, GreenNode, GreenNodeData, Slot},
	token::{GreenToken, GreenTokenData},
};

pub use self::node_cache::NodeCache;
pub(crate) use self::node_cache::NodeCacheNodeEntryMut;

/// SyntaxKind is a type tag for each token or node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawSyntaxKind(pub u16);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn assert_send_sync() {
		fn f<T: Send + Sync>() {}
		f::<GreenNode>();
		f::<GreenToken>();
		f::<GreenElement>();
	}

	#[test]
	fn test_size_of() {
		use std::mem::size_of;

		eprintln!("GreenNode          {}", size_of::<GreenNode>());
		eprintln!("GreenToken         {}", size_of::<GreenToken>());
		eprintln!("GreenElement       {}", size_of::<GreenElement>());
	}
}
