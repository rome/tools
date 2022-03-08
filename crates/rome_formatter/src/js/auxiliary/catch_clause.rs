use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsCatchClause;
use rome_js_syntax::JsCatchClauseFields;

impl ToFormatElement for JsCatchClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCatchClauseFields {
            catch_token,
            declaration,
            body,
        } = self.as_fields();

        declaration.format_with_or(
            formatter,
            |declaration| {
                Ok(format_elements![
                    catch_token.format(formatter)?,
                    space_token(),
                    declaration,
                    space_token(),
                    body.format(formatter)?
                ])
            },
            || {
                Ok(format_elements![
                    catch_token.format(formatter)?,
                    space_token(),
                    body.format(formatter)?
                ])
            },
        )
    }
}
