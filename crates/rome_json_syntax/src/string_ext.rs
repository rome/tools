use crate::{inner_text, JsonStringValue};
use rome_rowan::{SyntaxResult, SyntaxTokenText};

impl JsonStringValue {
    pub fn inner_text(&self) -> SyntaxResult<SyntaxTokenText> {
        Ok(inner_text(self.value_token()?))
    }
}
