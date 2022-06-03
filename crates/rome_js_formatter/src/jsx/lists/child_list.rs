use crate::generated::FormatJsxChildList;
use crate::prelude::*;
use crate::utils::jsx_utils::contains_meaningful_jsx_text;
use crate::{FormatElement, JsFormatter};
use rome_formatter::{fill_elements, FormatResult};
use rome_js_syntax::JsxChildList;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn format(node: &JsxChildList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let mut flattened_children = Vec::new();
        for child in formatter.format_all(node.iter().formatted())? {
            match child {
                FormatElement::List(list) => {
                    for item in list.into_vec() {
                        flattened_children.push(item);
                    }
                }
                item => flattened_children.push(item),
            }
        }

        if contains_meaningful_jsx_text(node) {
            Ok(fill_elements(soft_line_break(), flattened_children))
        } else {
            Ok(join_elements(soft_line_break(), flattened_children))
        }
    }
}
