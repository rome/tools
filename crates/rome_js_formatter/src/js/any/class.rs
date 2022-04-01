use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::hard_group_elements;
use crate::{
    format_elements, join_elements_hard_line, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::{AstNode, JsAnyClass};

impl ToFormatElement for JsAnyClass {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let abstract_token = self
            .abstract_token()
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let id = self
            .id()
            .format_with_or_empty(formatter, |id| format_elements![space_token(), id])?;

        let type_parameters = self.type_parameters().format_or_empty(formatter)?;

        let extends = self
            .extends_clause()
            .format_with_or_empty(formatter, |extends_clause| {
                format_elements![space_token(), extends_clause]
            })?;

        let implements_clause = self
            .implements_clause()
            .format_with_or_empty(formatter, |implements_clause| {
                format_elements![space_token(), implements_clause]
            })?;

        Ok(hard_group_elements(format_elements![
            abstract_token,
            self.class_token().format(formatter)?,
            id,
            type_parameters,
            extends,
            implements_clause,
            space_token(),
            formatter.format_delimited_block_indent(
                &self.l_curly_token()?,
                join_elements_hard_line(
                    self.members()
                        .into_iter()
                        .map(|node| node.syntax().clone())
                        .zip(formatter.format_nodes(self.members())?)
                ),
                &self.r_curly_token()?
            )?
        ]))
    }
}
