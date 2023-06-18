//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
pub(crate) mod no_duplicate_keys;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_duplicate_keys :: NoDuplicateKeys ,] } }
