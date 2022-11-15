//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_banned_types;
mod no_cond_assign;
mod no_dupe_keys;
mod no_explicit_any;
mod no_invalid_constructor_super;
mod use_flat_map;
mod use_numeric_literals;
mod use_valid_for_direction;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_banned_types :: NoBannedTypes , self :: no_cond_assign :: NoCondAssign , self :: no_dupe_keys :: NoDupeKeys , self :: no_explicit_any :: NoExplicitAny , self :: no_invalid_constructor_super :: NoInvalidConstructorSuper , self :: use_flat_map :: UseFlatMap , self :: use_numeric_literals :: UseNumericLiterals , self :: use_valid_for_direction :: UseValidForDirection ,] } }
