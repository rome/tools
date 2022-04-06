use crate::formatter_traits::FormatTokenAndNode;

use crate::FormatResult;

use crate::{
    format_elements, hard_line_break, indent, space_token, FormatElement, Formatter,
    ToFormatElement,
};

use rome_js_syntax::JsCaseClause;
use rome_js_syntax::JsCaseClauseFields;

impl ToFormatElement for JsCaseClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCaseClauseFields {
            case_token,
            test,
            colon_token,
            consequent,
        } = self.as_fields();

        let case_word = case_token.format(formatter)?;
        let colon = colon_token.format(formatter)?;
        let test = test.format(formatter)?;
        let cons = formatter.format_list(consequent);

        let cons = if cons.is_empty() {
            // Skip inserting an indent block is the consequent is empty to print
            // the trailing comments for the case clause inline if there is no
            // block to push them into
            hard_line_break()
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            indent(format_elements![hard_line_break(), cons])
        };

        Ok(format_elements![
            case_word,
            space_token(),
            test,
            colon,
            cons
        ])
    }
}
