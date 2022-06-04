use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsImportTypeQualifier;
use rome_js_syntax::TsImportTypeQualifierFields;

impl FormatNodeFields<TsImportTypeQualifier> for FormatNodeRule<TsImportTypeQualifier> {
    fn fmt_fields(node: &TsImportTypeQualifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImportTypeQualifierFields { dot_token, right } = node.as_fields();

        write![f, [dot_token.format(), right.format()]]
    }
}
