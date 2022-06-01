use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTypeParameter, TsTypeParameterFields};

impl FormatNodeFields<TsTypeParameter> for FormatNodeRule<TsTypeParameter> {
    fn format_fields(
        node: &TsTypeParameter,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsTypeParameterFields {
            name,
            constraint,
            default,
        } = node.as_fields();

        formatted![
            formatter,
            [
                name.format(),
                constraint
                    .format()
                    .with_or_empty(|constraint| formatted![formatter, [space_token(), constraint]]),
                default
                    .format()
                    .with_or_empty(|default| formatted![formatter, [space_token(), default]])
            ]
        ]
    }
}
