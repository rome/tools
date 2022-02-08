use crate::formatter_traits::FormatTokenAndNode;

use crate::{block_indent, FormatResult};

use crate::{
    format_element::indent, format_elements, group_elements, hard_line_break,
    join_elements_hard_line, soft_block_indent, space_token, FormatElement, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::{JsAnySwitchClause, JsCaseClause, JsDefaultClause, JsSwitchStatement};

impl ToFormatElement for JsCaseClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let case_word = self.case_token().format(formatter)?;
        let colon = self.colon_token().format(formatter)?;
        let test = self.test().format(formatter)?;
        let cons = formatter.format_list(self.consequent());

        Ok(format_elements![
            case_word,
            space_token(),
            test,
            colon,
            // no line break needed after because it is added by the indent in the switch statement
            indent(format_elements![hard_line_break(), cons])
        ])
    }
}
