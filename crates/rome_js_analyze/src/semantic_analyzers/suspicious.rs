//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_array_index_key;
mod no_catch_assign;
mod no_dupe_args;
mod no_function_assign;
mod no_import_assign;
mod no_label_var;
declare_group! { pub (crate) Suspicious { name : "suspicious" , rules : [self :: no_array_index_key :: NoArrayIndexKey , self :: no_catch_assign :: NoCatchAssign , self :: no_dupe_args :: NoDupeArgs , self :: no_function_assign :: NoFunctionAssign , self :: no_import_assign :: NoImportAssign , self :: no_label_var :: NoLabelVar ,] } }
