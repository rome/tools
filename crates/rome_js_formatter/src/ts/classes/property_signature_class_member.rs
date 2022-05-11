use crate::prelude::*;
use crate::utils::format_with_semicolon;
use rome_js_syntax::{TsPropertySignatureClassMember, TsPropertySignatureClassMemberFields};

impl FormatNode for TsPropertySignatureClassMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsPropertySignatureClassMemberFields {
            modifiers,
            name,
            property_annotation,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                modifiers.format(formatter)?,
                space_token(),
                name.format(formatter)?,
                property_annotation,
            ]?,
            semicolon_token,
        )
    }
}
