use crate::{Format, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{map_syntax_node, JsSyntaxNode};

impl Format for JsSyntaxNode {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        map_syntax_node!(self.clone(), node => node.format(formatter))
    }
}
