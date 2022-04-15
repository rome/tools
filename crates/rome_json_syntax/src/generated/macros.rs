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
                $crate::JsonSyntaxKind::JSON_ARRAY_STATEMENT => {
                    let $pattern = unsafe { $crate::JsonArrayStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_BOOLEAN_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsonBooleanLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_NULL_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsonNullLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_NUMBER_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsonNumberLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_OBJECT_STATEMENT => {
                    let $pattern = unsafe { $crate::JsonObjectStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_OBJECT_VALUE => {
                    let $pattern = unsafe { $crate::JsonObjectValue::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_STRING_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsonStringLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_UNKNOWN => {
                    let $pattern = unsafe { $crate::JsonUnknown::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_ARRAY_VALUE_LIST => {
                    let $pattern = unsafe { $crate::JsonArrayValueList::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_OBJECT_VALUE_LIST => {
                    let $pattern = unsafe { $crate::JsonObjectValueList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
