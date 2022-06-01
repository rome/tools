use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsObjectBindingPatternProperty;
use rome_js_syntax::JsObjectBindingPatternPropertyFields;

impl FormatNodeFields<JsObjectBindingPatternProperty>
    for FormatNodeRule<JsObjectBindingPatternProperty>
{
    fn format_fields(
        node: &JsObjectBindingPatternProperty,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternPropertyFields {
            member,
            colon_token,
            pattern,
            init,
        } = node.as_fields();

        formatted![
            formatter,
            [
                member.format(),
                colon_token.format(),
                space_token(),
                pattern.format(),
                init.format()
                    .with_or_empty(|node| formatted![formatter, [space_token(), node]]),
            ]
        ]
    }
}
