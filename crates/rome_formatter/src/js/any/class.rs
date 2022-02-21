use crate::format_element::hard_group_elements;
use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, join_elements_hard_line, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsAnyClass;

impl ToFormatElement for JsAnyClass {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let id = self
            .id()
            .format_with_or_empty(formatter, |id| format_elements![space_token(), id])?;

        let extends = self
            .extends_clause()
            .format_with_or_empty(formatter, |extends_clause| {
                format_elements![space_token(), extends_clause]
            })?;

        Ok(hard_group_elements(format_elements![
            self.class_token().format(formatter)?,
            id,
            extends,
            space_token(),
            formatter.format_delimited_block_indent(
                &self.l_curly_token()?,
                join_elements_hard_line(
                    self.members()
                        .into_iter()
                        .zip(formatter.format_nodes(self.members())?)
                ),
                &self.r_curly_token()?
            )?
        ]))
    }
}
