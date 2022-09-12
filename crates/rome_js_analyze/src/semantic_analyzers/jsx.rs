//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_danger;
declare_group! { pub (crate) Jsx { name : "jsx" , rules : [self :: no_danger :: NoDanger ,] } }
