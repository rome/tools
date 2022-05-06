use crate::{
    empty_element, if_group_breaks, join_elements, soft_line_break_or_space,
    token, Format, FormatElement, Formatter,
};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTypeMemberList;
use rome_rowan::AstNodeList;

impl Format for TsTypeMemberList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let items = self.iter();
        let last_index = items.len().saturating_sub(1);

        let items = items
            .enumerate()
            .map(|(index, element)| {
                let formatted_element = element.format(formatter)?;

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

                formatted![formatter, formatted_element, separator]
            })
            .collect::<FormatResult<Vec<_>>>()?;

        Ok(join_elements(soft_line_break_or_space(), items))
    }
}
