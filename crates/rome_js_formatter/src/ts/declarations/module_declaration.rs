use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsModuleDeclaration;
use rome_js_syntax::TsModuleDeclarationFields;

impl FormatNode for TsModuleDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsModuleDeclarationFields {
            module_or_namespace,
            name,
            body,
        } = self.as_fields();

        Ok(format_elements![
            module_or_namespace.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            space_token(),
            body.format(formatter)?,
        ])
    }
}
