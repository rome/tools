use crate::format_traits::FormatWith;
use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsMappedTypeAsClause;

impl FormatNode for TsMappedTypeAsClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.as_token().format_with(formatter, |as_token| {
                format_elements![as_token, space_token()]
            })?,
            self.ty().format(formatter)?
        ])
    }
}
