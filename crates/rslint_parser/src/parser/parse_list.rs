use std::ops::Range;

use super::ParsedSyntax::Present;
use super::{ParsedSyntax, ParserProgress, RecoveryResult};
use crate::{CompletedMarker, Marker, ParseRecovery, Parser};
use rslint_errors::Diagnostic;
use rslint_syntax::SyntaxKind;

/// An utility that gives finer control on how to parse a list of elements
pub trait ParseList {
	/// Parses a simple list
	///
	/// # Panics
	///
	/// It panics if the parser doesn't advance at each cycle of the loop
	fn parse_list(&mut self, p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
		let elements = self.start_list(p);
		let mut progress = ParserProgress::default();
		while !p.at(SyntaxKind::EOF) && !self.is_at(p) {
			progress.assert_progressing(p);

			let parsed_element = self.parse_element(p);

			if parsed_element.is_absent() && self.recover(p, parsed_element).is_err() {
				break;
			}
		}
		self.finish_list(p, elements)
	}

	/// Parses a list of elements separated by a recurring element
	///
	/// # Panics
	///
	/// It panics if the parser doesn't advance at each cycle of the loop
	fn parse_separated_list(&mut self, p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
		let elements = self.start_list(p);
		let mut progress = ParserProgress::default();
		while !p.at(SyntaxKind::EOF) && !self.is_at(p) {
			progress.assert_progressing(p);

			if self.is_at_missing_element(p) {
				self.parse_missing_element(p);
				continue;
			}

			let parsed_element = self.parse_element(p);

			if parsed_element.is_absent() && self.recover(p, parsed_element).is_err() {
				break;
			}
		}
		self.finish_list(p, elements)
	}

	/// Parses a single element of the list
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<CompletedMarker>;

	/// Tells the parser to mark the current token as missing, continuing the loop
	fn parse_missing_element(&mut self, p: &mut Parser) {
		p.missing();
	}

	/// The [SyntaxKind] used to name the list
	fn list_kind(&mut self) -> SyntaxKind {
		SyntaxKind::LIST
	}

	/// It creates a marker just before starting a list
	fn start_list(&mut self, p: &mut Parser) -> Marker {
		p.start()
	}

	/// It creates a [ParsedSyntax] that will contain the list
	fn finish_list(&mut self, p: &mut Parser, m: Marker) -> ParsedSyntax<CompletedMarker> {
		Present(m.complete(p, self.list_kind()))
	}

	/// This method is used to check the current token inside the loop. When this method return [false],
	/// the trait will exit from the loop.
	fn is_at(&mut self, p: &mut Parser) -> bool;

	/// When calling [parse_separated_list], this method checks, inside the loop, if the parser
	/// is inside a token that marks the
	fn is_at_missing_element(&mut self, _p: &mut Parser) -> bool {
		unimplemented!("When calling `parse_separated_list`, you need to implement this method.");
	}

	/// This method is used to recover the parser in case [parse_element] returns [ParsedSyntax::Absent]
	fn recover(
		&mut self,
		p: &mut Parser,
		parsed_element: ParsedSyntax<CompletedMarker>,
	) -> RecoveryResult {
		parsed_element.or_recover(p, &Self::recovery(), Self::expected_element_error)
	}

	/// [Diagnostic] thrown in case the parser is not able to recover
	fn expected_element_error(p: &Parser, range: Range<usize>) -> Diagnostic;

	/// A [TokenSet] that will be given to the parser in order to recover
	fn recovery() -> ParseRecovery;
}
