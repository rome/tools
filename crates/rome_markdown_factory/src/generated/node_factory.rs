//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use rome_markdown_syntax::{
    MdSyntaxElement as SyntaxElement, MdSyntaxNode as SyntaxNode, MdSyntaxToken as SyntaxToken, *,
};
use rome_rowan::AstNode;
pub fn md_heading(heading_level_token: SyntaxToken) -> MdHeadingBuilder {
    MdHeadingBuilder {
        heading_level_token,
        value: None,
    }
}
pub struct MdHeadingBuilder {
    heading_level_token: SyntaxToken,
    value: Option<MdText>,
}
impl MdHeadingBuilder {
    pub fn with_value(mut self, value: MdText) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> MdHeading {
        MdHeading::unwrap_cast(SyntaxNode::new_detached(
            MdSyntaxKind::MD_HEADING,
            [
                Some(SyntaxElement::Token(self.heading_level_token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn md_root(value: MdElementList, eof_token: SyntaxToken) -> MdRoot {
    MdRoot::unwrap_cast(SyntaxNode::new_detached(
        MdSyntaxKind::MD_ROOT,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn md_text(value_token: SyntaxToken) -> MdText {
    MdText::unwrap_cast(SyntaxNode::new_detached(
        MdSyntaxKind::MD_TEXT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn md_element_list<I>(items: I) -> MdElementList
where
    I: IntoIterator<Item = AnyMdElement>,
    I::IntoIter: ExactSizeIterator,
{
    MdElementList::unwrap_cast(SyntaxNode::new_detached(
        MdSyntaxKind::MD_ELEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn md_bogus<I>(slots: I) -> MdBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MdBogus::unwrap_cast(SyntaxNode::new_detached(MdSyntaxKind::MD_BOGUS, slots))
}
