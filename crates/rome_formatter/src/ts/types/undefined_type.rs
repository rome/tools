use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsUndefinedType;

impl ToFormatElement for TsUndefinedType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.undefined_token().format(formatter)
    }
}
