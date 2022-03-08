use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsImportTypeQualifier;
use rome_js_syntax::TsImportTypeQualifierFields;

impl ToFormatElement for TsImportTypeQualifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsImportTypeQualifierFields { dot_token, right } = self.as_fields();

        Ok(format_elements![
            dot_token.format(formatter)?,
            right.format(formatter)?,
        ])
    }
}
