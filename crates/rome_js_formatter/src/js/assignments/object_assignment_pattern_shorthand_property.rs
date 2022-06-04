use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsObjectAssignmentPatternShorthandProperty;
use rome_js_syntax::JsObjectAssignmentPatternShorthandPropertyFields;

impl FormatNodeFields<JsObjectAssignmentPatternShorthandProperty>
    for FormatNodeRule<JsObjectAssignmentPatternShorthandProperty>
{
    fn fmt_fields(
        node: &JsObjectAssignmentPatternShorthandProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectAssignmentPatternShorthandPropertyFields { identifier, init } =
            node.as_fields();

        write!(f, [identifier.format()?,])?;

        if let Some(init) = init {
            write!(f, [space_token(), init.format()])?;
        }
        Ok(())
    }
}
