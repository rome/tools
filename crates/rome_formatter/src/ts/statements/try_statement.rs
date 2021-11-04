use crate::{
	empty_element, format_elements, space_token, token, FormatElement, FormatError, Formatter,
	ToFormatElement,
};
use rslint_parser::{
	ast::{CatchClause, Finalizer, TryStmt},
	AstNode,
};

impl ToFormatElement for TryStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let try_token = formatter.format_token(&self.try_token()?)?;
		let test = formatter.format_node(self.test()?)?;
		let handler = if let Some(catch_clause) = self.handler() {
			format_elements![space_token(), formatter.format_node(catch_clause)?]
		} else {
			empty_element()
		};
		let finalizer = if let Some(finally_node) = self.finalizer() {
			format_elements![space_token(), formatter.format_node(finally_node)?]
		} else {
			empty_element()
		};
		if handler.is_empty() && finalizer.is_empty() {
			Err(FormatError::MissingNode(self.syntax().kind()))
		} else {
			Ok(format_elements![
				try_token,
				space_token(),
				test,
				handler,
				finalizer
			])
		}
	}
}

impl ToFormatElement for Finalizer {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let cons = formatter.format_node(self.cons()?)?;
		let finally = formatter.format_token(&self.finally_token()?)?;
		Ok(format_elements![finally, space_token(), cons])
	}
}

impl ToFormatElement for CatchClause {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let l_paren = self.l_paren_token();
		let r_paren = self.r_paren_token();
		let error = self.error();
		let cons = formatter.format_node(self.cons()?)?;
		let catch_token = formatter.format_token(&self.catch_token()?)?;
		// TODO: #1725 this will change once we have a better grammar
		match (l_paren, r_paren, error) {
			(Err(_), Err(_), Err(_)) => Ok(format_elements![token("catch"), space_token(), cons]),
			(Ok(l_paren), Ok(r_paren), Ok(error)) => Ok(format_elements![
				catch_token,
				space_token(),
				formatter.format_token(&l_paren)?,
				formatter.format_node(error)?,
				formatter.format_token(&r_paren)?,
				space_token(),
				cons
			]),
			_ => {
				// Here we return None, because a valid catch clause must have a condition or no condition at all:
				// - catch (e) {}
				// - catch {}
				//
				// Other cases should fail.
				Err(FormatError::MissingNode(self.syntax().kind()))
			}
		}
	}
}
