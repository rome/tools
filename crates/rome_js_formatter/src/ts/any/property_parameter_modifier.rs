//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::TsAnyPropertyParameterModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyPropertyParameterModifier;
impl FormatRule<TsAnyPropertyParameterModifier> for FormatTsAnyPropertyParameterModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyPropertyParameterModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyPropertyParameterModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            TsAnyPropertyParameterModifier::TsReadonlyModifier(node) => node.format().fmt(f),
            TsAnyPropertyParameterModifier::TsOverrideModifier(node) => node.format().fmt(f),
        }
    }
}
