//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyPropertyParameterModifier;
use crate::prelude::*;
use rome_js_syntax::TsAnyPropertyParameterModifier;
impl FormatRule<TsAnyPropertyParameterModifier> for FormatTsAnyPropertyParameterModifier {
    fn format(
        node: &TsAnyPropertyParameterModifier,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyPropertyParameterModifier::TsAccessibilityModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyPropertyParameterModifier::TsReadonlyModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyPropertyParameterModifier::TsOverrideModifier(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
