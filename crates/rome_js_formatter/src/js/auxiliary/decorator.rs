use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{JsDecorator, JsDecoratorFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDecorator;
impl FormatNodeRule<JsDecorator> for FormatJsDecorator {
    fn fmt_fields(&self, node: &JsDecorator, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDecoratorFields {
            at_token,
            expression,
        } = node.as_fields();

        write![f, [at_token.format(), expression.format()]]
    }
}
