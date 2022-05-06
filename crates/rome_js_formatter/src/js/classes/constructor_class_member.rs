use crate::{
    formatted, hard_group_elements, space_token, Format, FormatElement,
    FormatNode, Formatter,
};
use rome_formatter::FormatResult;

use rome_js_syntax::JsConstructorClassMember;
use rome_js_syntax::JsConstructorClassMemberFields;

impl FormatNode for JsConstructorClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsConstructorClassMemberFields {
            modifiers,
            name,
            parameters,
            body,
        } = self.as_fields();

        Ok(hard_group_elements(formatted![
            formatter,
            modifiers.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            parameters.format(formatter)?,
            space_token(),
            body.format(formatter)?
        ]?))
    }
}
