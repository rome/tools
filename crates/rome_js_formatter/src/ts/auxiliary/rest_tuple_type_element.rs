use crate::prelude::*;
use rome_js_syntax::{TsRestTupleTypeElement, TsRestTupleTypeElementFields};

impl FormatNode for TsRestTupleTypeElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsRestTupleTypeElementFields {
            dotdotdot_token,
            ty,
        } = self.as_fields();
        let dotdotdot = dotdotdot_token.format(formatter)?;
        let ty = ty.format(formatter)?;
        formatted![formatter, dotdotdot, ty]
    }
}
