use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::NewTarget;
use rome_js_syntax::NewTargetFields;

impl ToFormatElement for NewTarget {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let NewTargetFields {
            new_token,
            dot_token,
            target_token,
        } = self.as_fields();

        Ok(format_elements![
            new_token.format(formatter)?,
            dot_token.format(formatter)?,
            target_token.format(formatter)?,
        ])
    }
}
