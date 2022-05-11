//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureAnnotation;
impl Format for TsAnyPropertySignatureAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsTypeAnnotation(node) => node.format(formatter),
            Self::TsOptionalPropertyAnnotation(node) => node.format(formatter),
        }
    }
}
