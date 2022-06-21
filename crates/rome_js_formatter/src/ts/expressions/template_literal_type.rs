use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsTemplateLiteralType;
use rome_js_syntax::TsTemplateLiteralTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTemplateLiteralType;

impl FormatNodeRule<TsTemplateLiteralType> for FormatTsTemplateLiteralType {
    fn fmt_fields(node: &TsTemplateLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTemplateLiteralTypeFields {
            l_tick_token,
            elements,
            r_tick_token,
        } = node.as_fields();

        write![
            f,
            [
                l_tick_token.format(),
                elements.format(),
                r_tick_token.format(),
            ]
        ]
    }
}
