//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod flip_bin_exp;
declare_group! { pub (crate) Js { name : "js" , rules : [self :: flip_bin_exp :: FlipBinExp ,] } }
