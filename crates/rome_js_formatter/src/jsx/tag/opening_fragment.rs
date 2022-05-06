use crate::formatter::verbatim_node;
use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsxOpeningFragment;
use rome_rowan::AstNode;

impl FormatNode for JsxOpeningFragment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        verbatim_node(self.syntax()).format(formatter)
    }
}
