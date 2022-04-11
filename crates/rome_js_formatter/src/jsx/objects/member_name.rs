use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_formatter::format_elements;
use rome_js_syntax::{JsxMemberName, JsxMemberNameFields};

impl ToFormatElement for JsxMemberName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxMemberNameFields {
            object,
            dot_token,
            member,
        } = self.as_fields();

        Ok(format_elements![
            object.format(formatter)?,
            dot_token.format(formatter)?,
            member.format(formatter)?,
        ])
    }
}
