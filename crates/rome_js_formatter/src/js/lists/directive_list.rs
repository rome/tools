use crate::{
    empty_element, formatted, hard_line_break, Format, FormatElement, Formatter,
    JsFormatter,
};
use rome_formatter::{empty_line, format_element::get_lines_before, FormatResult};
use rome_js_syntax::JsDirectiveList;
use rome_rowan::{AstNode, AstNodeList};

impl Format for JsDirectiveList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        if !self.is_empty() {
            let syntax_node = self.syntax();
            let next_sibling = syntax_node.next_sibling();
            // if next_sibling's first leading_trivia has more than one new_line, we should add an extra empty line at the end of
            // JsDirectiveList, for example:
            //```js
            // "use strict"; <- first leading new_line
            //  			 <- second leading new_line
            // function foo() {

            // }
            //```
            // so we should keep an extra empty line after JsDirectiveList
            let need_extra_empty_line = if let Some(next_sibling) = next_sibling {
                get_lines_before(&next_sibling) > 1
            } else {
                false
            };
            formatted![
                formatter,
                formatter.format_list(self.clone()),
                hard_line_break(),
                if need_extra_empty_line {
                    empty_line()
                } else {
                    empty_element()
                }
            ]
        } else {
            Ok(empty_element())
        }
    }
}
