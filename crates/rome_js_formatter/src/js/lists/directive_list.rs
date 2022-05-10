use crate::generated::FormatJsDirectiveList;
use crate::prelude::*;
use rome_js_syntax::JsDirectiveList;
use rome_rowan::{AstNode, AstNodeList};

impl FormatRule<JsDirectiveList> for FormatJsDirectiveList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsDirectiveList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        if !node.is_empty() {
            let syntax_node = node.syntax();
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
                [
                    formatter.format_list_with_hard_line(node),
                    hard_line_break(),
                    if need_extra_empty_line {
                        empty_line()
                    } else {
                        empty_element()
                    }
                ]
            ]
        } else {
            Ok(empty_element())
        }
    }
}
