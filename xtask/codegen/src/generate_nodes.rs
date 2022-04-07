use crate::css_kinds_src::CSS_KINDS_SRC;
use crate::kinds_src::{AstSrc, Field, TokenKind, JS_KINDS_SRC};
use crate::{to_lower_snake_case, to_upper_snake_case, LanguageKind};
use proc_macro2::Literal;
use quote::{format_ident, quote};
use std::collections::HashMap;
use xtask::Result;

pub fn generate_nodes(ast: &AstSrc, language_kind: LanguageKind) -> Result<String> {
    let syntax_kind = language_kind.syntax_kind();

    let (node_defs, node_boilerplate_impls): (Vec<_>, Vec<_>) = ast
        .nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name);
            let node_kind = format_ident!("{}", to_upper_snake_case(node.name.as_str()));

            let methods = node
                .fields
                .iter()
                .enumerate()
                .map(|(slot_index, field)| match field {
                    Field::Token { name, kind, .. } => {
                        let many = matches!(kind, TokenKind::Many(_));

                        let method_name = if many {
                            format_ident!("{}", name)
                        } else {
                            field.method_name(language_kind)
                        };

                        let is_optional = field.is_optional();

                        if is_optional {
                            quote! {
                                pub fn #method_name(&self) -> Option<SyntaxToken> {
                                    support::token(&self.syntax, #slot_index)
                                }
                            }
                        } else {
                            quote! {
                                pub fn #method_name(&self) -> SyntaxResult<SyntaxToken> {
                                    support::required_token(&self.syntax, #slot_index)
                                }
                            }
                        }
                    }
                    Field::Node { ty, optional, .. } => {
                        let is_list = ast.is_list(ty);
                        let ty = format_ident!("{}", &ty);

                        let method_name = field.method_name(language_kind);
                        if is_list {
                            quote! {
                                pub fn #method_name(&self) -> #ty {
                                    support::list(&self.syntax, #slot_index)
                                }
                            }
                        } else if *optional {
                            quote! {
                                pub fn #method_name(&self) -> Option<#ty> {
                                    support::node(&self.syntax, #slot_index)
                                }
                            }
                        } else {
                            quote! {
                                pub fn #method_name(&self) -> SyntaxResult<#ty> {
                                    support::required_node(&self.syntax, #slot_index)
                                }
                            }
                        }
                    }
                });

            let fields = node.fields.iter().map(|field| {
                let name = match field {
                    Field::Token {
                        name,
                        kind: TokenKind::Many(_),
                        ..
                    } => format_ident!("{}", name),
                    _ => field.method_name(language_kind),
                };

                let is_list = match field {
                    Field::Node { ty, .. } => ast.is_list(ty),
                    _ => false,
                };

                let string_name = name.to_string();

                if is_list {
                    quote! {
                        .field(#string_name, &self.#name())
                    }
                } else if field.is_optional() {
                    quote! {
                        .field(#string_name, &support::DebugOptionalElement(self.#name()))
                    }
                } else {
                    quote! {
                        .field(#string_name, &support::DebugSyntaxResult(self.#name()))
                    }
                }
            });

            let string_name = name.to_string();

            let slots_name = format_ident!("{}Fields", node.name);

            let (slot_fields, slot_constructors): (Vec<_>, Vec<_>) = node
                .fields
                .iter()
                .map(|field| match field {
                    Field::Token { name, kind, .. } => {
                        let many = matches!(kind, TokenKind::Many(_));

                        let method_name = if many {
                            format_ident!("{}", name)
                        } else {
                            field.method_name(language_kind)
                        };

                        let is_optional = field.is_optional();

                        let field = if is_optional {
                            quote! { #method_name: Option<SyntaxToken> }
                        } else {
                            quote! { #method_name: SyntaxResult<SyntaxToken> }
                        };

                        (field, quote! { #method_name: self.#method_name() })
                    }
                    Field::Node { ty, optional, .. } => {
                        let is_list = ast.is_list(ty);
                        let ty = format_ident!("{}", &ty);

                        let method_name = field.method_name(language_kind);
                        let field = if is_list {
                            quote! { #method_name: #ty }
                        } else if *optional {
                            quote! { #method_name: Option<#ty> }
                        } else {
                            quote! { #method_name: SyntaxResult<#ty> }
                        };

                        (field, quote! { #method_name: self.#method_name() })
                    }
                })
                .unzip();

            (
                quote! {
                    // TODO: review documentation
                    // #[doc = #documentation]
                    #[derive(Clone, PartialEq, Eq, Hash)]
                    pub struct #name {
                        pub(crate) syntax: SyntaxNode,
                    }

                    impl #name {
                        /// Create an AstNode from a SyntaxNode without checking its kind
                        ///
                        /// # Safety
                        /// This function must be guarded with a call to [AstNode::can_cast]
                        /// or a match on [SyntaxNode::kind]
                        #[inline]
                        pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
                            Self { syntax }
                        }

                        pub fn as_fields(&self) -> #slots_name {
                            #slots_name {
                                #( #slot_constructors, )*
                            }
                        }

                        #(#methods)*
                    }

                    pub struct #slots_name {
                        #( pub #slot_fields, )*
                    }
                },
                quote! {
                    impl AstNode for #name {
                        fn can_cast(kind: #syntax_kind) -> bool {
                            kind == #node_kind
                        }
                        fn cast(syntax: SyntaxNode) -> Option<Self> {
                            if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                        }
                        fn syntax(&self) -> &SyntaxNode { &self.syntax }
                    }

                    impl std::fmt::Debug for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.debug_struct(#string_name)
                                #(#fields)*
                                .finish()
                        }
                    }

                    impl From<#name> for SyntaxNode {
                        fn from(n: #name) -> SyntaxNode {
                            n.syntax
                        }
                    }

                    impl From<#name> for SyntaxElement {
                        fn from(n: #name) -> SyntaxElement {
                            n.syntax.into()
                        }
                    }
                },
            )
        })
        .unzip();

    // it maps enum name A and its corresponding variants
    let name_to_variants: HashMap<_, _> = ast
        .unions
        .iter()
        .map(|current_enum| (current_enum.name.clone(), current_enum.variants.clone()))
        .collect();

    let (union_defs, union_boilerplate_impls): (Vec<_>, Vec<_>) = ast
        .unions
        .iter()
        .map(|union| {
            // here we collect all the variants because this will generate the enums
            // so we don't care about filtered variants
            let variants_for_union: Vec<_> = union
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = format_ident!("{}", variant);

                    quote! {
                        #variant_name(#variant_name)
                    }
                })
                .collect();

            // Here we make the partition
            //
            // Inside an enum, we can have variants that point to a "flat" type or to another enum;
            // we want to divide these variants as we will generate a different code based on these requirements
            let (variant_of_variants, simple_variants): (Vec<_>, Vec<_>) =
                union.variants.iter().partition(|current_enum| {
                    if let Some(variants) = name_to_variants.get(*current_enum) {
                        !variants.is_empty()
                    } else {
                        false
                    }
                });

            let variants: Vec<_> = simple_variants
                .iter()
                .map(|var| format_ident!("{}", var))
                .collect();

            let name = format_ident!("{}", union.name);
            let kinds: Vec<_> = variants
                .iter()
                .map(|name| format_ident!("{}", to_upper_snake_case(&name.to_string())))
                .collect();

            let variant_cast: Vec<_> = simple_variants
                .iter()
                .map(|current_enum| {
                    let variant_is_enum = ast.unions.iter().find(|e| &e.name == *current_enum);
                    let variant_name = format_ident!("{}", current_enum);

                    if variant_is_enum.is_some() {
                        quote! {
                            #variant_name::cast(syntax)?
                        }
                    } else {
                        quote! {
                            #variant_name { syntax }
                        }
                    }
                })
                .collect();

            // variant of variants
            let vv: Vec<_> = variant_of_variants
                .iter()
                .enumerate()
                .map(|(i, en)| {
                    let variant_name = format_ident!("{}", en);
                    let variable_name = format_ident!("{}", to_lower_snake_case(en.as_str()));
                    (
                        // cast() code
                        if i != variant_of_variants.len() - 1 {
                            quote! {
                            if let Some(#variable_name) = #variant_name::cast(syntax.clone()) {
                                    return Some(#name::#variant_name(#variable_name));
                            }}
                        } else {
                            // if this is the last variant, do not clone syntax
                            quote! {
                                if let Some(#variable_name) = #variant_name::cast(syntax) {
                                    return Some(#name::#variant_name(#variable_name));
                            }}
                        },
                        // can_cast() code
                        quote! {
                            k if #variant_name::can_cast(k) => true,
                        },
                        // syntax() code
                        quote! {
                            #name::#variant_name(it) => it.syntax()
                        },
                    )
                })
                .collect();

            let vv_cast = vv.iter().map(|v| v.0.clone());

            let vv_can_cast = vv.iter().map(|v| v.1.clone());
            let vv_syntax = vv.iter().map(|v| v.2.clone());

            let all_kinds = if !kinds.is_empty() {
                quote! {
                    #(#kinds)|* => true,
                }
            } else {
                quote! {}
            };

            let cast_fn = if !kinds.is_empty() {
                quote! {
                    let res = match syntax.kind() {
                        #(
                            #kinds => #name::#variants(#variant_cast),
                        )*
                        _ =>  {
                            #(
                                #vv_cast
                            )*
                            return None
                        }
                    };
                    Some(res)
                }
            } else {
                quote! {
                        #(
                        #vv_cast
                    )*
                    None
                }
            };

            let can_cast_fn = if union.variants.iter().any(|v| !simple_variants.contains(&v)) {
                quote! {
                    match kind {
                        #all_kinds
                        #(#vv_can_cast)*
                        _ => false
                    }
                }
            } else {
                quote! {
                    matches!(kind, #(#kinds)|*)
                }
            };

            let variant_can_cast: Vec<_> = simple_variants
                .iter()
                .map(|_| {
                    quote! {
                        &it.syntax
                    }
                })
                .collect();

            let all_variant_names: Vec<_> = union
                .variants
                .iter()
                .map(|variant| format_ident!("{}", variant))
                .collect();

            (
                quote! {
                    // #[doc = #doc]
                    #[derive(Clone, PartialEq, Eq, Hash)]
                    pub enum #name {
                        #(#variants_for_union),*
                    }
                },
                quote! {
                    #(
                    impl From<#variants> for #name {
                        fn from(node: #variants) -> #name {
                            #name::#variants(node)
                        }
                    }
                    )*

                    impl AstNode for #name {
                        fn can_cast(kind: #syntax_kind) -> bool {
                            #can_cast_fn
                        }
                        fn cast(syntax: SyntaxNode) -> Option<Self> {
                                #cast_fn
                        }
                        fn syntax(&self) -> &SyntaxNode {
                            match self {
                                #(
                                #name::#variants(it) => #variant_can_cast,
                                )*
                                #(
                                    #vv_syntax
                                ),*

                            }
                        }
                    }

                    impl std::fmt::Debug for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            match self {
                            #(
                                #name::#all_variant_names(it) => std::fmt::Debug::fmt(it, f),
                            )*
                        }
                            }
                    }

                    impl From<#name> for SyntaxNode {
                        fn from(n: #name) -> SyntaxNode {
                            match n {
                                #(
                                #name::#all_variant_names(it) => it.into(),
                                )*
                            }
                        }
                    }

                    impl From<#name> for SyntaxElement {
                        fn from(n: #name) -> SyntaxElement {
                            let node: SyntaxNode = n.into();
                            node.into()
                        }
                    }
                },
            )
        })
        .unzip();

    let union_names = ast.unions.iter().map(|it| &it.name);
    let node_names = ast.nodes.iter().map(|it| &it.name);

    let display_impls = union_names
        .chain(node_names.clone())
        .map(|it| format_ident!("{}", it))
        .map(|name| {
            quote! {
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        std::fmt::Display::fmt(self.syntax(), f)
                    }
                }
            }
        });

    let unknowns = ast.unknowns.iter().map(|unknown| {
        let name = format_ident!("{}", unknown);
        let string_name = unknown;
        let kind = format_ident!("{}", to_upper_snake_case(unknown));

        quote! {
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct #name {
                syntax: SyntaxNode
            }

            impl #name {
                /// Create an AstNode from a SyntaxNode without checking its kind
                ///
                /// # Safety
                /// This function must be guarded with a call to [AstNode::can_cast]
                /// or a match on [SyntaxNode::kind]
                #[inline]
                pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
                    Self { syntax }
                }

                pub fn items(&self) -> SyntaxElementChildren {
                    support::elements(&self.syntax)
                }
            }

            impl AstNode for #name {
                fn can_cast(kind: #syntax_kind) -> bool {
                    kind == #kind
                }

                fn cast(syntax: SyntaxNode) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) {
                        Some(Self { syntax })
                    } else {
                        None
                    }
                }
                fn syntax(&self) -> &SyntaxNode {
                    &self.syntax
                }
            }

            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(#string_name)
                        .field("items", &support::DebugSyntaxElementChildren(self.items()))
                        .finish()
                }
            }

            impl From<#name> for SyntaxNode {
                fn from(n: #name) -> SyntaxNode {
                    n.syntax
                }
            }

            impl From<#name> for SyntaxElement {
                fn from(n: #name) -> SyntaxElement {
                    n.syntax.into()
                }
            }
        }
    });

    let lists = ast.lists().map(|(name, list)| {
        let list_name = format_ident!("{}", name);
        let list_kind = format_ident!("{}", to_upper_snake_case(name));
        let element_type = format_ident!("{}", list.element_name);

        let node_impl = quote! {
            impl #list_name {
                /// Create an AstNode from a SyntaxNode without checking its kind
                ///
                /// # Safety
                /// This function must be guarded with a call to [AstNode::can_cast]
                /// or a match on [SyntaxNode::kind]
                #[inline]
                pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
                    Self { syntax_list: syntax.into_list() }
                }
            }

            impl AstNode for #list_name {
                fn can_cast(kind: #syntax_kind) -> bool {
                    kind == #list_kind
                }

                fn cast(syntax: SyntaxNode) -> Option<#list_name> {
                    if Self::can_cast(syntax.kind()) {
                        Some(#list_name { syntax_list: syntax.into_list() })
                    } else {
                        None
                    }
                }

                fn syntax(&self) -> &SyntaxNode {
                    self.syntax_list.node()
                }
            }
        };

        let padded_name = format!("{} ", name);
        let list_impl = if list.separator.is_some() {
            quote! {
                impl AstSeparatedList<#element_type> for #list_name {
                    fn syntax_list(&self) -> &SyntaxList {
                        &self.syntax_list
                    }
                }

                impl Debug for #list_name {
                    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                        f.write_str(#padded_name)?;
                        f.debug_list().entries(self.elements()).finish()
                    }
                }

                impl IntoIterator for #list_name {
                    type Item = SyntaxResult<#element_type>;
                    type IntoIter = AstSeparatedListNodesIterator<#element_type>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }

                impl IntoIterator for &#list_name {
                    type Item = SyntaxResult<#element_type>;
                    type IntoIter = AstSeparatedListNodesIterator<#element_type>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }
            }
        } else {
            quote! {
                impl AstNodeList<#element_type> for #list_name {
                    fn syntax_list(&self) -> &SyntaxList {
                        &self.syntax_list
                    }
                }

                impl Debug for #list_name {
                    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                        f.write_str(#padded_name)?;
                        f.debug_list().entries(self.iter()).finish()
                    }
                }

                impl IntoIterator for &#list_name {
                    type Item = #element_type;
                    type IntoIter = AstNodeListIterator<#element_type>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }

                impl IntoIterator for #list_name {
                    type Item = #element_type;
                    type IntoIter = AstNodeListIterator<#element_type>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.iter()
                    }
                }

            }
        };

        quote! {
            #[derive(Clone, Eq, PartialEq, Hash)]
            pub struct #list_name {
              syntax_list: SyntaxList,
            }

            #node_impl
            #list_impl
        }
    });

    let debug_syntax_element = {
        let mut all_nodes: Vec<_> = ast
            .nodes
            .iter()
            .map(|node| &node.name)
            .chain(ast.unknowns.iter())
            .chain(ast.lists().map(|(name, _)| name))
            .collect();

        all_nodes.sort_unstable();

        let node_arms = all_nodes.iter().map(|node| {
            let kind = format_ident!("{}", to_upper_snake_case(node));
            let ident = format_ident!("{}", node);

            quote! {
                #kind => std::fmt::Debug::fmt(&#ident::cast(node.clone()).unwrap(), f)
            }
        });

        quote! {
            pub struct DebugSyntaxElement(pub(crate) SyntaxElement);

            impl Debug for DebugSyntaxElement {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    match &self.0 {
                        NodeOrToken::Node(node) => match node.kind() {
                            #(#node_arms),*,
                            _ => std::fmt::Debug::fmt(node, f),
                        },
                        NodeOrToken::Token(token) => Debug::fmt(token, f),
                    }
                }
            }
        }
    };

    let ast = quote! {
        #![allow(clippy::enum_variant_names)]
        // sometimes we generate comparison of simple tokens
        #![allow(clippy::match_like_matches_macro)]
        use crate::{
            ast::*,
            #syntax_kind::{self, *},
            SyntaxElement, SyntaxElementChildren, SyntaxList, SyntaxNode, SyntaxResult, SyntaxToken,
        };
        use rome_rowan::NodeOrToken;
        use std::fmt::{Debug, Formatter};

        #(#node_defs)*
        #(#union_defs)*
        #(#node_boilerplate_impls)*
        #(#union_boilerplate_impls)*
        #(#display_impls)*
        #(#unknowns)*
        #(#lists)*
        #debug_syntax_element
    };

    let ast = ast
        .to_string()
        .replace("T ! [ ", "T![")
        .replace(" ] )", "])");

    let pretty = xtask::reformat(ast)?;
    Ok(pretty)
}

pub(crate) fn token_kind_to_code(
    name: &str,
    language_kind: LanguageKind,
) -> proc_macro2::TokenStream {
    let kind_variant_name = to_upper_snake_case(name);

    let kind_source = match language_kind {
        LanguageKind::Js => JS_KINDS_SRC,
        LanguageKind::Css => CSS_KINDS_SRC,
    };
    if kind_source.literals.contains(&kind_variant_name.as_str())
        || kind_source.tokens.contains(&kind_variant_name.as_str())
    {
        let ident = format_ident!("{}", kind_variant_name);
        quote! {  #ident }
    } else {
        // $ is valid syntax in rust and it's part of macros,
        // so we need to decorate the tokens with quotes
        if name == "$=" {
            let token = Literal::string(name);
            quote! { T![#token] }
        } else {
            let token: proc_macro2::TokenStream = name.parse().unwrap();
            quote! { T![#token] }
        }
    }
}
