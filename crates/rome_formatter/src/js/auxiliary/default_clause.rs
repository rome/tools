use crate::formatter_traits::FormatTokenAndNode;

use crate::FormatResult;

use crate::{
    format_element::indent, format_elements, hard_line_break, space_token, FormatElement,
    Formatter, ToFormatElement,
};

use rslint_parser::ast::JsDefaultClause;

impl ToFormatElement for JsDefaultClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let default = self.default_token().format(formatter)?;
        let colon = self.colon_token().format(formatter)?;
        let statements = formatter.format_list(self.consequent());

        Ok(format_elements![
            default,
            colon,
            space_token(),
            // no line break needed after because it is added by the indent in the switch statement
            indent(format_elements![hard_line_break(), statements])
        ])
    }
}
