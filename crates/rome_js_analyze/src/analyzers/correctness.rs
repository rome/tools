//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_constructor_return;
mod no_empty_pattern;
mod no_inner_declarations;
mod no_invalid_constructor_super;
mod no_new_symbol;
mod no_precision_loss;
mod no_setter_return;
mod no_string_case_mismatch;
mod no_switch_declarations;
mod no_unnecessary_continue;
mod no_unreachable;
mod no_unreachable_super;
mod no_unsafe_finally;
mod no_unsafe_optional_chaining;
mod no_unused_labels;
mod no_void_type_return;
mod use_valid_for_direction;
mod use_yield;
declare_group! { pub (crate) Correctness { name : "correctness" , rules : [self :: no_constructor_return :: NoConstructorReturn , self :: no_empty_pattern :: NoEmptyPattern , self :: no_inner_declarations :: NoInnerDeclarations , self :: no_invalid_constructor_super :: NoInvalidConstructorSuper , self :: no_new_symbol :: NoNewSymbol , self :: no_precision_loss :: NoPrecisionLoss , self :: no_setter_return :: NoSetterReturn , self :: no_string_case_mismatch :: NoStringCaseMismatch , self :: no_switch_declarations :: NoSwitchDeclarations , self :: no_unnecessary_continue :: NoUnnecessaryContinue , self :: no_unreachable :: NoUnreachable , self :: no_unreachable_super :: NoUnreachableSuper , self :: no_unsafe_finally :: NoUnsafeFinally , self :: no_unsafe_optional_chaining :: NoUnsafeOptionalChaining , self :: no_unused_labels :: NoUnusedLabels , self :: no_void_type_return :: NoVoidTypeReturn , self :: use_valid_for_direction :: UseValidForDirection , self :: use_yield :: UseYield ,] } }
