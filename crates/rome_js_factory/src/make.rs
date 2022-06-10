use rome_js_syntax::{JsSyntaxKind, JsSyntaxToken, TextSize, TriviaPieceKind};
use rome_rowan::TriviaPiece;

pub use crate::generated::node_factory::*;

/// Create a new identifier token with no attached trivia
pub fn ident(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(JsSyntaxKind::IDENT, text, [], [])
}

/// Create a new string literal token with no attached trivia
pub fn js_string_literal(text: &str) -> JsSyntaxToken {
    JsSyntaxToken::new_detached(
        JsSyntaxKind::JS_STRING_LITERAL,
        &format!("\"{text}\""),
        [],
        [],
    )
}

/// Create a new token with the specified syntax kind and no attached trivia
pub fn token(kind: JsSyntaxKind) -> JsSyntaxToken {
    if let Some(text) = kind.to_string() {
        JsSyntaxToken::new_detached(kind, text, [], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// Create a new token with the specified syntax kind, and a whitespace trivia
/// piece on both the leading and trailing positions
pub fn token_decorated_with_space(kind: JsSyntaxKind) -> JsSyntaxToken {
    if let Some(text) = kind.to_string() {
        JsSyntaxToken::new_detached(
            kind,
            &format!(" {text} "),
            [TriviaPiece::new(TriviaPieceKind::Whitespace, 1)],
            [TriviaPiece::new(TriviaPieceKind::Whitespace, 1)],
        )
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// Returns a clone of `token` with the leading trivia trimmed at the first
/// newline piece (included), starting from the token itself and iterating backward
///
/// # Example
/// ```
/// # use rome_rowan::TriviaPiece;
/// # use rome_js_syntax::{JsSyntaxToken, T};
/// # use rome_js_factory::make::clone_token_up_to_first_newline;
/// let input = JsSyntaxToken::new_detached(
///     T![let],
///     "\n  // Comment\n  let ",
///     [TriviaPiece::newline(1), TriviaPiece::whitespace(2), TriviaPiece::single_line_comment(10), TriviaPiece::newline(1), TriviaPiece::whitespace(2)],
///     [TriviaPiece::whitespace(1)],
/// );
///
/// let output = clone_token_up_to_first_newline(&input);
///
/// let expected = JsSyntaxToken::new_detached(
///     T![let],
///     "\n  let ",
///     [TriviaPiece::newline(1), TriviaPiece::whitespace(2)],
///     [TriviaPiece::whitespace(1)],
/// );
///
/// assert_eq!(output.text(), expected.text());
/// ```
pub fn clone_token_up_to_first_newline(token: &JsSyntaxToken) -> JsSyntaxToken {
    let leading_trivia = token.leading_trivia().pieces();
    let num_pieces = leading_trivia.len();
    let skip_count = leading_trivia
        .rev()
        .position(|piece| piece.is_newline())
        .and_then(|index| num_pieces.checked_sub(index + 1))
        .unwrap_or(0);

    let mut text = String::new();

    for piece in token.leading_trivia().pieces().skip(skip_count) {
        text.push_str(piece.text());
    }

    text.push_str(token.text_trimmed());

    for piece in token.trailing_trivia().pieces() {
        text.push_str(piece.text());
    }

    JsSyntaxToken::new_detached(
        token.kind(),
        &text,
        token
            .leading_trivia()
            .pieces()
            .skip(skip_count)
            .map(|piece| TriviaPiece::new(piece.kind(), TextSize::of(piece.text()))),
        token
            .trailing_trivia()
            .pieces()
            .map(|piece| TriviaPiece::new(piece.kind(), TextSize::of(piece.text()))),
    )
}
