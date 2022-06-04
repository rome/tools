//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyName;
use crate::prelude::*;
use rome_js_syntax::TsAnyName;
impl FormatRule<TsAnyName> for FormatTsAnyName {
    type Context = JsFormatContext;
    fn fmt(node: &TsAnyName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyName::JsReferenceIdentifier(node) => node.format().fmt(f),
            TsAnyName::TsQualifiedName(node) => node.format().fmt(f),
        }
    }
}
