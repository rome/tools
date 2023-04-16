//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_markdown_syntax::{MdSyntaxKind, MdSyntaxKind::*, T, *};
use rome_rowan::{AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind};
#[derive(Debug)]
pub struct MdSyntaxFactory;
impl SyntaxFactory for MdSyntaxFactory {
    type Kind = MdSyntaxKind;
    #[allow(unused_mut)]
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            MD_BOGUS => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
            MD_HEADING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if matches!(
                        element.kind(),
                        T ! [#] | T ! [##] | T ! [###] | T ! [####] | T ! [#####]
                    ) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if MdText::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MD_HEADING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MD_HEADING, children)
            }
            MD_ROOT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if MdElementList::can_cast(element.kind()) {
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
                    return RawSyntaxNode::new(MD_ROOT.to_bogus(), children.into_iter().map(Some));
                }
                slots.into_node(MD_ROOT, children)
            }
            MD_TEXT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == MD_STRING_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(MD_TEXT.to_bogus(), children.into_iter().map(Some));
                }
                slots.into_node(MD_TEXT, children)
            }
            MD_ELEMENT_LIST => Self::make_node_list_syntax(kind, children, AnyMdElement::can_cast),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
