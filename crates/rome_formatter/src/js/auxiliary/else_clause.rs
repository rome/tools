use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, soft_block_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};

use rslint_parser::ast::{JsElseClause, JsIfStatement};

impl ToFormatElement for JsElseClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.else_token().format(formatter)?,
            space_token(),
            self.alternate().format(formatter)?,
        ])
    }
}
