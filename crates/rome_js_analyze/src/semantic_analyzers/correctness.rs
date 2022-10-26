//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_arguments;
mod no_array_index_key;
mod no_catch_assign;
mod no_children_prop;
mod no_dupe_args;
mod no_function_assign;
mod no_import_assign;
mod no_label_var;
mod no_render_return_value;
mod no_restricted_globals;
mod no_undeclared_variables;
mod no_unused_variables;
mod no_useless_fragments;
mod no_void_elements_with_children;
declare_group! { pub (crate) Correctness { name : "correctness" , rules : [self :: no_arguments :: NoArguments , self :: no_array_index_key :: NoArrayIndexKey , self :: no_catch_assign :: NoCatchAssign , self :: no_children_prop :: NoChildrenProp , self :: no_dupe_args :: NoDupeArgs , self :: no_function_assign :: NoFunctionAssign , self :: no_import_assign :: NoImportAssign , self :: no_label_var :: NoLabelVar , self :: no_render_return_value :: NoRenderReturnValue , self :: no_restricted_globals :: NoRestrictedGlobals , self :: no_undeclared_variables :: NoUndeclaredVariables , self :: no_unused_variables :: NoUnusedVariables , self :: no_useless_fragments :: NoUselessFragments , self :: no_void_elements_with_children :: NoVoidElementsWithChildren ,] } }
