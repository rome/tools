//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_debugger;
declare_group! { pub (crate) Security { name : "security" , rules : [self :: no_debugger :: NoDebugger ,] } }
