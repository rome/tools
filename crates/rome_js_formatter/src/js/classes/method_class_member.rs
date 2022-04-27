use crate::format_traits::FormatOptional;
use crate::utils::PropertyNameCheckMode;
use crate::{
    format_elements, space_token, utils::format_property_name, FormatElement, FormatNode, Formatter,
};
use crate::{hard_group_elements, Format};
use rome_formatter::FormatResult;
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

        let async_token = async_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let star_token = star_token.format_or_empty(formatter)?;
        let question_mark_token = question_mark_token.format_or_empty(formatter)?;
        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let params = parameters.format(formatter)?;
        let return_type_annotation = return_type_annotation.format_or_empty(formatter)?;
        let body = body.format(formatter)?;

        Ok(hard_group_elements(format_elements![
            modifiers.format(formatter)?,
            space_token(),
            async_token,
            star_token,
            format_property_name(name?, formatter, PropertyNameCheckMode::Alphanumeric)?,
            question_mark_token,
            type_parameters,
            params,
            return_type_annotation,
            space_token(),
            body
        ]))
    }
}
