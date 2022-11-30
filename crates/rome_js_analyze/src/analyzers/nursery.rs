//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_access_key;
mod no_banned_types;
mod no_conditional_assignment;
mod no_const_enum;
mod no_constructor_return;
mod no_distracting_elements;
mod no_dupe_keys;
mod no_empty_interface;
mod no_extra_non_null_assertion;
mod no_header_scope;
mod no_invalid_constructor_super;
mod no_non_null_assertion;
mod no_precision_loss;
mod no_redundant_use_strict;
mod no_setter_return;
mod no_string_case_mismatch;
mod no_unsafe_finally;
mod no_void_type_return;
mod use_default_parameter_last;
mod use_default_switch_clause_last;
mod use_enum_initializers;
mod use_exponentiation_operator;
mod use_numeric_literals;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_access_key :: NoAccessKey , self :: no_banned_types :: NoBannedTypes , self :: no_conditional_assignment :: NoConditionalAssignment , self :: no_const_enum :: NoConstEnum , self :: no_constructor_return :: NoConstructorReturn , self :: no_distracting_elements :: NoDistractingElements , self :: no_dupe_keys :: NoDupeKeys , self :: no_empty_interface :: NoEmptyInterface , self :: no_extra_non_null_assertion :: NoExtraNonNullAssertion , self :: no_header_scope :: NoHeaderScope , self :: no_invalid_constructor_super :: NoInvalidConstructorSuper , self :: no_non_null_assertion :: NoNonNullAssertion , self :: no_precision_loss :: NoPrecisionLoss , self :: no_redundant_use_strict :: NoRedundantUseStrict , self :: no_setter_return :: NoSetterReturn , self :: no_string_case_mismatch :: NoStringCaseMismatch , self :: no_unsafe_finally :: NoUnsafeFinally , self :: no_void_type_return :: NoVoidTypeReturn , self :: use_default_parameter_last :: UseDefaultParameterLast , self :: use_default_switch_clause_last :: UseDefaultSwitchClauseLast , self :: use_enum_initializers :: UseEnumInitializers , self :: use_exponentiation_operator :: UseExponentiationOperator , self :: use_numeric_literals :: UseNumericLiterals ,] } }
