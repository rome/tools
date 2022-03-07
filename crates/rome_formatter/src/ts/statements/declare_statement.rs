use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsDeclareStatement;
use rome_js_syntax::TsDeclareStatementFields;

impl ToFormatElement for TsDeclareStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDeclareStatementFields {
            declaration,
            declare_token,
        } = self.as_fields();
        Ok(format_elements![
            declare_token.format(formatter)?,
            space_token(),
            declaration.format(formatter)?
        ])
    }
}
