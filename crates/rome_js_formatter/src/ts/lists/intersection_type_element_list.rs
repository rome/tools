use crate::generated::FormatTsIntersectionTypeElementList;
use crate::prelude::*;
use rome_js_syntax::TsIntersectionTypeElementList;
use rome_rowan::AstSeparatedList;

impl FormatRule<TsIntersectionTypeElementList> for FormatTsIntersectionTypeElementList {
    type Context = JsFormatContext;

    fn format(
        node: &TsIntersectionTypeElementList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let mut elements = Vec::with_capacity(node.len());
        let last_index = node.len().saturating_sub(1);

        for (index, item) in node.elements().enumerate() {
            let ty = formatted![formatter, [item.node().format()]]?;
            let separator = item.trailing_separator()?;

            let separator = match separator {
                Some(token) => {
                    if index == last_index {
                        formatter.format_replaced(token, empty_element())
                    } else {
                        formatted![
                            formatter,
                            [soft_line_break_or_space(), token.format(), space_token()]
                        ]?
                    }
                }
                None => {
                    if index == last_index {
                        empty_element()
                    } else {
                        formatted![
                            formatter,
                            [soft_line_break_or_space(), token("&"), space_token()]
                        ]?
                    }
                }
            };

            elements.push(format_elements![group_elements(ty), separator]);
        }

        Ok(concat_elements(elements))
    }
}
