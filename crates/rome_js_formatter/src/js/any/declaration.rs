//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyDeclaration;
impl Format for JsAnyDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsClassDeclaration(node) => node.format(formatter),
            Self::JsFunctionDeclaration(node) => node.format(formatter),
            Self::JsVariableDeclaration(node) => node.format(formatter),
            Self::TsEnumDeclaration(node) => node.format(formatter),
            Self::TsTypeAliasDeclaration(node) => node.format(formatter),
            Self::TsInterfaceDeclaration(node) => node.format(formatter),
            Self::TsDeclareFunctionDeclaration(node) => node.format(formatter),
            Self::TsModuleDeclaration(node) => node.format(formatter),
            Self::TsExternalModuleDeclaration(node) => node.format(formatter),
            Self::TsGlobalDeclaration(node) => node.format(formatter),
            Self::TsImportEqualsDeclaration(node) => node.format(formatter),
        }
    }
}
