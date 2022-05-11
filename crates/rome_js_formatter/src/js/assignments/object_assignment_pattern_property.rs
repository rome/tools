use crate::prelude::*;

use rome_js_syntax::JsObjectAssignmentPatternProperty;
use rome_js_syntax::JsObjectAssignmentPatternPropertyFields;

impl FormatNode for JsObjectAssignmentPatternProperty {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternPropertyFields {
            member,
            colon_token,
            pattern,
            init,
        } = self.as_fields();

        let init_node = init.with_or_empty(|node| formatted![formatter, space_token(), node]);
        formatted![
            formatter,
            member.format(formatter)?,
            colon_token.format(formatter)?,
            space_token(),
            pattern.format(formatter)?,
            init_node,
        ]
    }
}
