use crate::formatter_traits::FormatTokenAndNode;
use crate::FormatResult;
use crate::{
    format_elements, hard_line_break, indent, space_token, FormatElement, Formatter,
    ToFormatElement,
};

use rome_js_syntax::JsDefaultClause;
use rome_js_syntax::JsDefaultClauseFields;

impl ToFormatElement for JsDefaultClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDefaultClauseFields {
            default_token,
            colon_token,
            consequent,
        } = self.as_fields();

        let default = default_token.format(formatter)?;
        let colon = colon_token.format(formatter)?;
        let statements = formatter.format_list(consequent);

        Ok(format_elements![
            default,
            colon,
            space_token(),
            // no line break needed after because it is added by the indent in the switch statement
            indent(format_elements![hard_line_break(), statements])
        ])
    }
}
