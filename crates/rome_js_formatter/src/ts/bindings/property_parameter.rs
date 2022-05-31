use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsPropertyParameter, TsPropertyParameterFields};

impl FormatNodeFields<TsPropertyParameter> for FormatNodeRule<TsPropertyParameter> {
    fn format_fields(
        node: &TsPropertyParameter,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        let TsPropertyParameterFields {
            modifiers,
            formal_parameter,
        } = node.as_fields();

        formatted![
            formatter,
            [modifiers.format(), space_token(), formal_parameter.format()]
        ]
    }
}
