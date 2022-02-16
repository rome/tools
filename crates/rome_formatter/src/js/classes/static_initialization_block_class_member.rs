use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsStaticInitializationBlockClassMember;

impl ToFormatElement for JsStaticInitializationBlockClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let static_token = self.static_token().format(formatter)?;
        let separated = formatter.format_delimited_block_indent(
            &self.l_curly_token()?,
            formatter.format_list(self.statements()),
            &self.r_curly_token()?,
        )?;
        Ok(format_elements![static_token, space_token(), separated])
    }
}
