use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsSymbolType;

impl ToFormatElement for TsSymbolType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.symbol_token().format(formatter)
    }
}
