//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_dangerously_set_inner_html;
mod no_render_return_value;
mod no_unused_variables;
mod use_button_type;
mod use_camel_case;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_dangerously_set_inner_html :: NoDangerouslySetInnerHtml , self :: no_render_return_value :: NoRenderReturnValue , self :: no_unused_variables :: NoUnusedVariables , self :: use_button_type :: UseButtonType , self :: use_camel_case :: UseCamelCase ,] } }
