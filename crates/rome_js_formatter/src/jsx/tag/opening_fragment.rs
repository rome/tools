use crate::{FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::JsxOpeningFragment;
use rome_rowan::AstNode;

impl FormatNode for JsxOpeningFragment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}
