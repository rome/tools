use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsMappedTypeAsClause;

impl ToFormatElement for TsMappedTypeAsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.as_token().format_with(formatter, |as_token| {
                format_elements![as_token, space_token()]
            })?,
            self.ty().format(formatter)?
        ])
    }
}
