use crate::formatter::TryFormatNodeListExtension;
use crate::generated::FormatJsDirectiveList;
use crate::prelude::*;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsDirectiveList;
use rome_rowan::{AstNode, AstNodeList};

impl FormatRule<JsDirectiveList> for FormatJsDirectiveList {
    type Context = JsFormatContext;

    fn format(node: &JsDirectiveList, f: &mut JsFormatter) -> FormatResult<()> {
        if node.is_empty() {
            return Ok(());
        }

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

        f.join_with(&hard_line_break())
            .entries(node.try_format_nodes())
            .finish()?;

        if need_extra_empty_line {
            write!(f, [empty_line()])
        } else {
            write!(f, [hard_line_break()])
        }
    }
}
