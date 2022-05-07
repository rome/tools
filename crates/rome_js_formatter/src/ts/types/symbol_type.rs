use crate::prelude::*;
use rome_js_syntax::TsSymbolType;

impl FormatNode for TsSymbolType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.symbol_token().format(formatter)
    }
}
