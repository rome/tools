//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{generated::nodes::*, JsonSyntaxKind, JsonSyntaxKind::*, T};
use rome_rowan::{AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind};
#[derive(Debug)]
pub struct JsonSyntaxFactory;
impl SyntaxFactory for JsonSyntaxFactory {
    type Kind = JsonSyntaxKind;
    #[allow(unused_mut)]
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            JSON_UNKNOWN => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
            JSON_ARRAY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T!['['] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if JsonArrayValueList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![']'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        JSON_ARRAY.to_unknown(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(JSON_ARRAY, children)
            }
            JSON_BOOLEAN_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if matches!(element.kind(), T![true] | T![false]) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        JSON_BOOLEAN_LITERAL_EXPRESSION.to_unknown(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(JSON_BOOLEAN_LITERAL_EXPRESSION, children)
            }
            JSON_DOCUMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if JsonAnyValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if JsonUnknown::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![EOF] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        JSON_DOCUMENT.to_unknown(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(JSON_DOCUMENT, children)
            }
            JSON_NULL_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![null] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        JSON_NULL_LITERAL_EXPRESSION.to_unknown(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(JSON_NULL_LITERAL_EXPRESSION, children)
            }
            JSON_NUMBER_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == JSON_NUMBER_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        JSON_NUMBER_LITERAL_EXPRESSION.to_unknown(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(JSON_NUMBER_LITERAL_EXPRESSION, children)
            }
            JSON_OBJECT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T!['{'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if JsonObjectValueList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T!['}'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        JSON_OBJECT.to_unknown(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(JSON_OBJECT, children)
            }
            JSON_OBJECT_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if JsonStringLiteralExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [:] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if JsonAnyValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        JSON_OBJECT_VALUE.to_unknown(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(JSON_OBJECT_VALUE, children)
            }
            JSON_STRING_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == JSON_STRING_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        JSON_STRING_LITERAL_EXPRESSION.to_unknown(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(JSON_STRING_LITERAL_EXPRESSION, children)
            }
            JSON_ARRAY_VALUE_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                JsonAnyValue::can_cast,
                T ! [,],
                false,
            ),
            JSON_OBJECT_VALUE_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                JsonObjectValue::can_cast,
                T ! [,],
                false,
            ),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
