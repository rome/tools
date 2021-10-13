use crate::{
	empty_element, format_elements, space_token, token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::{CatchClause, Finalizer, TryStmt};

impl ToFormatElement for TryStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let try_token = formatter.format_token(&self.try_token().expect("try token is missing"));
		let test = formatter.format_node(self.test().expect("try test is missing"));
		let handler = self.handler().map_or(empty_element(), |catch_clause| {
			formatter.format_node(catch_clause)
		});
		let finalizer = self.finalizer().map_or(empty_element(), |finally_node| {
			formatter.format_node(finally_node)
		});
		if handler.is_empty() & finalizer.is_empty() {
			// TODO: better error handler
			panic!("catch and finally nodes are both missing")
		}
		format_elements![
			try_token,
			space_token(),
			test,
			space_token(),
			handler,
			finalizer
		]
	}
}

impl ToFormatElement for Finalizer {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let cons = formatter.format_node(self.cons().expect("consequence is missing"));
		format_elements![token("finally"), space_token(), cons]
	}
}

impl ToFormatElement for CatchClause {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let l_paren = self.l_paren_token();
		let r_paren = self.r_paren_token();
		let error = self.error();
		let cons = formatter.format_node(self.cons().expect("consequence is missing"));
		match (l_paren, r_paren, error) {
			(None, None, None) => {
				format_elements![token("catch"), space_token(), cons]
			}
			(Some(l_paren), Some(r_paren), Some(error)) => {
				format_elements![
					token("catch"),
					space_token(),
					formatter.format_token(&l_paren),
					formatter.format_node(error),
					formatter.format_token(&r_paren),
					space_token(),
					cons
				]
			}
			_ => {
				// Here we panic because a valid catch clause needs to have all the tokens of none:
				// - catch (e) {}
				// - catch {}
				//
				// Other cases should fall into an error
				// TODO: better error handling
				panic!("The catch error is invalid")
			}
		}
	}
}
