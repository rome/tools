use rslint_parser::ast::SequenceExpr;

use crate::{concat_elements, FormatElement, ToFormatElement};

impl ToFormatElement for SequenceExpr {
	fn to_format_element(&self) -> FormatElement {
		let tokens: Vec<_> = self
			.exprs()
			.map(|expression| expression.to_format_element())
			.collect();
		concat_elements(tokens)
	}
}
