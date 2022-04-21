use crate::{FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::JsxClosingFragment;
use rome_rowan::AstNode;

impl FormatNode for JsxClosingFragment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}
