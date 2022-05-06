use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsImportTypeQualifier;
use rome_js_syntax::TsImportTypeQualifierFields;

impl FormatNode for TsImportTypeQualifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsImportTypeQualifierFields { dot_token, right } = self.as_fields();

        formatted![
            formatter,
            dot_token.format(formatter)?,
            right.format(formatter)?,
        ]
    }
}
