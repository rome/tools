use crate::{
    concat_elements, empty_element, format_elements, formatter_traits::FormatTokenAndNode,
    soft_line_break_or_space, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::{AstSeparatedList, TsIntersectionTypeElementList};

impl ToFormatElement for TsIntersectionTypeElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let mut elements = Vec::with_capacity(self.len());
        let last_index = self.len().saturating_sub(1);

        for (index, item) in self.elements().enumerate() {
            let ty = item.node()?;
            let separator = item.trailing_separator()?;

            let separator = match separator {
                Some(token) => {
                    if index == last_index {
                        formatter.format_replaced(&token, empty_element())
                    } else {
                        format_elements![
                            soft_line_break_or_space(),
                            token.format(formatter)?,
                            space_token()
                        ]
                    }
                }
                None => {
                    if index == last_index {
                        empty_element()
                    } else {
                        format_elements![soft_line_break_or_space(), token("&"), space_token()]
                    }
                }
            };

            elements.push(format_elements![ty.format(formatter)?, separator])
        }

        Ok(concat_elements(elements))
    }
}
