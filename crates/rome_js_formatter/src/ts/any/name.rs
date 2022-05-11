//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyName;
use crate::prelude::*;
use rome_js_syntax::TsAnyName;
impl FormatRule<TsAnyName> for FormatTsAnyName {
    fn format(node: &TsAnyName, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            TsAnyName::JsReferenceIdentifier(node) => formatted![formatter, [node.format()]],
            TsAnyName::TsQualifiedName(node) => formatted![formatter, [node.format()]],
        }
    }
}
