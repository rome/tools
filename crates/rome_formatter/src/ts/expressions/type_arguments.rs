use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsTypeArguments;

impl ToFormatElement for TsTypeArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited_soft_block_indent(
            &self.l_angle_token()?,
            self.ts_type_argument_list().format(formatter)?,
            &self.r_angle_token()?,
        )
    }
}
