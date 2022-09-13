//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_unreachable;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_unreachable :: NoUnreachable ,] } }
