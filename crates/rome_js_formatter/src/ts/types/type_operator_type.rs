use crate::format_traits::FormatWith;
use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsTypeOperatorType;

impl FormatNode for TsTypeOperatorType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.operator_token().format_with(formatter, |operator| {
                format_elements![operator, space_token()]
            })?,
            self.ty().format(formatter)?
        ])
    }
}
