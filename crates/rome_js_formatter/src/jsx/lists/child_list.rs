use crate::generated::FormatJsxChildList;
use crate::prelude::*;
use crate::utils::jsx_utils::contains_text;
use crate::{FormatElement, Formatter, JsFormatter};
use rome_formatter::{empty_element, fill_elements, FormatResult};
use rome_js_syntax::JsxChildList;
use rome_rowan::AstNode;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn format(node: &JsxChildList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let children = formatter
            .format_list(node)
            .into_iter()
            .map(|(_, element)| element);

        println!();
        println!("\"{}\"", node.syntax().text());
        println!("{:#?}", node);
        println!("{:#?}", children);
        println!();

        if contains_text(node) {
            Ok(fill_elements(empty_element(), children))
        } else {
            Ok(join_elements(
                soft_line_break(),
                children.filter(|element| !element.is_empty_string()),
                //children,
            ))
        }
    }
}
