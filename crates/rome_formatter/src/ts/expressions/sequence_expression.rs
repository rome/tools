use rslint_parser::ast::SequenceExpr;

use crate::{concat_elements, FormatElement, FormatError, Formatter, ToFormatElement};

impl ToFormatElement for SequenceExpr {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		Ok(concat_elements(formatter.format_nodes(self.exprs())?))
	}
}
