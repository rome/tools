use crate::prelude::*;
use crate::utils::FormatMemberName;
use crate::FormatNodeFields;
use rome_js_syntax::JsObjectAssignmentPatternProperty;
use rome_js_syntax::JsObjectAssignmentPatternPropertyFields;

impl FormatNodeFields<JsObjectAssignmentPatternProperty>
    for FormatNodeRule<JsObjectAssignmentPatternProperty>
{
    fn format_fields(
        node: &JsObjectAssignmentPatternProperty,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternPropertyFields {
            member,
            colon_token,
            pattern,
            init,
        } = node.as_fields();

        formatted![
            formatter,
            [
                FormatMemberName::from(member?),
                colon_token.format(),
                space_token(),
                pattern.format(),
                init.format()
                    .with_or_empty(|node| formatted![formatter, [space_token(), node]]),
            ]
        ]
    }
}
