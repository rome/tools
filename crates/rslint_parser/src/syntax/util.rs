//! General utility functions for parsing and error checking.

use crate::{ast, AstNode, CompletedMarker, Parser, TextRange, Token};
use rslint_syntax::{SyntaxKind, T};

/// Check if the use of a statement label is valid and the label is defined.
///
/// # Panics
/// Panics if the marker is not a name with an ident
// FIXME: Labels should not cross function boundaries
pub fn check_label_use(p: &mut Parser, label: &Token) {
	let name = p.token_src(label);

	if p.state.labels.get(name).is_none() {
		let err = p
			.err_builder(&format!("Use of undefined statement label `{}`", name))
			.primary(&label.range, "This label is used, but it is never defined");

		p.error(err);
	}
}

/// Get the precedence of a token
pub fn get_precedence(tok: SyntaxKind) -> Option<u8> {
	Some(match tok {
		T![||] | T![??] => 1,
		T![&&] => 2,
		T![|] => 3,
		T![^] => 4,
		T![&] => 5,
		T![==] | T![!=] | T![===] | T![!==] => 6,
		T![>] | T![>=] | T![<] | T![<=] => 7,
		T![<<] | T![>>] | T![>>>] => 8,
		T![+] | T![-] => 9,
		T![*] | T![/] => 10,
		T![%] | T![**] => 11,
		_ => return None,
	})
}

/// Check if the var declaration in a for statement has multiple declarators, which is invalid
pub fn check_for_stmt_declaration(p: &mut Parser, marker: &CompletedMarker) {
	#[allow(deprecated)]
	let parsed = p.parse_marker::<ast::JsVariableDeclaration>(marker);
	let excess = parsed.declarators().iter().skip(1).collect::<Vec<_>>();

	if !excess.is_empty() {
		let start = marker
			.offset_range(
				p,
				excess
					.first()
					.unwrap()
					.as_ref()
					.unwrap()
					.syntax()
					.text_trimmed_range(),
			)
			.start();
		let end = marker
			.offset_range(
				p,
				excess
					.last()
					.unwrap()
					.as_ref()
					.unwrap()
					.syntax()
					.text_trimmed_range(),
			)
			.end();

		let err = p
			.err_builder("For statement variable declarations may only have one declaration")
			.primary(TextRange::new(start, end), "");

		p.error(err);
	}
}

/// Tells [is_at_async_function] if it needs to check line breaks
#[derive(PartialEq)]
pub(super) enum LineBreak {
	// check line breaks
	DoCheck,
	// do not check line break
	DoNotCheck,
}

#[inline]
/// Checks if the parser is inside a "async function"
pub(super) fn is_at_async_function(p: &mut Parser, should_check_line_break: LineBreak) -> bool {
	let async_function_tokens = p.cur_src() == "async" && p.nth_at(1, T![function]);
	if should_check_line_break == LineBreak::DoCheck {
		async_function_tokens && !p.has_linebreak_before_n(1)
	} else {
		async_function_tokens
	}
}
