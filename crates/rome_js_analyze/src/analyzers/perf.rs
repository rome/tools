//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_delete;
declare_group! { pub (crate) Perf { name : "perf" , rules : [self :: no_delete :: NoDelete ,] } }
