//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyModuleReference;
use crate::prelude::*;
use rome_js_syntax::TsAnyModuleReference;
impl FormatRule<TsAnyModuleReference> for FormatTsAnyModuleReference {
    fn format(node: &TsAnyModuleReference, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            TsAnyModuleReference::TsAnyName(node) => formatted![formatter, node.format()],
            TsAnyModuleReference::TsExternalModuleReference(node) => {
                formatted![formatter, node.format()]
            }
        }
    }
}
