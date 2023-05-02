//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_assign_in_expressions;
mod no_async_promise_executor;
mod no_comment_text;
mod no_compare_neg_zero;
mod no_confusing_labels;
mod no_const_enum;
mod no_debugger;
mod no_double_equals;
mod no_duplicate_case;
mod no_duplicate_class_members;
mod no_duplicate_object_keys;
mod no_empty_interface;
mod no_explicit_any;
mod no_extra_non_null_assertion;
mod no_prototype_builtins;
mod no_redundant_use_strict;
mod no_self_compare;
mod no_shadow_restricted_names;
mod no_sparse_array;
mod no_unsafe_negation;
mod use_default_switch_clause_last;
mod use_namespace_keyword;
mod use_valid_typeof;
declare_group! { pub (crate) Suspicious { name : "suspicious" , rules : [self :: no_assign_in_expressions :: NoAssignInExpressions , self :: no_async_promise_executor :: NoAsyncPromiseExecutor , self :: no_comment_text :: NoCommentText , self :: no_compare_neg_zero :: NoCompareNegZero , self :: no_confusing_labels :: NoConfusingLabels , self :: no_const_enum :: NoConstEnum , self :: no_debugger :: NoDebugger , self :: no_double_equals :: NoDoubleEquals , self :: no_duplicate_case :: NoDuplicateCase , self :: no_duplicate_class_members :: NoDuplicateClassMembers , self :: no_duplicate_object_keys :: NoDuplicateObjectKeys , self :: no_empty_interface :: NoEmptyInterface , self :: no_explicit_any :: NoExplicitAny , self :: no_extra_non_null_assertion :: NoExtraNonNullAssertion , self :: no_prototype_builtins :: NoPrototypeBuiltins , self :: no_redundant_use_strict :: NoRedundantUseStrict , self :: no_self_compare :: NoSelfCompare , self :: no_shadow_restricted_names :: NoShadowRestrictedNames , self :: no_sparse_array :: NoSparseArray , self :: no_unsafe_negation :: NoUnsafeNegation , self :: use_default_switch_clause_last :: UseDefaultSwitchClauseLast , self :: use_namespace_keyword :: UseNamespaceKeyword , self :: use_valid_typeof :: UseValidTypeof ,] } }
