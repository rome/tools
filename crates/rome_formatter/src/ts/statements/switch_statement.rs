use crate::ts::statements::format_statements;
use crate::{
	block_indent, format_element::indent, format_elements, group_elements, hard_line_break,
	join_elements, space_token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::{CaseClause, DefaultClause, SwitchCase, SwitchStmt};

impl ToFormatElement for SwitchStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let switch = formatter.format_token(&self.switch_token()?)?;
		let condition = formatter.format_node(self.test()?)?;
		let l_curly = formatter.format_token(&self.l_curly_token()?)?;

		let cases = formatter.format_children(self.cases())?;
		let r_curly = formatter.format_token(&self.r_curly_token()?)?;

		Some(format_elements![
			switch,
			space_token(),
			condition,
			space_token(),
			group_elements(format_elements![
				l_curly,
				block_indent(join_elements(hard_line_break(), cases)),
				r_curly
			])
		])
	}
}

impl ToFormatElement for SwitchCase {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self {
			SwitchCase::CaseClause(case_clause) => case_clause.to_format_element(formatter),
			SwitchCase::DefaultClause(default_clause) => {
				default_clause.to_format_element(formatter)
			}
		}
	}
}

impl ToFormatElement for DefaultClause {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let default = formatter.format_token(&self.default_token()?)?;
		let colon = formatter.format_token(&self.colon_token()?)?;
		let statements = format_statements(self.cons(), formatter);

		Some(format_elements![
			default,
			colon,
			space_token(),
			// no line break needed after because it is added by the indent in the switch statement
			indent(format_elements![hard_line_break(), statements])
		])
	}
}

impl ToFormatElement for CaseClause {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let case_word = formatter.format_token(&self.case_token()?)?;
		let colon = formatter.format_token(&self.colon_token()?)?;

		let test = formatter.format_node(self.test()?)?;

		let cons = format_statements(self.cons(), formatter);

		Some(format_elements![
			case_word,
			space_token(),
			test,
			colon,
			// no line break needed after because it is added by the indent in the switch statement
			indent(format_elements![hard_line_break(), cons])
		])
	}
}
