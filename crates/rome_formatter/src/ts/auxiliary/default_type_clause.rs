use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsDefaultTypeClause;

impl ToFormatElement for TsDefaultTypeClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let equals = self.eq_token().format(formatter)?;
        let ty = self.ty().format(formatter)?;
        Ok(format_elements![equals, space_token(), ty])
    }
}
