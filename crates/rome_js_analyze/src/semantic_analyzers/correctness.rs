//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_arguments;
mod no_catch_assign;
mod no_dupe_args;
mod no_function_assign;
mod no_import_assign;
mod no_label_var;
declare_group! { pub (crate) Correctness { name : "correctness" , rules : [self :: no_arguments :: NoArguments , self :: no_catch_assign :: NoCatchAssign , self :: no_dupe_args :: NoDupeArgs , self :: no_function_assign :: NoFunctionAssign , self :: no_import_assign :: NoImportAssign , self :: no_label_var :: NoLabelVar ,] } }
