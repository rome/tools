//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_children_prop;
mod no_const_assign;
mod no_render_return_value;
mod no_undeclared_variables;
mod no_unused_variables;
mod no_void_elements_with_children;
declare_group! { pub (crate) Correctness { name : "correctness" , rules : [self :: no_children_prop :: NoChildrenProp , self :: no_const_assign :: NoConstAssign , self :: no_render_return_value :: NoRenderReturnValue , self :: no_undeclared_variables :: NoUndeclaredVariables , self :: no_unused_variables :: NoUnusedVariables , self :: no_void_elements_with_children :: NoVoidElementsWithChildren ,] } }
