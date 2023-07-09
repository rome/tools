//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;

pub(crate) mod no_duplicate_private_class_members;
pub(crate) mod no_super_without_extends;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_duplicate_private_class_members :: NoDuplicatePrivateClassMembers ,
            self :: no_super_without_extends :: NoSuperWithoutExtends ,
        ]
     }
}
