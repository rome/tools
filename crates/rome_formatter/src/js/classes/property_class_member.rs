use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, utils::format_decorators, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};

use rome_js_syntax::JsPropertyClassMember;
use rome_js_syntax::JsPropertyClassMemberFields;

impl ToFormatElement for JsPropertyClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPropertyClassMemberFields {
            ts_decorators,
            modifiers,
            name,
            property_annotation,
            value,
            semicolon_token,
        } = self.as_fields();

        let property_annotation = property_annotation.format_or_empty(formatter)?;

        let init =
            value.format_with_or_empty(formatter, |node| format_elements![space_token(), node])?;

        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            format_decorators(ts_decorators, &formatter)?,
            modifiers.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            property_annotation,
            init,
            semicolon
        ])
    }
}
