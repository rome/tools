//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyExportDefaultDeclaration;
impl ToFormatElement for JsAnyExportDefaultDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsClassExportDefaultDeclaration(node) => node.to_format_element(formatter),
            Self::JsFunctionExportDefaultDeclaration(node) => node.to_format_element(formatter),
            Self::TsDeclareFunctionDeclaration(node) => node.to_format_element(formatter),
            Self::TsInterfaceDeclaration(node) => node.to_format_element(formatter),
        }
    }
}
