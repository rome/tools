use crate::utils::format_with_semicolon;
use crate::{
    format_elements, hard_group_elements, space_token, Format, FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsSetterSignatureClassMember, TsSetterSignatureClassMemberFields};

impl FormatNode for TsSetterSignatureClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsSetterSignatureClassMemberFields {
            modifiers,
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            semicolon_token,
        } = self.as_fields();

        let set_token = set_token.format(formatter)?;
        let name = name.format(formatter)?;
        let l_paren_token = l_paren_token.format(formatter)?;
        let parameters = parameter.format(formatter)?;
        let r_paren_token = r_paren_token.format(formatter)?;

        Ok(hard_group_elements(format_with_semicolon(
            formatter,
            format_elements![
                modifiers.format(formatter)?,
                space_token(),
                set_token,
                space_token(),
                name,
                l_paren_token,
                parameters,
                r_paren_token,
            ],
            semicolon_token,
        )?))
    }
}
