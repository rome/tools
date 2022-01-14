use crate::{
    empty_element, format_elements, group_elements, if_group_breaks, if_group_fits_on_single_line,
    join_elements, soft_indent, soft_line_break_or_space, space_token, token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsObjectExpression;
use rslint_parser::AstSeparatedList;

impl ToFormatElement for JsObjectExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let members = self.members();

        let space = if members.len() == 0 {
            empty_element()
        } else {
            if_group_fits_on_single_line(space_token())
        };

        Ok(group_elements(format_elements!(
            formatter.format_token(&self.l_curly_token()?)?,
            space.clone(),
            soft_indent(format_elements![
                join_elements(
                    soft_line_break_or_space(),
                    formatter.format_separated(members)?
                ),
                if_group_breaks(token(",")),
            ]),
            space,
            formatter.format_token(&self.r_curly_token()?)?,
        )))
    }
}
