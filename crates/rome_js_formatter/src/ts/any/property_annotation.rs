//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyPropertyAnnotation;
use crate::prelude::*;
use rome_js_syntax::TsAnyPropertyAnnotation;
impl FormatRule<TsAnyPropertyAnnotation> for FormatTsAnyPropertyAnnotation {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyPropertyAnnotation,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        match node {
            TsAnyPropertyAnnotation::TsTypeAnnotation(node) => node.format().format(f),
            TsAnyPropertyAnnotation::TsOptionalPropertyAnnotation(node) => node.format().format(f),
            TsAnyPropertyAnnotation::TsDefinitePropertyAnnotation(node) => node.format().format(f),
        }
    }
}
