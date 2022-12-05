//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_async_promise_executor;
mod no_comment_text;
mod no_compare_neg_zero;
mod no_debugger;
mod no_double_equals;
mod no_explicit_any;
mod no_shadow_restricted_names;
mod no_sparse_array;
mod no_unsafe_negation;
mod use_valid_typeof;
declare_group! { pub (crate) Suspicious { name : "suspicious" , rules : [self :: no_async_promise_executor :: NoAsyncPromiseExecutor , self :: no_comment_text :: NoCommentText , self :: no_compare_neg_zero :: NoCompareNegZero , self :: no_debugger :: NoDebugger , self :: no_double_equals :: NoDoubleEquals , self :: no_explicit_any :: NoExplicitAny , self :: no_shadow_restricted_names :: NoShadowRestrictedNames , self :: no_sparse_array :: NoSparseArray , self :: no_unsafe_negation :: NoUnsafeNegation , self :: use_valid_typeof :: UseValidTypeof ,] } }
