//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_constructor_return;
mod no_duplicate_object_keys;
mod no_empty_pattern;
mod no_new_symbol;
mod no_precision_loss;
mod no_string_case_mismatch;
mod no_unnecessary_continue;
mod no_unreachable;
mod no_unsafe_finally;
mod no_void_type_return;
mod use_enum_initializers;
mod use_valid_for_direction;
declare_group! { pub (crate) Correctness { name : "correctness" , rules : [self :: no_constructor_return :: NoConstructorReturn , self :: no_duplicate_object_keys :: NoDuplicateObjectKeys , self :: no_empty_pattern :: NoEmptyPattern , self :: no_new_symbol :: NoNewSymbol , self :: no_precision_loss :: NoPrecisionLoss , self :: no_string_case_mismatch :: NoStringCaseMismatch , self :: no_unnecessary_continue :: NoUnnecessaryContinue , self :: no_unreachable :: NoUnreachable , self :: no_unsafe_finally :: NoUnsafeFinally , self :: no_void_type_return :: NoVoidTypeReturn , self :: use_enum_initializers :: UseEnumInitializers , self :: use_valid_for_direction :: UseValidForDirection ,] } }
