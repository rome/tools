//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::TsAnyTypePredicateParameterName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyTypePredicateParameterName;
impl FormatRule<TsAnyTypePredicateParameterName> for FormatTsAnyTypePredicateParameterName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyTypePredicateParameterName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyTypePredicateParameterName::JsReferenceIdentifier(node) => node.format().fmt(f),
            TsAnyTypePredicateParameterName::TsThisType(node) => node.format().fmt(f),
        }
    }
}
