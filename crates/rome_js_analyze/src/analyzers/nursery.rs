//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_banned_types;
mod no_conditional_assignment;
mod no_dupe_keys;
mod no_empty_interface;
mod no_explicit_any;
mod no_invalid_constructor_super;
mod no_precision_loss;
mod no_unsafe_finally;
mod use_flat_map;
mod use_numeric_literals;
mod use_valid_for_direction;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_banned_types :: NoBannedTypes , self :: no_conditional_assignment :: NoConditionalAssignment , self :: no_dupe_keys :: NoDupeKeys , self :: no_empty_interface :: NoEmptyInterface , self :: no_explicit_any :: NoExplicitAny , self :: no_invalid_constructor_super :: NoInvalidConstructorSuper , self :: no_precision_loss :: NoPrecisionLoss , self :: no_unsafe_finally :: NoUnsafeFinally , self :: use_flat_map :: UseFlatMap , self :: use_numeric_literals :: UseNumericLiterals , self :: use_valid_for_direction :: UseValidForDirection ,] } }
