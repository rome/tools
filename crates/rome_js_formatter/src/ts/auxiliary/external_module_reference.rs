use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsExternalModuleReference;
use rome_js_syntax::TsExternalModuleReferenceFields;

impl FormatNode for TsExternalModuleReference {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExternalModuleReferenceFields {
            require_token,
            l_paren_token,
            source,
            r_paren_token,
        } = self.as_fields();

        Ok(format_elements![
            require_token.format(formatter)?,
            l_paren_token.format(formatter)?,
            source.format(formatter)?,
            r_paren_token.format(formatter)?,
        ])
    }
}
