use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{
    formatted, space_token, Format, FormatElement, FormatNode, Formatter,
};

use rome_js_syntax::JsCatchClause;
use rome_js_syntax::JsCatchClauseFields;

impl FormatNode for JsCatchClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCatchClauseFields {
            catch_token,
            declaration,
            body,
        } = self.as_fields();

        declaration.format_with_or(
            formatter,
            |declaration| {
                formatted![
                    formatter,
                    catch_token.format(formatter)?,
                    space_token(),
                    declaration,
                    space_token(),
                    body.format(formatter)?
                ]
            },
            || {
                formatted![
                    formatter,
                    catch_token.format(formatter)?,
                    space_token(),
                    body.format(formatter)?
                ]
            },
        )
    }
}
