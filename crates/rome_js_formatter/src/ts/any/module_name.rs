//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyModuleName;
use crate::prelude::*;
use rome_js_syntax::TsAnyModuleName;
impl FormatRule<TsAnyModuleName> for FormatTsAnyModuleName {
    fn format(node: &TsAnyModuleName, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            TsAnyModuleName::TsIdentifierBinding(node) => formatted![formatter, [node.format()]],
            TsAnyModuleName::TsQualifiedModuleName(node) => formatted![formatter, [node.format()]],
        }
    }
}
