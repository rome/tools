use crate::format_traits::FormatOptional;
use crate::{
    formatted, join_elements_hard_line, space_token, FormatElement, Formatter,
    JsFormatter,
};
use crate::{hard_group_elements, Format};
use rome_formatter::FormatResult;
use rome_js_syntax::JsAnyClass;
use rome_rowan::AstNode;

impl Format for JsAnyClass {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let abstract_token = self
            .abstract_token()
            .format_with_or_empty(formatter, |token| {
                formatted![formatter, token, space_token()]
            })?;

        let id = self
            .id()
            .format_with_or_empty(formatter, |id| formatted![formatter, space_token(), id])?;

        let type_parameters = self.type_parameters().format_or_empty(formatter)?;

        let extends = self
            .extends_clause()
            .format_with_or_empty(formatter, |extends_clause| {
                formatted![formatter, space_token(), extends_clause]
            })?;

        let implements_clause = self
            .implements_clause()
            .format_with_or_empty(formatter, |implements_clause| {
                formatted![formatter, space_token(), implements_clause]
            })?;

        Ok(hard_group_elements(formatted![
            formatter,
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
                        .zip(formatter.format_all(self.members())?)
                ),
                &self.r_curly_token()?
            )?
        ]?))
    }
}
