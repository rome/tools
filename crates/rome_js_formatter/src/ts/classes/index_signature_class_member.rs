use crate::utils::format_with_semicolon;
use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsIndexSignatureClassMember;
use rome_js_syntax::TsIndexSignatureClassMemberFields;

impl FormatNode for TsIndexSignatureClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsIndexSignatureClassMemberFields {
            modifiers,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(
            formatter,
            format_elements![
                modifiers.format(formatter)?,
                space_token(),
                l_brack_token.format(formatter)?,
                parameter.format(formatter)?,
                r_brack_token.format(formatter)?,
                type_annotation.format(formatter)?,
            ],
            semicolon_token,
        )
    }
}
