use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::AnyJsImportClause;

impl ToFormatElement for AnyJsImportClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            AnyJsImportClause::JsImportBareClause(bare_clause) => {
                bare_clause.to_format_element(formatter)
            }
            AnyJsImportClause::JsImportDefaultClause(e) => e.to_format_element(formatter),
            AnyJsImportClause::JsImportNamedClause(named_clause) => {
                named_clause.to_format_element(formatter)
            }
            AnyJsImportClause::JsImportNamespaceClause(_) => todo!(),
            AnyJsImportClause::JsName(name) => name.to_format_element(formatter),
        }
    }
}
