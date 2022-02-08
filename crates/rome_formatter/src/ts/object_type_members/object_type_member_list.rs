use crate::{
    format_elements, if_group_breaks, join_elements, soft_line_break_or_space, token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsObjectTypeMemberList;
use rslint_parser::AstNodeList;

impl ToFormatElement for TsObjectTypeMemberList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let items = self.iter();
        let last_index = items.len().saturating_sub(1);

        let items = items
            .enumerate()
            .map(|(index, element)| {
                let formatted_element = element.to_format_element(formatter)?;

                // Children don't format the separator on purpose, so it's up to the parent - this node,
                // to decide to print their separator
                let separator = if index == last_index {
                    if_group_breaks(token(","))
                } else {
                    token(",")
                };

                Ok(format_elements![formatted_element, separator])
            })
            .collect::<FormatResult<Vec<_>>>()?;

        Ok(join_elements(soft_line_break_or_space(), items))
    }
}
