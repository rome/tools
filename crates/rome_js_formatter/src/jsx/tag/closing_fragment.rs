use crate::prelude::*;
use rome_js_syntax::{JsxClosingFragment, JsxClosingFragmentFields};

impl FormatNode for JsxClosingFragment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxClosingFragmentFields {
            r_angle_token,
            slash_token,
            l_angle_token,
        } = self.as_fields();

        formatted![
            formatter,
            l_angle_token.format(formatter)?,
            slash_token.format(formatter)?,
            r_angle_token.format(formatter)?
        ]
    }
}
