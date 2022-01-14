//! General utility functions for parsing and error checking.

use crate::Parser;
use rslint_syntax::{JsSyntaxKind, T};

/// Get the precedence of a token
pub(crate) fn get_precedence(tok: JsSyntaxKind) -> Option<u8> {
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

pub(crate) fn expect_keyword(p: &mut Parser, keyword_name: &str, kind: JsSyntaxKind) {
	if p.at(T![ident]) && p.cur_src() == keyword_name {
		p.bump_remap(kind);
	} else {
		let err = if p.cur() == JsSyntaxKind::EOF {
			p.err_builder(&format!(
				"expected `{}` but instead the file ends",
				keyword_name
			))
			.primary(p.cur_tok().range(), "the file ends here")
		} else {
			p.err_builder(&format!(
				"expected `{}` but instead found `{}`",
				keyword_name,
				p.cur_src()
			))
			.primary(p.cur_tok().range(), "unexpected")
		};

		p.error(err);
	}
}
