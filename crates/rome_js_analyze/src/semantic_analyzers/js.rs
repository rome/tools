//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_arguments;
mod no_catch_assign;
mod no_function_assign;
mod no_import_assign;
mod no_label_var;
mod no_shouty_constants;
mod use_camel_case;
declare_group! { pub (crate) Js { name : "js" , rules : [no_arguments :: NoArguments , no_catch_assign :: NoCatchAssign , no_function_assign :: NoFunctionAssign , no_label_var :: NoLabelVar , no_shouty_constants :: NoShoutyConstants , use_camel_case :: UseCamelCase ,] } }
