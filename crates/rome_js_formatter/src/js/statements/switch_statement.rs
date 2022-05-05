use crate::hard_group_elements;
use crate::{
    format_elements, join_elements_hard_line, space_token, Format, FormatElement, FormatNode,
    Formatter, JsFormatter,
};
use rome_formatter::{empty_line, FormatResult};
use rome_js_syntax::{JsSwitchStatement, JsSwitchStatementFields};
use rome_rowan::AstNode;

impl FormatNode for JsSwitchStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSwitchStatementFields {
            switch_token,
            l_paren_token,
            discriminant,
            r_paren_token,
            l_curly_token,
            cases,
            r_curly_token,
        } = self.as_fields();

        let is_cases_empty = cases.clone().into_iter().len() == 0;
        Ok(hard_group_elements(format_elements![
            switch_token.format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                discriminant.format(formatter)?,
                &r_paren_token?,
            )?,
            space_token(),
            formatter.format_delimited_block_indent(
                &l_curly_token?,
                if is_cases_empty {
                    empty_line()
                } else {
                    join_elements_hard_line(
                        cases
                            .clone()
                            .into_iter()
                            .map(|node| node.syntax().clone())
                            .zip(formatter.format_all(cases)?),
                    )
                },
                &r_curly_token?
            )?
        ]))
    }
}
