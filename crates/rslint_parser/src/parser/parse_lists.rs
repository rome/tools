///! A set of traits useful to parse various type of lists
use super::{ParsedSyntax, ParserProgress, RecoveryResult};
use crate::{CompletedMarker, Marker, ParseRecovery, Parser};
use rslint_errors::Diagnostic;
use rslint_syntax::SyntaxKind;
use std::ops::Range;

/// Use this trait to parse simple lists
pub trait ParseList {
	/// The type contained inside [ParsedSyntax]
	type FinishedSyntax;

	/// Parses a simple list
	///
	/// # Panics
	///
	/// It panics if the parser doesn't advance at each cycle of the loop
	fn parse_list(&mut self, p: &mut Parser) -> ParsedSyntax<Self::FinishedSyntax> {
		let elements = self.start_list(p);
		let mut progress = ParserProgress::default();
		while !p.at(SyntaxKind::EOF) && self.is_at_list_end(p) {
			progress.assert_progressing(p);

			let parsed_element = self.parse_element(p);

			if parsed_element.is_absent() && self.recover(p, parsed_element).is_err() {
				break;
			}
		}
		self.finish_list(p, elements)
	}

	/// Parses a single element of the list
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<CompletedMarker>;

	/// Tells the parser to mark the current token as missing, continuing the loop.
	/// This function is used for [Self::parse_list].
	fn parse_separating_element(&mut self, p: &mut Parser) {
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
	fn finish_list(&mut self, p: &mut Parser, m: Marker) -> ParsedSyntax<Self::FinishedSyntax>;

	/// This method is used to check the current token inside the loop. When this method return [false],
	/// the trait will exit from the loop.
	///
	/// Usually here you want to check the current token.
	fn is_at_list_end(&mut self, p: &mut Parser) -> bool;

	/// This method is used to recover the parser in case [Self::parse_element] returns [ParsedSyntax::Absent]
	fn recover(
		&mut self,
		p: &mut Parser,
		parsed_element: ParsedSyntax<CompletedMarker>,
	) -> RecoveryResult {
		parsed_element.or_recover(p, &Self::recovery(), Self::expected_element_error)
	}

	/// [Diagnostic] thrown in case the parser is not able to recover
	fn expected_element_error(p: &Parser, range: Range<usize>) -> Diagnostic;

	/// A [crate::TokenSet] that will be given to the parser in order to recover
	fn recovery() -> ParseRecovery;
}

/// A trait to parse lists that will be separated by a recurring element
pub trait ParseSeparatedList {
	/// The type contained inside [ParsedSyntax]
	type FinishedSyntax;

	/// Parses a single element of the list
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<CompletedMarker>;

	/// Tells the parser to mark the current token as missing, continuing the loop.
	/// This function is used for [Self::parse_list].
	fn parse_separating_element(&mut self, p: &mut Parser) {
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
	fn finish_list(&mut self, p: &mut Parser, m: Marker) -> ParsedSyntax<Self::FinishedSyntax>;

	/// This method is used to check if the parser is at the end of the list
	fn is_at_list_end(&mut self, p: &mut Parser) -> bool;

	/// When calling [Self::parse_list], this method checks, inside the loop, if the parser
	/// is at a position where the current token is element that will separate the list.
	///
	/// Usually here you want to check the current token.
	fn is_at_separating_element(&mut self, _p: &mut Parser) -> bool;

	/// Parses a list of elements separated by a recurring element
	///
	/// # Panics
	///
	/// It panics if the parser doesn't advance at each cycle of the loop
	fn parse_list(&mut self, p: &mut Parser) -> ParsedSyntax<Self::FinishedSyntax> {
		let elements = self.start_list(p);
		let mut progress = ParserProgress::default();
		while !p.at(SyntaxKind::EOF) && self.is_at_list_end(p) {
			progress.assert_progressing(p);

			if self.is_at_separating_element(p) {
				self.parse_separating_element(p);
				continue;
			}

			let parsed_element = self.parse_element(p);

			if parsed_element.is_absent() && self.recover(p, parsed_element).is_err() {
				break;
			}
		}
		self.finish_list(p, elements)
	}

	/// This method is used to recover the parser in case [Self::parse_element] returns [ParsedSyntax::Absent]
	fn recover(
		&mut self,
		p: &mut Parser,
		parsed_element: ParsedSyntax<CompletedMarker>,
	) -> RecoveryResult {
		parsed_element.or_recover(p, &Self::recovery(), Self::expected_element_error)
	}

	/// [Diagnostic] thrown in case the parser is not able to recover
	fn expected_element_error(p: &Parser, range: Range<usize>) -> Diagnostic;

	/// A [crate::TokenSet] that will be given to the parser in order to recover
	fn recovery() -> ParseRecovery;
}
