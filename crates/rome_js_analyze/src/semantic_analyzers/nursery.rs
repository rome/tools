//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_array_index_key;
mod no_children_prop;
mod no_dangerously_set_inner_html;
mod no_render_return_value;
mod no_unused_variables;
mod no_useless_fragments;
mod no_void_elements_with_children;
mod use_button_type;
mod use_camel_case;
mod use_fragment_syntax;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_array_index_key :: NoArrayIndexKey , self :: no_children_prop :: NoChildrenProp , self :: no_dangerously_set_inner_html :: NoDangerouslySetInnerHtml , self :: no_render_return_value :: NoRenderReturnValue , self :: no_unused_variables :: NoUnusedVariables , self :: no_useless_fragments :: NoUselessFragments , self :: no_void_elements_with_children :: NoVoidElementsWithChildren , self :: use_button_type :: UseButtonType , self :: use_camel_case :: UseCamelCase , self :: use_fragment_syntax :: UseFragmentSyntax ,] } }
