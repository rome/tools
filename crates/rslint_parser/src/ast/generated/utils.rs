//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::SyntaxKind::{self, *};
pub fn is_unknown_kind(kind: SyntaxKind) -> bool {
	matches!(
		kind,
		JS_UNKNOWN_EXPRESSION
			| JS_UNKNOWN_STATEMENT
			| JS_UNKNOWN_PATTERN
			| JS_UNKNOWN_MEMBER
			| JS_UNKNOWN_BINDING
			| JS_UNKNOWN_ASSIGNMENT_TARGET
	)
}
