use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsPropertyParameter, TsPropertyParameterFields};

impl FormatNodeFields<TsPropertyParameter> for FormatNodeRule<TsPropertyParameter> {
    fn fmt_fields(node: &TsPropertyParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let TsPropertyParameterFields {
            modifiers,
            formal_parameter,
        } = node.as_fields();

        write![
            f,
            [modifiers.format(), space_token(), formal_parameter.format()]
        ]
    }
}
