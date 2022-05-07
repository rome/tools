use crate::prelude::*;
use crate::utils::format_with_semicolon;
use rome_js_syntax::TsConstructorSignatureClassMember;
use rome_js_syntax::TsConstructorSignatureClassMemberFields;

impl FormatNode for TsConstructorSignatureClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsConstructorSignatureClassMemberFields {
            modifiers,
            name,
            parameters,
            semicolon_token,
        } = self.as_fields();

        Ok(hard_group_elements(format_with_semicolon(
            formatter,
            formatted![
                formatter,
                modifiers.format(formatter)?,
                space_token(),
                name.format(formatter)?,
                parameters.format(formatter)?,
            ]?,
            semicolon_token,
        )?))
    }
}
