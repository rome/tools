use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_formatter::{concat_elements, group_elements, soft_line_indent_or_space};
use rome_js_syntax::{JsSequenceExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::AstNode;

impl ToFormatElement for JsSequenceExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_sequence_expression(self, formatter)
    }
}

#[derive(Debug)]
enum FlattenItem {
    SequenceExpression(JsSequenceExpression, FormatElement),
    OtherNode(JsSyntaxNode, FormatElement),
}

impl FlattenItem {
    pub fn into_format_element(self) -> FormatElement {
        match self {
            FlattenItem::SequenceExpression(_, element) => element,
            FlattenItem::OtherNode(_, element) => element,
        }
    }
}

#[derive(Debug, Default)]
struct FlattenItemList {
    item_list: Vec<FlattenItem>,
}

impl FlattenItemList {
    pub fn push(&mut self, item: FlattenItem) {
        self.item_list.push(item);
    }

    pub fn into_format_element(self) -> FormatElement {
        let element_list = self.item_list.into_iter().enumerate().map(|(index, item)| {
            if index == 0 {
                item.into_format_element()
            } else {
                format_elements![soft_line_indent_or_space(item.into_format_element())]
            }
        });

        group_elements(concat_elements(element_list))
    }
}

fn format_sequence_expression(
    node: &JsSequenceExpression,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    let mut flatten_item_list = FlattenItemList::default();

    flat_sequence_expression(
        &mut flatten_item_list,
        node.syntax().clone(),
        formatter,
        None,
    )?;

    Ok(group_elements(flatten_item_list.into_format_element()))
}

fn flat_sequence_expression(
    list: &mut FlattenItemList,
    node: JsSyntaxNode,
    formatter: &Formatter,
    previous_comma_token: Option<JsSyntaxToken>,
) -> FormatResult<()> {
    let formatted_comma = previous_comma_token.format_or_empty(formatter)?;
    match node.kind() {
        JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
            // SAFETY: protected from the previous match
            let sequence_expression = JsSequenceExpression::cast(node).unwrap();
            let left = sequence_expression.left()?;
            let right = sequence_expression.right()?;
            let comma = sequence_expression.comma_token()?;
            flat_sequence_expression(list, left.syntax().clone(), formatter, Some(comma))?;
            let formatted =
                format_elements![space_token(), right.format(formatter)?, formatted_comma];
            list.push(FlattenItem::SequenceExpression(
                sequence_expression,
                formatted,
            ))
        }
        _ => {
            let formatted = format_elements![node.to_format_element(formatter)?, formatted_comma];
            list.push(FlattenItem::OtherNode(node, formatted))
        }
    }

    Ok(())
}
