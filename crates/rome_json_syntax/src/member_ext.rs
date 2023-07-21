use crate::{inner_text, JsonMemberName};
use rome_rowan::{SyntaxResult, SyntaxTokenText};

impl JsonMemberName {
    pub fn inner_text(&self) -> SyntaxResult<SyntaxTokenText> {
        Ok(inner_text(self.value_token()?))
    }
}
