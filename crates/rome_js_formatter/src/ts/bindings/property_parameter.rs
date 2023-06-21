use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsPropertyParameter, TsPropertyParameterFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertyParameter;

impl FormatNodeRule<TsPropertyParameter> for FormatTsPropertyParameter {
    fn fmt_fields(&self, node: &TsPropertyParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let TsPropertyParameterFields {
            decorators,
            modifiers,
            formal_parameter,
        } = node.as_fields();

        write![
            f,
            [
                decorators.format(),
                modifiers.format(),
                space(),
                formal_parameter.format()
            ]
        ]
    }
}
