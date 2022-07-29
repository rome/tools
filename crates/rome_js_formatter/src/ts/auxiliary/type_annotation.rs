use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsTypeAnnotation, TsTypeAnnotationFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeAnnotation;

impl FormatNodeRule<TsTypeAnnotation> for FormatTsTypeAnnotation {
    fn fmt_fields(&self, node: &TsTypeAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeAnnotationFields { colon_token, ty } = node.as_fields();
        let colon = colon_token.format();
        let ty = ty.format();

        write![f, [colon, space(), ty]]
    }
}
