use crate::{JsonStringValue, TextSize};
use rome_rowan::{SyntaxResult, SyntaxTokenText};

impl JsonStringValue {
    pub fn inner_string_text(&self) -> SyntaxResult<SyntaxTokenText> {
        let value = self.value_token()?;
        let mut text = value.token_text_trimmed();

        static QUOTES: [char; 1] = ['"'];

        if text.starts_with(QUOTES) {
            let range = text.range().add_start(TextSize::from(1));
            text = text.slice(range);
        }
        if text.ends_with(QUOTES) {
            let range = text.range().sub_end(TextSize::from(1));
            text = text.slice(range);
        }

        Ok(text)
    }
}
