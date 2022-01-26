use crate::{
    empty_element, format_elements, group_elements, if_group_fits_on_single_line, join_elements,
    soft_indent, soft_line_break_or_space, space_token, token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rslint_parser::ast::JsExportNamedFromClause;
use rslint_parser::AstSeparatedList;

impl ToFormatElement for JsExportNamedFromClause {
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

        let from = formatter.format_token(&self.from_token()?)?;
        let source = formatter.format_node(self.source()?)?;
        let assertion = if let Some(assertion) = self.assertion() {
            formatter.format_node(assertion)?
        } else {
            empty_element()
        };
        let semicolon = if let Some(semicolon) = self.semicolon_token() {
            formatter.format_token(&semicolon)?
        } else {
            token(";")
        };

        Ok(format_elements![
            list,
            space_token(),
            from,
            space_token(),
            source,
            space_token(),
            assertion,
            semicolon
        ])
    }
}
