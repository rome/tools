use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsExternalModuleDeclaration;
use rome_js_syntax::TsExternalModuleDeclarationFields;

impl ToFormatElement for TsExternalModuleDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExternalModuleDeclarationFields {
            body,
            module_token,
            source,
        } = self.as_fields();

        let module_token = module_token.format(formatter)?;
        let source = source.format(formatter)?;
        let body = body.format_or_empty(formatter)?;

        Ok(format_elements![
            module_token,
            space_token(),
            source,
            space_token(),
            body
        ])
    }
}
