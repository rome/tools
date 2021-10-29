use crate::{
	concat_elements, format_elements, group_elements, soft_indent, soft_line_break_or_space,
	space_token, token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::{ForHead, ForStmt, ForStmtInit, ForStmtTest, ForStmtUpdate};

impl ToFormatElement for ForStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let inner = if self.init().is_some() || self.test().is_some() || self.update().is_some() {
			dbg!(self);
			let mut inner = vec![];
			if let Some(init) = self.init() {
				inner.push(formatter.format_node(init)?);
			}

			inner.push(token(";"));
			inner.push(soft_line_break_or_space());

			if let Some(test) = self.test() {
				inner.push(formatter.format_node(test)?);
			}

			inner.push(token(";"));
			inner.push(soft_line_break_or_space());

			if let Some(update) = self.update() {
				inner.push(formatter.format_node(update)?);
			}

			concat_elements(inner)
		} else {
			token(";;")
		};

		Some(group_elements(format_elements![
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

impl ToFormatElement for ForStmtInit {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		formatter.format_node(self.inner()?)
	}
}

impl ToFormatElement for ForHead {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self {
			ForHead::VarDecl(decl) => decl.to_format_element(formatter),
			ForHead::Expr(expr) => expr.to_format_element(formatter),
			ForHead::NameRef(name_ref) => name_ref.to_format_element(formatter),
		}
	}
}

impl ToFormatElement for ForStmtTest {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		formatter.format_node(self.expr()?)
	}
}

impl ToFormatElement for ForStmtUpdate {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		formatter.format_node(self.expr()?)
	}
}
