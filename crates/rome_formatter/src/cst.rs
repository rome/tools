use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{map_syntax_node, SyntaxNode};

impl ToFormatElement for SyntaxNode {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        map_syntax_node!(self.clone(), node => node.to_format_element(formatter))
    }
}
