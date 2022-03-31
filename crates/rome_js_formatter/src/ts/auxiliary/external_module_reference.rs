use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsExternalModuleReference;
use rome_js_syntax::TsExternalModuleReferenceFields;

impl ToFormatElement for TsExternalModuleReference {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
