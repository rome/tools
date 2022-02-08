use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::{JsAnyClass, JsExportClassClause};

impl ToFormatElement for JsExportClassClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyClass::from(self.clone()).to_format_element(formatter)
    }
}
