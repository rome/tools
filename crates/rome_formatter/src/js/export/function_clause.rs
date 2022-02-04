use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{JsAnyFunction, JsExportFunctionClause};

impl ToFormatElement for JsExportFunctionClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).to_format_element(formatter)
    }
}
