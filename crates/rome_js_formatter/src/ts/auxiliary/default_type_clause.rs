use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsDefaultTypeClause;

impl FormatNode for TsDefaultTypeClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let equals = self.eq_token().format(formatter)?;
        let ty = self.ty().format(formatter)?;
        Ok(format_elements![equals, space_token(), ty])
    }
}
