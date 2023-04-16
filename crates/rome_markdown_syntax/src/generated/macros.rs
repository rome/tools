//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](rome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [rome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::MdSyntaxNode::kind(&node) {
                $crate::MdSyntaxKind::MD_HEADING => {
                    let $pattern = unsafe { $crate::MdHeading::new_unchecked(node) };
                    $body
                }
                $crate::MdSyntaxKind::MD_ROOT => {
                    let $pattern = unsafe { $crate::MdRoot::new_unchecked(node) };
                    $body
                }
                $crate::MdSyntaxKind::MD_TEXT => {
                    let $pattern = unsafe { $crate::MdText::new_unchecked(node) };
                    $body
                }
                $crate::MdSyntaxKind::MD_BOGUS => {
                    let $pattern = unsafe { $crate::MdBogus::new_unchecked(node) };
                    $body
                }
                $crate::MdSyntaxKind::MD_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::MdElementList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
