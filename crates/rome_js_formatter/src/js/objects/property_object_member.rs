use crate::{
    formatted, space_token, Format, FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;

use rome_js_syntax::JsPropertyObjectMember;
use rome_js_syntax::JsPropertyObjectMemberFields;

impl FormatNode for JsPropertyObjectMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPropertyObjectMemberFields {
            name,
            colon_token,
            value,
        } = self.as_fields();

        let key = name.format(formatter)?;
        let colon = colon_token.format(formatter)?;
        let value = value.format(formatter)?;
        formatted![formatter, key, colon, space_token(), value]
    }
}
