//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::TsAnyPropertyAnnotation;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyPropertyAnnotation;
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
