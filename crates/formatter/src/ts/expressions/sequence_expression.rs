use rslint_parser::ast::SequenceExpr;

use crate::{concat_elements, FormatContext, FormatElement, ToFormatElement};

impl ToFormatElement for SequenceExpr {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		concat_elements(
			self.exprs()
				.map(|expression| context.format_node(expression)),
		)
	}
}
