use crate::ts::statements::format_statements;
use crate::{
    block_indent, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsStaticInitializationBlockClassMember;

impl ToFormatElement for JsStaticInitializationBlockClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let static_token = formatter.format_token(&self.static_token()?)?;
        let separated = formatter.format_delimited(
            &self.l_curly_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(block_indent(format_elements![
                    open_token_trailing,
                    format_statements(self.statements(), formatter),
                    close_token_leading,
                ]))
            },
            &self.r_curly_token()?,
        )?;
        Ok(format_elements![static_token, space_token(), separated])
    }
}
