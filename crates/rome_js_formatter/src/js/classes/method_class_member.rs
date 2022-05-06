use crate::format_traits::FormatOptional;
use crate::{formatted, hard_group_elements, Format};
use rome_formatter::FormatResult;

use crate::{space_token, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsMethodClassMember;
use rome_js_syntax::JsMethodClassMemberFields;

impl FormatNode for JsMethodClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsMethodClassMemberFields {
            modifiers,
            async_token,
            star_token,
            name,
            question_mark_token,
            type_parameters,
            parameters,
            return_type_annotation,
            body,
        } = self.as_fields();

        let async_token = async_token.format_with_or_empty(formatter, |token| {
            formatted![formatter, token, space_token()]
        })?;

        let name = name.format(formatter)?;
        let params = parameters.format(formatter)?;
        let body = body.format(formatter)?;

        Ok(hard_group_elements(formatted![
            formatter,
            modifiers.format(formatter)?,
            space_token(),
            async_token,
            star_token,
            name,
            question_mark_token,
            type_parameters,
            params,
            return_type_annotation,
            space_token(),
            body
        ]?))
    }
}
