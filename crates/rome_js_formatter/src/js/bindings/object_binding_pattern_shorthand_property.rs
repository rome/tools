use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsObjectBindingPatternShorthandProperty;
use rome_js_syntax::JsObjectBindingPatternShorthandPropertyFields;

impl FormatNodeFields<JsObjectBindingPatternShorthandProperty>
    for FormatNodeRule<JsObjectBindingPatternShorthandProperty>
{
    fn fmt_fields(
        node: &JsObjectBindingPatternShorthandProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectBindingPatternShorthandPropertyFields { identifier, init } = node.as_fields();

        write![f, [identifier.format()]]?;

        if let Some(init) = init {
            write!(f, [space_token(), init.format()])?;
        }

        Ok(())
    }
}
