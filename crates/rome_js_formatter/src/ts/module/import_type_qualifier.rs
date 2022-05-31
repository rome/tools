use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsImportTypeQualifier;
use rome_js_syntax::TsImportTypeQualifierFields;

impl FormatNodeFields<TsImportTypeQualifier> for FormatNodeRule<TsImportTypeQualifier> {
    fn format_fields(
        node: &TsImportTypeQualifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsImportTypeQualifierFields { dot_token, right } = node.as_fields();

        formatted![formatter, [dot_token.format(), right.format(),]]
    }
}
