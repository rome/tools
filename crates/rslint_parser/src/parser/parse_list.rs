use super::ParsedSyntax::Present;
use super::{ParsedSyntax, ParserProgress};
use crate::{CompletedMarker, ParseRecovery, Parser};
use rslint_errors::Diagnostic;
use rslint_syntax::SyntaxKind;
use std::ops::Range;

/// An utility that gives some control on how to parse a list of elements
pub struct ParseList<Condition, CreateElement> {
	/// This is a condition used inside the loop. When it's false, it exists the loop
	condition: Option<Condition>,
	/// A closure that is charge of creating the element inside the loop
	create_element: Option<CreateElement>,
	/// The [SyntaxKind] of the list
	list_kind: Option<SyntaxKind>,
}

impl<Condition, CreateElement> ParseList<Condition, CreateElement>
where
	Condition: Fn(&mut Parser) -> bool,
	CreateElement: Fn(&mut Parser) -> ParsedSyntax<CompletedMarker>,
{
	pub fn new() -> Self {
		Self {
			condition: None,
			create_element: None,
			list_kind: None,
		}
	}

	#[must_use = "You need to provide the condition to exit from the loop"]
	#[inline]
	pub fn set_condition(mut self, condition: Condition) -> Self {
		self.condition = Some(condition);
		self
	}

	#[must_use = "You need provide the function that creates the single element list"]
	#[inline]
	pub fn set_create_element(mut self, create_element: CreateElement) -> Self {
		self.create_element = Some(create_element);
		self
	}

	#[must_use = "You need to tell the struct the [SyntaxKind] of the list"]
	#[inline]
	pub fn set_list_kind(mut self, list_kind: SyntaxKind) -> Self {
		self.list_kind = Some(list_kind);
		self
	}

	/// This function is in charge to create a list without any sort of recovering.
	///
	/// When using it, if [create_element] returns a [ParsedSyntax::Absent], it will
	/// mark it as a missing element
	pub fn create_list(self, p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
		let condition = self
			.condition
			.expect("`condition` can't be `None`, use set_condition");
		let create_element = self
			.create_element
			.expect("`create_element` can't be `None`, use set_condition");

		let list_kind = self
			.list_kind
			.expect("`list_kind` can't be `None`, use set_condition");

		let elements = p.start();
		let mut progress = ParserProgress::default();
		while (condition)(p) {
			progress.assert_progressing(p);
			(create_element)(p).or_missing(p);
		}

		Present(elements.complete(p, list_kind))
	}

	/// This function is in charge to create a list without any sort of recovering.
	///
	/// When using it, if [create_element] returns a [ParsedSyntax::Absent], it will
	/// mark it as a missing and a diagnostic error will be emitted
	#[allow(unused)]
	pub fn create_list_with_missing_error<E>(
		self,
		p: &mut Parser,
		error_builder: E,
	) -> ParsedSyntax<CompletedMarker>
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic + Copy,
	{
		let condition = self
			.condition
			.expect("`condition` can't be `None`, use set_condition");
		let create_element = self
			.create_element
			.expect("`create_element` can't be `None`, use set_condition");

		let list_kind = self
			.list_kind
			.expect("`list_kind` can't be `None`, use set_condition");

		let elements = p.start();
		let mut progress = ParserProgress::default();
		while (condition)(p) {
			progress.assert_progressing(p);
			(create_element)(p).or_missing_with_error(p, error_builder);
		}

		Present(elements.complete(p, list_kind))
	}

	/// A list that allows the parser to recover from what's returned from [create_element].
	///
	/// The function needs to accept a [ParseRecovery] and an error builder.
	#[allow(unused)]
	pub fn create_list_with_recover<E>(
		self,
		p: &mut Parser,
		recovery: &ParseRecovery,
		error_builder: E,
	) -> ParsedSyntax<CompletedMarker>
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic + Copy,
	{
		let condition = self
			.condition
			.expect("`condition` can't be `None`, use set_condition");
		let create_element = self
			.create_element
			.expect("`create_element` can't be `None`, use set_condition");

		let list_kind = self
			.list_kind
			.expect("`list_kind` can't be `None`, use set_condition");

		let elements = p.start();
		let mut progress = ParserProgress::default();
		while (condition)(p) {
			progress.assert_progressing(p);

			let parsed_element = (create_element)(p);

			let recovered_element = parsed_element.or_recover(p, recovery, error_builder);

			if recovered_element.is_err() {
				break;
			}
		}
		Present(elements.complete(p, list_kind))
	}
}

#[cfg(test)]
mod test {
	use super::ParseList;
	use super::ParsedSyntax::Present;
	use crate::{Parser, Syntax, TokenSource};
	use rslint_lexer::Token;
	use rslint_lexer::T;
	use rslint_syntax::SyntaxKind;

	#[test]
	fn can_new() {
		let tokens = vec![Token::new(SyntaxKind::JS_STRING_LITERAL, 12)];
		let token_source = TokenSource::new("await", tokens.as_slice());

		let mut p = Parser::new(token_source, 0, Syntax::default());

		let list = ParseList::new()
			.set_condition(|p| p.at(T![#]))
			.set_create_element(|p| {
				let m = p.start();
				p.bump_any();
				Present(m.complete(p, rslint_lexer::SyntaxKind::JS_UNARY_EXPRESSION))
			})
			.set_list_kind(rslint_lexer::SyntaxKind::LIST)
			.create_list(&mut p);

		let marker = list.unwrap();

		assert_eq!(marker.kind(), rslint_lexer::SyntaxKind::LIST);
	}
}
