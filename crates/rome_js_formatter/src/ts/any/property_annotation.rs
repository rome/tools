//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyPropertyAnnotation;
use crate::prelude::*;
use rome_js_syntax::TsAnyPropertyAnnotation;
impl FormatRule<TsAnyPropertyAnnotation> for FormatTsAnyPropertyAnnotation {
    fn format(
        node: &TsAnyPropertyAnnotation,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyPropertyAnnotation::TsTypeAnnotation(node) => formatted![formatter, node.format()],
            TsAnyPropertyAnnotation::TsOptionalPropertyAnnotation(node) => {
                formatted![formatter, node.format()]
            }
            TsAnyPropertyAnnotation::TsDefinitePropertyAnnotation(node) => {
                formatted![formatter, node.format()]
            }
        }
    }
}
