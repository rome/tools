use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsObjectAssignmentPatternShorthandProperty;
use rome_js_syntax::JsObjectAssignmentPatternShorthandPropertyFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectAssignmentPatternShorthandProperty;

impl FormatNodeRule<JsObjectAssignmentPatternShorthandProperty>
    for FormatJsObjectAssignmentPatternShorthandProperty
{
    fn fmt_fields(
        &self,
        node: &JsObjectAssignmentPatternShorthandProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectAssignmentPatternShorthandPropertyFields { identifier, init } =
            node.as_fields();

        write!(f, [identifier.format()?,])?;

        if let Some(init) = init {
            write!(f, [space(), init.format()])?;
        }
        Ok(())
    }
}
