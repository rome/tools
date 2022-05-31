use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsObjectBindingPatternShorthandProperty;
use rome_js_syntax::JsObjectBindingPatternShorthandPropertyFields;

impl FormatNodeFields<JsObjectBindingPatternShorthandProperty>
    for FormatNodeRule<JsObjectBindingPatternShorthandProperty>
{
    fn format_fields(
        node: &JsObjectBindingPatternShorthandProperty,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternShorthandPropertyFields { identifier, init } = node.as_fields();

        formatted![
            formatter,
            [
                identifier.format(),
                init.format()
                    .with_or_empty(|node| formatted![formatter, [space_token(), node]])
            ]
        ]
    }
}
