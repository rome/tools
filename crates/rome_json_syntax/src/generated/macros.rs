//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](SyntaxNode::kind)"]
#[doc = r" of the provided [SyntaxNode] and constructs the appropriate"]
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
            node => match $crate::JsonSyntaxNode::kind(&node) {
                $crate::JsonSyntaxKind::JSON_ARRAY => {
                    let $pattern = unsafe { $crate::JsonArray::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_BOOLEAN => {
                    let $pattern = unsafe { $crate::JsonBoolean::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_MEMBER => {
                    let $pattern = unsafe { $crate::JsonMember::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_NULL => {
                    let $pattern = unsafe { $crate::JsonNull::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_NUMBER => {
                    let $pattern = unsafe { $crate::JsonNumber::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_OBJECT => {
                    let $pattern = unsafe { $crate::JsonObject::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_ROOT => {
                    let $pattern = unsafe { $crate::JsonRoot::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_STRING => {
                    let $pattern = unsafe { $crate::JsonString::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_UNKNOWN => {
                    let $pattern = unsafe { $crate::JsonUnknown::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_ARRAY_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::JsonArrayElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::JsonMemberList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
