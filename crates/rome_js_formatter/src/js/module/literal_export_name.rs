use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsLiteralExportName;
use rome_js_syntax::JsLiteralExportNameFields;

impl FormatNodeFields<JsLiteralExportName> for FormatNodeRule<JsLiteralExportName> {
    fn format_fields(node: &JsLiteralExportName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsLiteralExportNameFields { value } = node.as_fields();

        write!(f, [FormatLiteralStringToken::from_string(&value?)])
    }
}
