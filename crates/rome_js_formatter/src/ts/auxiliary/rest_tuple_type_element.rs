use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsRestTupleTypeElement, TsRestTupleTypeElementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsRestTupleTypeElement;

impl FormatNodeRule<TsRestTupleTypeElement> for FormatTsRestTupleTypeElement {
    fn fmt_fields(node: &TsRestTupleTypeElement, f: &mut JsFormatter) -> FormatResult<()> {
        let TsRestTupleTypeElementFields {
            dotdotdot_token,
            ty,
        } = node.as_fields();
        let dotdotdot = dotdotdot_token.format();
        let ty = ty.format();
        write![f, [dotdotdot, ty]]
    }
}
