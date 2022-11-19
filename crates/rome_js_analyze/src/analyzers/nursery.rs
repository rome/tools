//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_access_key;
mod no_banned_types;
mod no_conditional_assignment;
mod no_constructor_return;
mod no_distracting_elements;
mod no_dupe_keys;
mod no_empty_interface;
mod no_explicit_any;
mod no_extra_non_null_assertion;
mod no_header_scope;
mod no_invalid_constructor_super;
mod no_precision_loss;
mod no_setter_return;
mod no_string_case_mismatch;
mod no_unsafe_finally;
mod use_flat_map;
mod use_numeric_literals;
mod use_valid_for_direction;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_access_key :: NoAccessKey , self :: no_banned_types :: NoBannedTypes , self :: no_conditional_assignment :: NoConditionalAssignment , self :: no_constructor_return :: NoConstructorReturn , self :: no_distracting_elements :: NoDistractingElements , self :: no_dupe_keys :: NoDupeKeys , self :: no_empty_interface :: NoEmptyInterface , self :: no_explicit_any :: NoExplicitAny , self :: no_extra_non_null_assertion :: NoExtraNonNullAssertion , self :: no_header_scope :: NoHeaderScope , self :: no_invalid_constructor_super :: NoInvalidConstructorSuper , self :: no_precision_loss :: NoPrecisionLoss , self :: no_setter_return :: NoSetterReturn , self :: no_string_case_mismatch :: NoStringCaseMismatch , self :: no_unsafe_finally :: NoUnsafeFinally , self :: use_flat_map :: UseFlatMap , self :: use_numeric_literals :: UseNumericLiterals , self :: use_valid_for_direction :: UseValidForDirection ,] } }
