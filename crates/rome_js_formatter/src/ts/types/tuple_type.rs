use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsTupleType;

impl FormatNode for TsTupleType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited_soft_block_indent(
            &self.l_brack_token()?,
            self.elements().format(formatter)?,
            &self.r_brack_token()?,
        )
    }
}
