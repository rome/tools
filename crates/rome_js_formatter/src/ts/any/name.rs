//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyName;
use crate::prelude::*;
use rome_js_syntax::TsAnyName;
impl FormatRule<TsAnyName> for FormatTsAnyName {
    type Options = JsFormatOptions;
    fn format(
        node: &TsAnyName,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyName::JsReferenceIdentifier(node) => formatted![formatter, [node.format()]],
            TsAnyName::TsQualifiedName(node) => formatted![formatter, [node.format()]],
        }
    }
}
