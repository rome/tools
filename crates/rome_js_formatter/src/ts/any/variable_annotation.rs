//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyVariableAnnotation;
impl Format for TsAnyVariableAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsTypeAnnotation(node) => node.format(formatter),
            Self::TsDefiniteVariableAnnotation(node) => node.format(formatter),
        }
    }
}
