use crate::{JsonStringValue, TextSize};
use rome_rowan::{SyntaxResult, TextRange, TokenText};

impl JsonStringValue {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        let value = self.value_token()?;
        let mut text = value.token_text_trimmed();

        static QUOTES: [char; 1] = ['"'];

        if text.starts_with(QUOTES) {
            let range = TextRange::new(1.into(), text.len());
            text = text.slice(range);
        }

        if text.ends_with(QUOTES) {
            let range = TextRange::new(0.into(), text.len() - TextSize::from(1));
            text = text.slice(range);
        }

        Ok(text)
    }
}
