use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTypeOperatorType, TsTypeOperatorTypeFields};

impl FormatNodeFields<TsTypeOperatorType> for FormatNodeRule<TsTypeOperatorType> {
    fn format_fields(
        node: &TsTypeOperatorType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsTypeOperatorTypeFields { operator_token, ty } = node.as_fields();

        formatted![
            formatter,
            [
                operator_token
                    .format()
                    .with(|operator| { formatted![formatter, [operator, space_token()]] }),
                ty.format()
            ]
        ]
    }
}
