use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyExportClause;

impl ToFormatElement for JsAnyExportClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyExportClause::JsExportClassClause(node) => node.to_format_element(formatter),
            JsAnyExportClause::JsExportDefaultClassClause(node) => {
                node.to_format_element(formatter)
            }
            JsAnyExportClause::JsExportDefaultExpressionClause(_) => todo!(),
            JsAnyExportClause::JsExportDefaultFunctionClause(_) => todo!(),
            JsAnyExportClause::JsExportFromClause(_) => todo!(),
            JsAnyExportClause::JsExportFunctionClause(_) => todo!(),
            JsAnyExportClause::JsExportNamedClause(_) => todo!(),
            JsAnyExportClause::JsExportNamedFromClause(_) => todo!(),
            JsAnyExportClause::JsExportVariableClause(_) => todo!(),
        }
    }
}
