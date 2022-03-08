use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsFinallyClause;
use rome_js_syntax::JsFinallyClauseFields;

impl ToFormatElement for JsFinallyClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsFinallyClauseFields {
            finally_token,
            body,
        } = self.as_fields();

        Ok(format_elements![
            finally_token.format(formatter)?,
            space_token(),
            body.format(formatter)?
        ])
    }
}
