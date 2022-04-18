use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_formatter::{concat_elements, group_elements, soft_line_indent_or_space};
use rome_js_syntax::{
    JsAnyExpression, JsSequenceExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::AstNode;

impl ToFormatElement for JsSequenceExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_sequence_expression(self.clone(), formatter)
    }
}

#[derive(Debug, Default)]
struct FlattenItemList {
    item_list: Vec<FormatElement>,
}

impl FlattenItemList {
    pub fn push(&mut self, item: FormatElement) {
        self.item_list.push(item);
    }

    pub fn into_format_element(self) -> FormatElement {
        let element_list = self.item_list.into_iter().enumerate().map(|(index, item)| {
            if index == 0 {
                item
            } else {
                format_elements![soft_line_indent_or_space(item)]
            }
        });

        group_elements(concat_elements(element_list))
    }
}

fn format_sequence_expression(
    node: JsSequenceExpression,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    let mut flatten_item_list = FlattenItemList::default();

    flat_sequence_expression(&mut flatten_item_list, node, formatter, None)?;

    Ok(group_elements(flatten_item_list.into_format_element()))
}

fn flat_sequence_expression(
    list: &mut FlattenItemList,
    sequence_expression: JsSequenceExpression,
    formatter: &Formatter,
    previous_comma_token: Option<JsSyntaxToken>,
) -> FormatResult<()> {
    let formatted_comma = previous_comma_token.format_or_empty(formatter)?;
    let left = sequence_expression.left()?;
    let right = sequence_expression.right()?;
    let comma = sequence_expression.comma_token()?;
    if let JsAnyExpression::JsSequenceExpression(sequence_expression) = left {
        flat_sequence_expression(list, sequence_expression, formatter, Some(comma))?;
        let formatted = format_elements![space_token(), right.format(formatter)?, formatted_comma];
        list.push(formatted)
    } else {
        list.push(format_elements![
            left.format(formatter)?,
            comma.format(formatter)?,
        ]);
        list.push(format_elements![
            space_token(),
            right.format(formatter)?,
            formatted_comma
        ])
    }

    Ok(())
}
