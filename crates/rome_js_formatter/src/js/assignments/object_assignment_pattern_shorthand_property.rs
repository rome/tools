use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsObjectAssignmentPatternShorthandProperty;
use rome_js_syntax::JsObjectAssignmentPatternShorthandPropertyFields;

impl FormatNodeFields<JsObjectAssignmentPatternShorthandProperty>
    for FormatNodeRule<JsObjectAssignmentPatternShorthandProperty>
{
    fn format_fields(
        node: &JsObjectAssignmentPatternShorthandProperty,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternShorthandPropertyFields { identifier, init } =
            node.as_fields();

        formatted![
            formatter,
            [
                identifier.format()?,
                init.format()
                    .with_or_empty(|node| formatted![formatter, [space_token(), node]])
            ]
        ]
    }
}
