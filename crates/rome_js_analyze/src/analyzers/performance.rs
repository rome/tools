//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;

pub(crate) mod no_delete;

declare_group! {
    pub (crate) Performance {
        name : "performance" ,
        rules : [
            self :: no_delete :: NoDelete ,
        ]
     }
}
