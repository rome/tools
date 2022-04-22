use rome_rowan::{Language, SyntaxNode, SyntaxToken, TextSize};
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TokenTrackMode {
    Replaced,
    Formatted,
    Verbatim,
}

/// Tracks the ranges of the formatted (including replaced or tokens formatted as verbatim) tokens.
///
/// This implementation uses the fact that no two tokens can have an overlapping range to avoid the need for an interval tree.
/// Thus, testing if a token has already been formatted only requires testing if a token starting at the same offset has been formatted.
#[derive(Debug, Clone, Default)]
pub(crate) struct PrintedTokens {
    /// Key: Start of range, value: whatever this is formatted or replaced
    offsets: BTreeMap<TextSize, TokenTrackMode>,
}

impl PrintedTokens {
    fn track_token<L: Language>(&mut self, token: &SyntaxToken<L>, mode: TokenTrackMode) {
        let range = token.text_trimmed_range();

        if let Some(previous_mode) = self.offsets.insert(range.start(), mode) {
            panic!("You tried to print the token '{token:?}' twice, and this is not valid. Printed now as {mode:?}, previously printed as {previous_mode:?}.");
        }
    }

    /// Tracks a formatted token
    ///
    /// ## Panics
    /// If this token has been formatted before.
    pub(crate) fn track_formatted<L: Language>(&mut self, token: &SyntaxToken<L>) {
        self.track_token(token, TokenTrackMode::Formatted)
    }

    /// Tracks a token that has been replaced with other content
    ///
    /// ## Panics
    /// If this token has been formatted before.
    pub(crate) fn track_replaced<L: Language>(&mut self, token: &SyntaxToken<L>) {
        self.track_token(token, TokenTrackMode::Replaced)
    }

    /// Tracks a verbatim formatted token
    ///
    /// ## Panics
    /// If this token has been formatted before.
    pub(crate) fn track_verbatim<L: Language>(&mut self, token: &SyntaxToken<L>) {
        self.track_token(token, TokenTrackMode::Verbatim)
    }

    /// Asserts that all tokens of the passed in node have been tracked
    ///
    /// ## Panics
    /// If any descendant token of `root` hasn't been tracked
    pub(crate) fn assert_all_tracked<L: Language>(&self, root: &SyntaxNode<L>) {
        let mut descendants = root.descendants_tokens();
        let mut offsets = self.offsets.iter();

        loop {
            match (descendants.next(), offsets.next()) {
                (Some(descendant), Some((offset, mode))) => {
                    match descendant.text_trimmed_range().start() {
                        descendant_offset if descendant_offset < *offset => {
                            panic!("token has not been seen by the formatter: {descendant:#?}")
                        }
                        descendant_offset if descendant_offset > *offset => {
                            panic!("tracked offset {offset:?} formatted with {mode:?} doesn't match any token of {root:#?}");
                        }
                        _ => {}
                    }
                }
                (Some(descendant), None) => {
                    panic!("token has not been seen by the formatter: {descendant:#?}")
                }
                (None, Some((offset, mode))) => {
                    panic!("tracked offset {offset:?} formatted with {mode:?} doesn't match any token of {root:#?}");
                }
                (None, None) => break,
            };
        }
    }
}
