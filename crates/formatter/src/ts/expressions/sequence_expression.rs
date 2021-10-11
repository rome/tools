use rslint_parser::ast::SequenceExpr;

use crate::{concat_elements, FormatElement, Formatter, ToFormatElement};

impl ToFormatElement for SequenceExpr {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		Some(concat_elements(
			self.exprs()
				.map(|expression| formatter.format_node(expression))
				.flatten(),
		))
	}
}
