use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, if_group_breaks, soft_line_break, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rslint_parser::ast::TsExtendsClause;

impl ToFormatElement for TsExtendsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let extends = self.extends_token().format(formatter)?;
        let types = self.types().format(formatter)?;
        Ok(format_elements![extends, space_token(), types])
    }
}
