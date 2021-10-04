//! This module is for adding extensions to AST Nodes so that the
//! auto-generated files don't need to be modified.

use crate::{
	ast::{self, support},
	SyntaxKind, SyntaxToken,
};

// This is an example of extending as AST node, but these are actually
// methods that rust-analyzer is able to auto-generate
impl ast::Array {
	pub fn l_brack_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::LBRACK_TOKEN)
	}
	pub fn r_brack_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, SyntaxKind::RBRACK_TOKEN)
	}
}
