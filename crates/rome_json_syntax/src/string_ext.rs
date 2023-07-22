use crate::{inner_text, JsonStringValue};
use rome_rowan::{SyntaxResult, SyntaxTokenText};

impl JsonStringValue {
    /// Get the inner text of a string not including the quotes.
    pub fn inner_text(&self) -> SyntaxResult<SyntaxTokenText> {
        Ok(inner_text(&self.value_token()?))
    }
}
