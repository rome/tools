use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsLiteralExportName;
use rslint_syntax::JsLiteralExportNameFields;

impl ToFormatElement for JsLiteralExportName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLiteralExportNameFields { value } = self.as_fields();

        value.format(formatter)
    }
}
