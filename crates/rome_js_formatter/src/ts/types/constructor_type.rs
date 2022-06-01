use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsConstructorType;
use rome_js_syntax::TsConstructorTypeFields;

impl FormatNodeFields<TsConstructorType> for FormatNodeRule<TsConstructorType> {
    fn format_fields(
        node: &TsConstructorType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsConstructorTypeFields {
            abstract_token,
            new_token,
            type_parameters,
            parameters,
            fat_arrow_token,
            return_type,
        } = node.as_fields();

        formatted![
            formatter,
            [
                abstract_token
                    .format()
                    .with_or_empty(|element| formatted![formatter, [element, space_token()]]),
                new_token.format(),
                type_parameters.format(),
                parameters.format(),
                space_token(),
                fat_arrow_token.format(),
                space_token(),
                return_type.format()
            ]
        ]
    }
}
