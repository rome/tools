use crate::format_element::join_elements_soft_line;
use crate::{
    empty_element, format_elements, group_elements, if_group_fits_on_single_line,
    soft_block_indent, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
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

        Ok(group_elements(formatter.format_delimited(
            &self.l_curly_token()?,
            |open_token_trailing, close_token_leading| {
                let members = formatter.format_separated(members, || token(","))?;

                Ok(format_elements!(
                    space.clone(),
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        join_elements_soft_line(
                            self.members()
                                .elements()
                                // This unwrap is guarded by the call to format_separated above
                                .map(|node| node.node().unwrap())
                                .zip(members)
                        ),
                        close_token_leading
                    ]),
                    space,
                ))
            },
            &self.r_curly_token()?,
        )?))
    }
}
