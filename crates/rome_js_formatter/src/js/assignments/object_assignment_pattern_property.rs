use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatMemberName;
use crate::FormatNodeFields;
use rome_js_syntax::JsObjectAssignmentPatternProperty;
use rome_js_syntax::JsObjectAssignmentPatternPropertyFields;

impl FormatNodeFields<JsObjectAssignmentPatternProperty>
    for FormatNodeRule<JsObjectAssignmentPatternProperty>
{
    fn fmt_fields(
        node: &JsObjectAssignmentPatternProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectAssignmentPatternPropertyFields {
            member,
            colon_token,
            pattern,
            init,
        } = node.as_fields();

        write!(
            f,
            [
                FormatMemberName::from(member?),
                colon_token.format(),
                space_token(),
                pattern.format(),
            ]
        )?;

        if let Some(init) = init {
            write!(f, [space_token(), init.format()])?;
        }

        Ok(())
    }
}
