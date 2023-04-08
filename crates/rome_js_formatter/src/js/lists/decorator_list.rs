use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::JsDecoratorList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDecoratorList;
impl FormatRule<JsDecoratorList> for FormatJsDecoratorList {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsDecoratorList, f: &mut JsFormatter) -> FormatResult<()> {
        if node.is_empty() {
            return Ok(());
        }

        f.join_with(&soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()?;

        write!(f, [soft_line_break_or_space()])
    }
}
