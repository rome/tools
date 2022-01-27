use crate::{
    empty_element, format_elements, group_elements, indent, join_elements,
    soft_line_break_or_space, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::{
    ast::{JsVariableDeclaration, JsVariableDeclarations, JsVariableStatement},
    AstSeparatedList,
};

impl ToFormatElement for JsVariableStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_node(self.declarations()?)?,
            formatter
                .format_token(&self.semicolon_token())?
                .unwrap_or_else(|| token(";")),
        ])
    }
}

impl ToFormatElement for JsVariableDeclarations {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let items = self.items();
        let last_index = items.len().saturating_sub(1);

        let items = items
            .elements()
            .enumerate()
            .map(|(index, element)| {
                let node = formatter.format_node(element.node()?)?;
                let separator = if let Some(separator) = element.trailing_separator()? {
                    if index == last_index {
                        formatter.format_replaced(&separator, empty_element())?
                    } else {
                        formatter.format_token(&separator)?
                    }
                } else if index == last_index {
                    empty_element()
                } else {
                    token(",")
                };

                Ok(format_elements![node, separator])
            })
            .collect::<FormatResult<Vec<_>>>()?;

        let mut items = items.into_iter();

        let leading_element = items.next();
        let trailing_elements = join_elements(soft_line_break_or_space(), items);

        Ok(format_elements![
            formatter.format_token(&self.kind()?)?,
            space_token(),
            group_elements(concat_elements(leading_element.into_iter().chain(
                if !trailing_elements.is_empty() {
                    Some(indent(format_elements![
                        soft_line_break_or_space(),
                        trailing_elements
                    ]))
                } else {
                    None
                }
            ))),
        ])
    }
}

impl ToFormatElement for JsVariableDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let initializer = if let Some(initializer) = self.initializer() {
            format_elements![space_token(), formatter.format_node(initializer)?]
        } else {
            empty_element()
        };

        Ok(format_elements![
            formatter.format_node(self.id()?)?,
            initializer
        ])
    }
}
