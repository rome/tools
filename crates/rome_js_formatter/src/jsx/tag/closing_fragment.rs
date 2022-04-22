use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatNode, Formatter};
use rome_formatter::format_elements;
use rome_formatter::FormatResult;
use rome_js_syntax::{JsxClosingFragment, JsxClosingFragmentFields};
use rome_rowan::AstNode;

impl FormatNode for JsxClosingFragment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxClosingFragmentFields {
            r_angle_token,
            slash_token,
            l_angle_token,
        } = self.as_fields();

        Ok(format_elements![
            l_angle_token.format(formatter)?,
            slash_token.format(formatter)?,
            r_angle_token.format(formatter)?
        ])
    }
}
