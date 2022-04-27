use crate::{empty_element, format_elements, hard_line_break, Format, FormatElement, Formatter};
use rome_formatter::{empty_line, FormatResult};
use rome_js_syntax::JsDirectiveList;
use rome_rowan::{AstNode, AstNodeList};

impl Format for JsDirectiveList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        if !self.is_empty() {
            let syntax_node = self.syntax();
            // let child = match syntax_node.last_child() {
            //     Some(node) => {
            //     }
            //     None => false,
            // };
            let token = syntax_node.next_sibling();
            let child = match token {
                Some(node_or_token) => {
                    match node_or_token.first_leading_trivia() {
                        Some(trivia) => trivia.pieces().filter(|piece| piece.is_newline()).count() > 1,
                        None => false,
                    }
                }
                None => false,
            };
            // child.
            Ok(format_elements![
                formatter.format_list(self.clone()),
                hard_line_break(),
                if child { empty_line() } else { empty_element() }
            ])
        } else {
            Ok(empty_element())
        }
    }
}
