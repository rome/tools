//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
pub(crate) mod no_children_prop;
pub(crate) mod no_const_assign;
pub(crate) mod no_global_object_calls;
pub(crate) mod no_render_return_value;
pub(crate) mod no_undeclared_variables;
pub(crate) mod no_unused_variables;
pub(crate) mod no_void_elements_with_children;
declare_group! { pub (crate) Correctness { name : "correctness" , rules : [self :: no_children_prop :: NoChildrenProp , self :: no_const_assign :: NoConstAssign , self :: no_global_object_calls :: NoGlobalObjectCalls , self :: no_render_return_value :: NoRenderReturnValue , self :: no_undeclared_variables :: NoUndeclaredVariables , self :: no_unused_variables :: NoUnusedVariables , self :: no_void_elements_with_children :: NoVoidElementsWithChildren ,] } }
