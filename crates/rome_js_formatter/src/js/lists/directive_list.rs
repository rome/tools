use crate::{empty_element, format_elements, hard_line_break, Format, FormatElement, Formatter};
use rome_formatter::{empty_line, FormatResult};
use rome_js_syntax::JsDirectiveList;
use rome_rowan::{AstNode, AstNodeList};

impl Format for JsDirectiveList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        if !self.is_empty() {
            let syntax_node = self.syntax();
            let next_sibling = syntax_node.next_sibling();
            let need_extra_empty_line = match next_sibling {
                Some(node_or_token) => {
                    match node_or_token.first_leading_trivia() {
                        // if next_sibling's first leading_trivia has more than one new_line, we should add an extra empty line at the end of
                        // JsDirectiveList, for example:
                        //```js
                        // "use strict"; <- first leading new_line
                        //  			 <- second leading new_line
                        // function foo() {

                        // }
                        //```
                        // so we should keep an extra empty line after JsDirectiveList
                        Some(trivia) => {
                            trivia.pieces().filter(|piece| piece.is_newline()).count() > 1
                        }
                        None => false,
                    }
                }
                None => false,
            };
            Ok(format_elements![
                formatter.format_list(self.clone()),
                hard_line_break(),
                if need_extra_empty_line {
                    empty_line()
                } else {
                    empty_element()
                }
            ])
        } else {
            Ok(empty_element())
        }
    }
}
