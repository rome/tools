use crate::formatter::verbatim_node;
use crate::formatter_traits::FormatTokenAndNode;
use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{JsSyntaxToken, JsxText};
use rome_rowan::{AstNode, SyntaxToken};

impl FormatNode for JsxText {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.value_token().format(formatter)
    }
}
