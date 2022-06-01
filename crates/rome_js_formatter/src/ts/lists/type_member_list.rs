use crate::generated::FormatTsTypeMemberList;
use crate::prelude::*;
use rome_js_syntax::TsTypeMemberList;
use rome_rowan::AstNodeList;

impl FormatRule<TsTypeMemberList> for FormatTsTypeMemberList {
    type Context = JsFormatContext;

    fn format(node: &TsTypeMemberList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let items = node.iter();
        let last_index = items.len().saturating_sub(1);

        let items = items
            .enumerate()
            .map(|(index, element)| {
                let formatted_element = formatted![formatter, [element.format()]]?;

                let is_verbatim = matches!(
                    formatted_element.last_element(),
                    Some(FormatElement::Verbatim(_))
                );

                let separator = if !is_verbatim {
                    // Children don't format the separator on purpose, so it's up to the parent - this node,
                    // to decide to print their separator
                    if index == last_index {
                        if_group_breaks(token(";"))
                    } else {
                        token(";")
                    }
                } else {
                    empty_element()
                };

                Ok(format_elements![
                    group_elements(formatted_element),
                    separator
                ])
            })
            .collect::<FormatResult<Vec<_>>>()?;

        Ok(join_elements(soft_line_break_or_space(), items))
    }
}
