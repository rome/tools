use crate::{FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsUnknownExpression;
use rome_rowan::AstNode;

impl FormatNode for JsUnknownExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_unknown(self.syntax()))
    }
}
