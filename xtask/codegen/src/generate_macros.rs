use super::kinds_src::AstSrc;
use crate::{to_upper_snake_case, Result};
use quote::{format_ident, quote};

pub fn generate_macros(ast: &AstSrc) -> Result<String> {
    let match_arms: Vec<_> = ast
        .nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name);
            let node_kind = format_ident!("{}", to_upper_snake_case(&node.name));
            (name, node_kind)
        })
        .chain(ast.unknowns.iter().map(|node_name| {
            let name = format_ident!("{}", node_name);
            let node_kind = format_ident!("{}", to_upper_snake_case(node_name));
            (name, node_kind)
        }))
        .chain(ast.lists().map(|(node_name, _)| {
            let name = format_ident!("{}", node_name);
            let node_kind = format_ident!("{}", to_upper_snake_case(node_name));
            (name, node_kind)
        }))
        .map(|(name, node_kind)| {
            quote! {
                $crate::JsSyntaxKind::#node_kind => {
                    // SAFETY: The call to new_unchecked is guarded by matching on node.kind()
                    let $pattern = unsafe { $crate::ast::#name::new_unchecked(node) };
                    $body
                }
            }
        })
        .collect();

    let ast = quote! {
        #[macro_export]
        macro_rules! map_syntax_node {
            ($node:expr, $pattern:pat => $body:expr, $fallback:pat => $default:expr) => {
                match $node {
                    node => match $crate::SyntaxNode::kind(&node) {
                        #( #match_arms, )*
                        $fallback => $default
                    }
                }
            };
        }
    };

    xtask::reformat(ast)
}
