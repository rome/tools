use crate::{
	concat_elements, format_elements, group_elements, soft_indent, soft_line_break_or_space,
	space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{ForStmt, ForStmtTest, ForStmtUpdate, JsAnyForInitializer};

impl ToFormatElement for ForStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let inner =
			if self.initializer().is_some() || self.test().is_some() || self.update().is_some() {
				let mut inner = vec![];
				if let Some(init) = self.initializer() {
					inner.push(formatter.format_node(init)?);
				}

				inner.push(formatter.format_token(&self.first_semi_token()?)?);
				inner.push(soft_line_break_or_space());

				if let Some(test) = self.test() {
					inner.push(formatter.format_node(test)?);
				}

				inner.push(formatter.format_token(&self.second_semi_token()?)?);
				inner.push(soft_line_break_or_space());

				if let Some(update) = self.update() {
					inner.push(formatter.format_node(update)?);
				}

				concat_elements(inner)
			} else {
				format_elements![
					formatter.format_token(&self.first_semi_token()?)?,
					formatter.format_token(&self.second_semi_token()?)?
				]
			};

		Ok(group_elements(format_elements![
			formatter.format_token(&self.for_token()?)?,
			space_token(),
			formatter.format_token(&self.l_paren_token()?)?,
			group_elements(soft_indent(inner)),
			formatter.format_token(&self.r_paren_token()?)?,
			space_token(),
			formatter.format_node(self.cons()?)?
		]))
	}
}

impl ToFormatElement for JsAnyForInitializer {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyForInitializer::JsVariableDeclarations(decl) => decl.to_format_element(formatter),
			JsAnyForInitializer::JsAnyExpression(expr) => expr.to_format_element(formatter),
		}
	}
}

impl ToFormatElement for ForStmtTest {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_node(self.expr()?)
	}
}

impl ToFormatElement for ForStmtUpdate {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_node(self.expr()?)
	}
}
