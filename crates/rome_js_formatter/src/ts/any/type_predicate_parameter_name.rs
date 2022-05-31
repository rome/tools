//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTypePredicateParameterName;
use crate::prelude::*;
use rome_js_syntax::TsAnyTypePredicateParameterName;
impl FormatRule<TsAnyTypePredicateParameterName> for FormatTsAnyTypePredicateParameterName {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyTypePredicateParameterName,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        match node {
            TsAnyTypePredicateParameterName::JsReferenceIdentifier(node) => node.format().format(f),
            TsAnyTypePredicateParameterName::TsThisType(node) => node.format().format(f),
        }
    }
}
