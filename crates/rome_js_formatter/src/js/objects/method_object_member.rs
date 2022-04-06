use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::hard_group_elements;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsMethodObjectMember;
use rome_js_syntax::JsMethodObjectMemberFields;

impl ToFormatElement for JsMethodObjectMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsMethodObjectMemberFields {
            async_token,
            star_token,
            name,
            type_parameters,
            parameters,
            return_type_annotation,
            body,
        } = self.as_fields();

        let async_token = async_token.format_with_or_empty(formatter, |async_token| {
            format_elements![async_token, space_token()]
        })?;
        let star_token = star_token.format_or_empty(formatter)?;
        Ok(hard_group_elements(format_elements![
            async_token,
            star_token,
            name.format(formatter)?,
            type_parameters.format_or_empty(formatter)?,
            parameters.format(formatter)?,
            return_type_annotation.format_or_empty(formatter)?,
            space_token(),
            body.format(formatter)?,
        ]))
    }
}
