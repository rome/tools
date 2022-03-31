use crate::macros::map_syntax_node;
use crate::{JsSyntaxElement, JsSyntaxElementChildren};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub JsSyntaxElementChildren);

impl Debug for DebugSyntaxElementChildren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(DebugSyntaxElement))
            .finish()
    }
}

struct DebugSyntaxElement(JsSyntaxElement);

impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            JsSyntaxElement::Node(node) => {
                map_syntax_node!(node.clone(), node => std::fmt::Debug::fmt(&node, f))
            }
            JsSyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
