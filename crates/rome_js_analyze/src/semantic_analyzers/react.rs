//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_array_index_key;
mod no_children_prop;
mod no_dangerously_set_inner_html;
mod no_dangerously_set_inner_html_with_children;
mod no_render_return_value;
mod no_void_elements_with_children;
declare_group! { pub (crate) React { name : "react" , rules : [self :: no_array_index_key :: NoArrayIndexKey , self :: no_children_prop :: NoChildrenProp , self :: no_dangerously_set_inner_html :: NoDangerouslySetInnerHtml , self :: no_dangerously_set_inner_html_with_children :: NoDangerouslySetInnerHtmlWithChildren , self :: no_render_return_value :: NoRenderReturnValue , self :: no_void_elements_with_children :: NoVoidElementsWithChildren ,] } }
