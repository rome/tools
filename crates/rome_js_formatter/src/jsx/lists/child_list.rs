use crate::generated::FormatJsxChildList;
use crate::prelude::*;
use crate::utils::jsx_utils::contains_meaningful_jsx_text;
use crate::{FormatElement, JsFormatter};
use rome_formatter::{empty_element, fill_elements, FormatResult};
use rome_js_syntax::JsxChildList;
use std::ops::Deref;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn format(node: &JsxChildList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let children = formatter.format_all(node.iter().formatted())?;

        if contains_meaningful_jsx_text(node) {
            println!("1");
            Ok(fill_elements(empty_element(), children))
        } else {
            println!("2");
            Ok(join_elements(
                soft_line_break(),
                children.filter(|element| match element {
                    FormatElement::Token(token) => token.deref() != "",
                    _ => true,
                }),
            ))
        }
    }
}
