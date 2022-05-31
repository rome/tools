//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyPropertyParameterModifier;
use crate::prelude::*;
use rome_js_syntax::TsAnyPropertyParameterModifier;
impl FormatRule<TsAnyPropertyParameterModifier> for FormatTsAnyPropertyParameterModifier {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyPropertyParameterModifier,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        match node {
            TsAnyPropertyParameterModifier::TsAccessibilityModifier(node) => {
                node.format().format(f)
            }
            TsAnyPropertyParameterModifier::TsReadonlyModifier(node) => node.format().format(f),
            TsAnyPropertyParameterModifier::TsOverrideModifier(node) => node.format().format(f),
        }
    }
}
