use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, hard_group_elements, space_token, utils::format_decorators, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsConstructorClassMember;
use rome_js_syntax::JsConstructorClassMemberFields;

impl ToFormatElement for JsConstructorClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsConstructorClassMemberFields {
            ts_decorators,
            modifiers,
            name,
            parameters,
            body,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            format_decorators(ts_decorators, &formatter)?,
            modifiers.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            parameters.format(formatter)?,
            space_token(),
            body.format(formatter)?
        ]))
    }
}
