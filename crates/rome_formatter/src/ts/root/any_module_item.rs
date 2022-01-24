use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyModuleItem;

impl ToFormatElement for JsAnyModuleItem {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyModuleItem::JsAnyStatement(any_statement) => {
                any_statement.to_format_element(formatter)
            }
            JsAnyModuleItem::JsExport(_) => todo!(),
            JsAnyModuleItem::JsImport(_) => todo!(),
        }
    }
}
