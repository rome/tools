use rome_rowan::{Direction, Language, SyntaxNode, SyntaxToken, TextSize};
use std::collections::BTreeSet;

/// Tracks the ranges of the formatted (including replaced or tokens formatted as verbatim) tokens.
///
/// This implementation uses the fact that no two tokens can have an overlapping range to avoid the need for an interval tree.
/// Thus, testing if a token has already been formatted only requires testing if a token starting at the same offset has been formatted.
#[derive(Debug, Clone, Default)]
pub struct PrintedTokens {
    /// Key: Start of a token's range
    offsets: BTreeSet<TextSize>,
}

impl PrintedTokens {
    /// Tracks a formatted token
    ///
    /// ## Panics
    /// If this token has been formatted before.
    pub fn track_token<L: Language>(&mut self, token: &SyntaxToken<L>) {
        let range = token.text_trimmed_range();

        if !self.offsets.insert(range.start()) {
            panic!("You tried to print the token '{token:?}' twice, and this is not valid.");
        }
    }

    /// Asserts that all tokens of the passed in node have been tracked
    ///
    /// ## Panics
    /// If any descendant token of `root` hasn't been tracked
    pub fn assert_all_tracked<L: Language>(&self, root: &SyntaxNode<L>) {
        let mut descendants = root.descendants_tokens(Direction::Next);
        let mut offsets = self.offsets.iter();

        loop {
            match (descendants.next(), offsets.next()) {
                (Some(descendant), Some(offset)) => match descendant.text_trimmed_range().start() {
                    descendant_offset if descendant_offset < *offset => {
                        panic!("token has not been seen by the formatter: {descendant:#?}.\
                        \nUse `format_replaced` if you want to replace a token from the formatted output.\
                        \nUse `format_removed` if you to remove a token from the formatted output.\n\
                        parent: {:#?}", descendant.parent())
                    }
                    descendant_offset if descendant_offset > *offset => {
                        panic!("tracked offset {offset:?} doesn't match any token of {root:#?}. Have you passed a token from another tree?");
                    }
                    _ => {}
                },
                (Some(descendant), None) => {
                    panic!("token has not been seen by the formatter: {descendant:#?}.\n Use `formatter.format_replaced` if you intentionally remove or replace a token from the formatted output.")
                }
                (None, Some(offset)) => {
                    panic!("tracked offset {offset:?} doesn't match any token of {root:#?}. Have you passed a token from another tree?");
                }
                (None, None) => break,
            };
        }
    }
}
