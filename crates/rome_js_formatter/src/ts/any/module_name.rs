//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyModuleName;
use crate::prelude::*;
use rome_js_syntax::TsAnyModuleName;
impl FormatRule<TsAnyModuleName> for FormatTsAnyModuleName {
    type Options = JsFormatOptions;
    fn format(
        node: &TsAnyModuleName,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyModuleName::TsIdentifierBinding(node) => formatted![formatter, [node.format()]],
            TsAnyModuleName::TsQualifiedModuleName(node) => formatted![formatter, [node.format()]],
        }
    }
}
