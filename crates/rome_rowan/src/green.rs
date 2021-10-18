mod builder;
mod element;
mod node;
mod node_cache;
mod token;

use self::element::GreenElement;

pub(crate) use self::{element::GreenElementRef, node::GreenChild};

pub use self::{
	builder::{Checkpoint, GreenNodeBuilder},
	node::{Children, GreenNode, GreenNodeData},
	node_cache::NodeCache,
	token::{GreenToken, GreenTokenData},
};

/// SyntaxKind is a type tag for each token or node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntaxKind(pub u16);

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
