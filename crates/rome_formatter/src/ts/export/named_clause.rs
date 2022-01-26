use crate::{
    empty_element, format_elements, group_elements, if_group_fits_on_single_line, join_elements,
    soft_indent, soft_line_break_or_space, space_token, token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rslint_parser::ast::JsExportNamedClause;
use rslint_parser::AstSeparatedList;

impl ToFormatElement for JsExportNamedClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let specifiers = self.specifiers();
        let space = if specifiers.len() == 0 {
            empty_element()
        } else {
            if_group_fits_on_single_line(space_token())
        };
        let list = group_elements(formatter.format_delimited(
            &self.l_curly_token()?,
            |leading, trailing| {
                Ok(format_elements!(
                    space.clone(),
                    soft_indent(format_elements![
                        leading,
                        join_elements(
                            soft_line_break_or_space(),
                            formatter.format_separated(specifiers, || token(","))?
                        ),
                        trailing,
                    ]),
                    space,
                ))
            },
            &self.r_curly_token()?,
        )?);

        let semicolon = if let Some(semicolon) = self.semicolon_token() {
            formatter.format_token(&semicolon)?
        } else {
            token(";")
        };

        Ok(format_elements![list, semicolon])
    }
}
