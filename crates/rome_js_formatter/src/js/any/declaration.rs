//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyDeclaration;
use crate::prelude::*;
use rome_js_syntax::JsAnyDeclaration;
impl FormatRule<JsAnyDeclaration> for FormatJsAnyDeclaration {
    fn format(node: &JsAnyDeclaration, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyDeclaration::JsClassDeclaration(node) => formatted![formatter, node.format()],
            JsAnyDeclaration::JsFunctionDeclaration(node) => formatted![formatter, node.format()],
            JsAnyDeclaration::JsVariableDeclaration(node) => formatted![formatter, node.format()],
            JsAnyDeclaration::TsEnumDeclaration(node) => formatted![formatter, node.format()],
            JsAnyDeclaration::TsTypeAliasDeclaration(node) => formatted![formatter, node.format()],
            JsAnyDeclaration::TsInterfaceDeclaration(node) => formatted![formatter, node.format()],
            JsAnyDeclaration::TsDeclareFunctionDeclaration(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclaration::TsModuleDeclaration(node) => formatted![formatter, node.format()],
            JsAnyDeclaration::TsExternalModuleDeclaration(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyDeclaration::TsGlobalDeclaration(node) => formatted![formatter, node.format()],
            JsAnyDeclaration::TsImportEqualsDeclaration(node) => {
                formatted![formatter, node.format()]
            }
        }
    }
}
