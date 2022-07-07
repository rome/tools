//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_arguments;
mod no_label_var;
declare_group! { pub (crate) Js { name : "js" , rules : [no_arguments :: NoArguments , no_label_var :: NoLabelVar ,] } }
