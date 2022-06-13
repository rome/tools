use crate::prelude::*;
use crate::utils::FormatMemberName;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsObjectBindingPatternProperty;
use rome_js_syntax::JsObjectBindingPatternPropertyFields;

impl FormatNodeFields<JsObjectBindingPatternProperty>
    for FormatNodeRule<JsObjectBindingPatternProperty>
{
    fn fmt_fields(node: &JsObjectBindingPatternProperty, f: &mut JsFormatter) -> FormatResult<()> {
        let JsObjectBindingPatternPropertyFields {
            member,
            colon_token,
            pattern,
            init,
        } = node.as_fields();

        write![
            f,
            [
                FormatMemberName::from(member?),
                colon_token.format(),
                space_token(),
                pattern.format(),
            ]
        ]?;

        if let Some(init) = init {
            write!(f, [space_token(), init.format()])?;
        }

        Ok(())
    }
}
