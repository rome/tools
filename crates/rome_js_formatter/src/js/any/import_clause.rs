//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyImportClause;
impl Format for JsAnyImportClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsImportBareClause(node) => node.format(formatter),
            Self::JsImportNamedClause(node) => node.format(formatter),
            Self::JsImportDefaultClause(node) => node.format(formatter),
            Self::JsImportNamespaceClause(node) => node.format(formatter),
        }
    }
}
