use crate::format_traits::FormatWith;
use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTypeOperatorType;

impl FormatNode for TsTypeOperatorType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatted![
            formatter,
            self.operator_token().format_with(formatter, |operator| {
                formatted![formatter, operator, space_token()]
            })?,
            self.ty().format(formatter)?
        ]
    }
}
