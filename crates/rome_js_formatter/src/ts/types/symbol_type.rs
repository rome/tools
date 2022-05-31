use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsSymbolType, TsSymbolTypeFields};

impl FormatNodeFields<TsSymbolType> for FormatNodeRule<TsSymbolType> {
    fn format_fields(node: &TsSymbolType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsSymbolTypeFields { symbol_token } = node.as_fields();

        formatted![formatter, [symbol_token.format()]]
    }
}
