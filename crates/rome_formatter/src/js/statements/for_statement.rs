use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, soft_block_indent, soft_line_break_or_space, space_token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::{JsAnyForInitializer, JsForStatement};

impl ToFormatElement for JsForStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let inner =
            if self.initializer().is_some() || self.test().is_some() || self.update().is_some() {
                format_elements![
                    self.initializer().format_or_empty(formatter)?,
                    self.first_semi_token().format(formatter)?,
                    soft_line_break_or_space(),
                    self.test().format_or_empty(formatter)?,
                    self.second_semi_token().format(formatter)?,
                    soft_line_break_or_space(),
                    self.update().format_or_empty(formatter)?,
                ]
            } else {
                format_elements![
                    self.first_semi_token().format(formatter)?,
                    self.second_semi_token().format(formatter)?,
                ]
            };

        Ok(group_elements(format_elements![
            self.for_token().format(formatter)?,
            space_token(),
            formatter.format_delimited(
                &self.l_paren_token()?,
                |open_token_trailing, close_token_leading| Ok(group_elements(soft_block_indent(
                    format_elements![open_token_trailing, inner, close_token_leading]
                ))),
                &self.r_paren_token()?,
            )?,
            space_token(),
            self.body().format(formatter)?
        ]))
    }
}
