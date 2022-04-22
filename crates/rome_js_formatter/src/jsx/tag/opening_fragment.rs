use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::format_elements;
use rome_formatter::FormatResult;
use rome_js_syntax::{JsxOpeningFragment, JsxOpeningFragmentFields};

impl FormatNode for JsxOpeningFragment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxOpeningFragmentFields {
            r_angle_token,
            l_angle_token,
        } = self.as_fields();

        Ok(format_elements![
            l_angle_token.format(formatter)?,
            r_angle_token.format(formatter)?
        ])
    }
}
