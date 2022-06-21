//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyPropertyAnnotation;
#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyPropertyAnnotation;
impl FormatRule<TsAnyPropertyAnnotation> for FormatTsAnyPropertyAnnotation {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyPropertyAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyPropertyAnnotation::TsTypeAnnotation(node) => node.format().fmt(f),
            TsAnyPropertyAnnotation::TsOptionalPropertyAnnotation(node) => node.format().fmt(f),
            TsAnyPropertyAnnotation::TsDefinitePropertyAnnotation(node) => node.format().fmt(f),
        }
    }
}
