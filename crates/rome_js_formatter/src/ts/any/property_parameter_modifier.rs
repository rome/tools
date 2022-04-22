//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::TsAnyPropertyParameterModifier;
impl Format for TsAnyPropertyParameterModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsAccessibilityModifier(node) => node.format(formatter),
            Self::TsReadonlyModifier(node) => node.format(formatter),
            Self::TsOverrideModifier(node) => node.format(formatter),
        }
    }
}
