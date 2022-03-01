use crate::{
    format_elements,
    formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode},
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{TsPropertySignatureClassMember, TsPropertySignatureClassMemberFields};

impl ToFormatElement for TsPropertySignatureClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsPropertySignatureClassMemberFields {
            modifiers,
            name,
            property_annotation,
            semicolon_token,
        } = self.as_fields();

        let property_annotation = property_annotation.format_or_empty(formatter)?;

        let semicolon_token = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            modifiers.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            property_annotation,
            semicolon_token
        ])
    }
}
