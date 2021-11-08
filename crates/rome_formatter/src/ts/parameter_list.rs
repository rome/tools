use crate::{
	format_elements, group_elements, join_elements, soft_indent, soft_line_break_or_space, token,
	FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::ParameterList;

impl ToFormatElement for ParameterList {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let param_tokens = formatter.format_nodes(self.parameters())?;

		Some(group_elements(format_elements![
			formatter.format_token(&self.l_paren_token()?)?,
			soft_indent(join_elements(
				format_elements![token(","), soft_line_break_or_space()],
				param_tokens,
			),),
			formatter.format_token(&self.r_paren_token()?)?
		]))
	}
}
