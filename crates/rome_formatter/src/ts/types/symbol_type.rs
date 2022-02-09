use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsSymbolType;

impl ToFormatElement for TsSymbolType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.symbol_token().format(formatter)
    }
}
