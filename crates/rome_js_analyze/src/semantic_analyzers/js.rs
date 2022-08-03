//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_arguments;
mod no_catch_assign;
mod no_function_assign;
mod no_import_assign;
mod no_label_var;
mod no_shouty_constants;
mod no_unused_variables;
mod use_camel_case;
declare_group! { pub (crate) Js { name : "js" , rules : [self :: no_arguments :: NoArguments , self :: no_catch_assign :: NoCatchAssign , self :: no_function_assign :: NoFunctionAssign , self :: no_import_assign :: NoImportAssign , self :: no_label_var :: NoLabelVar , self :: no_shouty_constants :: NoShoutyConstants , self :: no_unused_variables :: NoUnusedVariables , self :: use_camel_case :: UseCamelCase ,] } }
