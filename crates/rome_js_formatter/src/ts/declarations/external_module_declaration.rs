
use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsExternalModuleDeclaration;
use rome_js_syntax::TsExternalModuleDeclarationFields;

impl FormatNode for TsExternalModuleDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExternalModuleDeclarationFields {
            body,
            module_token,
            source,
        } = self.as_fields();

        let module_token = module_token.format(formatter)?;
        let source = source.format(formatter)?;

        formatted![
            formatter,
            module_token,
            space_token(),
            source,
            space_token(),
            body
        ]
    }
}
