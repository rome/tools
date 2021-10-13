use crate::{
	block_indent, empty_element, format_element::indent, format_elements, group_elements,
	hard_line_break, join_elements, space_token, token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::{CaseClause, DefaultClause, SwitchCase, SwitchStmt};

impl ToFormatElement for SwitchStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let switch =
			formatter.format_token(&self.switch_token().expect("Switch token is mandatory"));
		let condition = formatter.format_node(self.test().expect("Condition is missing"));
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
				block_indent(join_elements(hard_line_break(), cases)),
				r_curly
			])
		]
	}
}

impl ToFormatElement for SwitchCase {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		match self {
			SwitchCase::CaseClause(case_clause) => case_clause.to_format_element(formatter),
			SwitchCase::DefaultClause(default_clause) => {
				default_clause.to_format_element(formatter)
			}
		}
	}
}

impl ToFormatElement for DefaultClause {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let default =
			formatter.format_token(&self.default_token().expect("default token is missing"));

		let colon = formatter.format_token(&self.colon_token().expect("colon token is missing"));

		let statements = self
			.cons()
			.map(|statement| formatter.format_node(statement));

		format_elements![
			default,
			colon,
			space_token(),
			// no line break needed after because it is added by the indent in the switch statement
			indent(format_elements![
				hard_line_break(),
				concat_elements(statements)
			])
		]
	}
}

impl ToFormatElement for CaseClause {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let case_word = formatter.format_token(&self.case_token().expect("case token is missing"));
		let colon = formatter.format_token(&self.colon_token().expect("colon token is missing"));

		let test = formatter.format_node(self.test().expect("Expression is missing"));

		let cons = self
			.cons()
			.map(|statement| formatter.format_node(statement));

		format_elements![
			case_word,
			space_token(),
			test,
			colon,
			// no line break needed after because it is added by the indent in the switch statement
			indent(format_elements![hard_line_break(), concat_elements(cons)])
		]
	}
}
