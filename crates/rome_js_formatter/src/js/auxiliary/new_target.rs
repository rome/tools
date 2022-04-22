use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::NewTarget;
use rome_js_syntax::NewTargetFields;

impl FormatNode for NewTarget {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
