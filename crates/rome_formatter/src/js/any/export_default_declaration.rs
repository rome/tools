//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyExportDefaultDeclaration;
impl ToFormatElement for JsAnyExportDefaultDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsClassExportDefaultDeclaration(node) => node.format(formatter),
            Self::JsFunctionExportDefaultDeclaration(node) => node.format(formatter),
            Self::TsDeclareFunctionDeclaration(node) => node.format(formatter),
            Self::TsInterfaceDeclaration(node) => node.format(formatter),
        }
    }
}
