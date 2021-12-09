///! A set of traits useful to parse various types of lists
use super::{ParsedSyntax, ParserProgress, RecoveryResult};
use crate::{Marker, Parser};
use rslint_syntax::SyntaxKind;

/// Use this trait to parse simple lists that don't have particular requirements.
///
/// In order to use this trait, you need to implement the [List] trait too.
///
/// ```rust,ignore
/// use rslint_parser::{ParseSeparatedList};
///
/// struct MyList;
///
///
/// impl ParseNormalList for MyList {
///   // impl missing members
/// }
/// ```
pub trait ParseNodeList {
	/// The type returned when calling the function [Self::parse_element]
	type ParsedElement;

	/// Parses a single element of the list
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement>;

	/// It creates a marker just before starting a list
	fn start_list(&mut self, p: &mut Parser) -> Marker {
		p.start()
	}

	/// This method is used to check the current token inside the loop. When this method return [false],
	/// the trait will exit from the loop.
	///
	/// Usually here you want to check the current token.
	fn is_at_list_end(&mut self, p: &mut Parser) -> bool;

	/// This method is used to recover the parser in case [Self::parse_element] returns [ParsedSyntax::Absent]
	fn recover(
		&mut self,
		p: &mut Parser,
		parsed_element: ParsedSyntax<Self::ParsedElement>,
	) -> RecoveryResult;

	/// It creates a [ParsedSyntax] that will contain the list
	fn finish_list(&mut self, p: &mut Parser, m: Marker) {
		m.complete(p, SyntaxKind::LIST);
	}
	/// Parses a simple list
	///
	/// # Panics
	///
	/// It panics if the parser doesn't advance at each cycle of the loop
	fn parse_list(&mut self, p: &mut Parser) {
		let elements = self.start_list(p);
		let mut progress = ParserProgress::default();
		while !p.at(SyntaxKind::EOF) && !self.is_at_list_end(p) {
			progress.assert_progressing(p);

			let parsed_element = self.parse_element(p);

			if self.recover(p, parsed_element).is_err() {
				break;
			}
		}
		self.finish_list(p, elements);
	}
}

/// A trait to parse lists that will be separated by a recurring element
///
/// In order to use this trait, you need to implement the [List] trait too.
///
/// ```rust,ignore
/// use rslint_parser::{ParseSeparatedList};
///
/// struct MyList;
///
/// impl ParseSeparatedList for MyList {
///   // impl missing members
/// }
/// ```
pub trait ParseSeparatedList {
	/// The type returned when calling the function [Self::parse_element]
	type ParsedElement;

	/// Parses a single element of the list
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement>;

	/// It creates a marker just before starting a list
	fn start_list(&mut self, p: &mut Parser) -> Marker {
		p.start()
	}

	/// This method is used to check the current token inside the loop. When this method return [false],
	/// the trait will exit from the loop.
	///
	/// Usually here you want to check the current token.
	fn is_at_list_end(&mut self, p: &mut Parser) -> bool;

	/// This method is used to recover the parser in case [Self::parse_element] returns [ParsedSyntax::Absent]
	fn recover(
		&mut self,
		p: &mut Parser,
		parsed_element: ParsedSyntax<Self::ParsedElement>,
	) -> RecoveryResult;

	/// It creates a [ParsedSyntax] that will contain the list
	/// Only called if the list isn't empty
	fn finish_list(&mut self, p: &mut Parser, m: Marker) {
		m.complete(p, SyntaxKind::LIST);
	}

	/// The [SyntaxKind] of the element that separates the elements of the list
	fn separating_element_kind(&mut self) -> SyntaxKind;

	/// `true` if the list allows for an optional trailing comma
	fn allow_trailing_comma(&self) -> bool {
		false
	}

	/// Method called at each iteration of the the loop and checks if the expected
	/// separator is present.
	///
	/// If present, it [parses](Self::parse_separating_element) it and continues with loop.
	/// If not present, it adds a missing marker.
	fn expect_separator(&mut self, p: &mut Parser) -> bool {
		p.expect_required(self.separating_element_kind())
	}

	/// Parses a list of elements separated by a recurring element
	///
	/// # Panics
	///
	/// It panics if the parser doesn't advance at each cycle of the loop
	fn parse_list(&mut self, p: &mut Parser) {
		let elements = self.start_list(p);
		let mut progress = ParserProgress::default();
		let mut first = true;

		while !p.at(SyntaxKind::EOF) && !self.is_at_list_end(p) {
			progress.assert_progressing(p);

			if first {
				first = false;
			} else {
				self.expect_separator(p);

				if self.allow_trailing_comma() && self.is_at_list_end(p) {
					break;
				}
			}

			let parsed_element = self.parse_element(p);

			if self.recover(p, parsed_element).is_err() {
				break;
			}
		}

		if first {
			elements.abandon(p);
			p.missing();
		} else {
			self.finish_list(p, elements);
		}
	}
}
