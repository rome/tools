use crate::{
	format_element::indent, format_elements, group_elements, hard_indent, hard_line_break,
	join_elements, space_token, token, FormatElement, ToFormatElement,
};
use rslint_parser::ast::{CaseClause, DefaultClause, SwitchCase, SwitchStmt};

impl ToFormatElement for SwitchStmt {
	fn to_format_element(&self, formatter: &crate::Formatter) -> FormatElement {
		let switch =
			formatter.format_token(&self.switch_token().expect("Switch token is mandatory"));
		let condition = if let Some(condition) = self.test() {
			formatter.format_node(condition)
		} else {
			FormatElement::Empty
		};
		let l_curly =
			formatter.format_token(&self.l_curly_token().expect("Left curly bracket is missing"));

		let cases = self.cases().map(|case| formatter.format_node(case));

		let r_curly = formatter.format_token(
			&self
				.r_curly_token()
				.expect("Right curly bracket is missing"),
		);

		format_elements![
			switch,
			space_token(),
			condition,
			space_token(),
			group_elements(format_elements![
				l_curly,
				hard_indent(join_elements(hard_line_break(), cases)),
				r_curly
			])
		]
	}
}

impl ToFormatElement for SwitchCase {
	fn to_format_element(&self, formatter: &crate::Formatter) -> FormatElement {
		match self {
			SwitchCase::CaseClause(case_clause) => case_clause.to_format_element(formatter),
			SwitchCase::DefaultClause(default_clause) => {
				default_clause.to_format_element(formatter)
			}
		}
	}
}

impl ToFormatElement for DefaultClause {
	fn to_format_element(&self, formatter: &crate::Formatter) -> FormatElement {
		let default = if let Some(token) = self.default_token() {
			formatter.format_token(&token)
		} else {
			token("default")
		};

		let colon = if let Some(colon) = self.colon_token() {
			formatter.format_token(&colon)
		} else {
			token(":")
		};

		let statements = self
			.cons()
			.map(|statement| formatter.format_node(statement));

		format_elements![
			default,
			colon,
			space_token(),
			indent(format_elements![
				hard_line_break(),
				concat_elements(statements)
			])
		]
	}
}

impl ToFormatElement for CaseClause {
	fn to_format_element(&self, formatter: &crate::Formatter) -> FormatElement {
		let case_word = if let Some(token) = self.case_token() {
			formatter.format_token(&token)
		} else {
			token("case")
		};
		let colon = if let Some(colon) = self.colon_token() {
			formatter.format_token(&colon)
		} else {
			token(":")
		};

		let test = formatter.format_node(self.test().expect("Expression is missing"));

		let cons = self
			.cons()
			.map(|statement| formatter.format_node(statement));

		format_elements![
			case_word,
			space_token(),
			test,
			colon,
			// no line break needed after because it is added by the parent
			indent(format_elements![hard_line_break(), concat_elements(cons)])
		]
	}
}
