use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::{format_elements, FormatResult};
use rome_js_syntax::{JsxMemberName, JsxMemberNameFields};

impl FormatNode for JsxMemberName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
