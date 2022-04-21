//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyExportDefaultDeclaration;
impl Format for JsAnyExportDefaultDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsClassExportDefaultDeclaration(node) => node.format(formatter),
            Self::JsFunctionExportDefaultDeclaration(node) => node.format(formatter),
            Self::TsDeclareFunctionDeclaration(node) => node.format(formatter),
            Self::TsInterfaceDeclaration(node) => node.format(formatter),
        }
    }
}
