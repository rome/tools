use super::kinds_src::AstSrc;
use crate::{to_upper_snake_case, LanguageKind, Result};
use quote::{format_ident, quote};

pub fn generate_macros(ast: &AstSrc, language_kind: LanguageKind) -> Result<String> {
    let syntax_kind = language_kind.syntax_kind();
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
                $crate::#syntax_kind::#node_kind => {
                    // SAFETY: The call to new_unchecked is guarded by matching on node.kind()
                    let $pattern = unsafe { $crate::#name::new_unchecked(node) };
                    $body
                }
            }
        })
        .collect();

    let ast = quote! {
        /// Reconstruct an AstNode from a SyntaxNode
        ///
        /// This macros performs a match over the [kind](SyntaxNode::kind)
        /// of the provided [SyntaxNode] and constructs the appropriate
        /// AstNode type for it, then execute the provided expression over it.
        ///
        /// The macro accepts an optional fallback branch wich defaults to
        /// `unreachable!()` as the only SyntaxKind variants not covered by
        /// this macro are token kinds that should not be used to construct
        /// a SyntaxNode.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// map_syntax_node!(syntax_node, node => node.format())
        /// ```
        ///
        /// ```ignore
        /// map_syntax_node!(syntax_node, node => Ok(node.format()), _ => Err("invalid node kind"))
        /// ```
        #[macro_export]
        macro_rules! map_syntax_node {
            ($node:expr, $pattern:pat => $body:expr) => {
                $crate::map_syntax_node!( $node, $pattern => $body, _ => unreachable!() )
            };

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
