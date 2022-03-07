use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsTypeConstraintClause;

impl ToFormatElement for TsTypeConstraintClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let extends = self.extends_token().format(formatter)?;
        let ty = self.ty().format(formatter)?;
        Ok(format_elements![extends, space_token(), ty])
    }
}
