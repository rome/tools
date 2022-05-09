//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTypePredicateParameterName;
use crate::prelude::*;
use rome_js_syntax::TsAnyTypePredicateParameterName;
impl FormatRule<TsAnyTypePredicateParameterName> for FormatTsAnyTypePredicateParameterName {
    fn format(
        node: &TsAnyTypePredicateParameterName,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyTypePredicateParameterName::JsReferenceIdentifier(node) => {
                formatted![formatter, node.format()]
            }
            TsAnyTypePredicateParameterName::TsThisType(node) => {
                formatted![formatter, node.format()]
            }
        }
    }
}
