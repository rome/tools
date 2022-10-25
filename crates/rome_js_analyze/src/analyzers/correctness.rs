//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_async_promise_executor;
mod no_comment_text;
mod no_compare_neg_zero;
mod no_debugger;
mod no_delete;
mod no_double_equals;
mod no_empty_pattern;
mod no_implicit_boolean;
mod no_multiple_spaces_in_regular_expression_literals;
mod no_new_symbol;
mod no_shadow_restricted_names;
mod no_sparse_array;
mod no_unnecessary_continue;
mod no_unreachable;
mod no_unsafe_negation;
mod no_unused_template_literal;
mod use_block_statements;
mod use_optional_chain;
mod use_single_case_statement;
mod use_single_var_declarator;
mod use_valid_typeof;
mod use_while;
declare_group! { pub (crate) Correctness { name : "correctness" , rules : [self :: no_async_promise_executor :: NoAsyncPromiseExecutor , self :: no_comment_text :: NoCommentText , self :: no_compare_neg_zero :: NoCompareNegZero , self :: no_debugger :: NoDebugger , self :: no_delete :: NoDelete , self :: no_double_equals :: NoDoubleEquals , self :: no_empty_pattern :: NoEmptyPattern , self :: no_implicit_boolean :: NoImplicitBoolean , self :: no_multiple_spaces_in_regular_expression_literals :: NoMultipleSpacesInRegularExpressionLiterals , self :: no_new_symbol :: NoNewSymbol , self :: no_shadow_restricted_names :: NoShadowRestrictedNames , self :: no_sparse_array :: NoSparseArray , self :: no_unnecessary_continue :: NoUnnecessaryContinue , self :: no_unreachable :: NoUnreachable , self :: no_unsafe_negation :: NoUnsafeNegation , self :: no_unused_template_literal :: NoUnusedTemplateLiteral , self :: use_block_statements :: UseBlockStatements , self :: use_optional_chain :: UseOptionalChain , self :: use_single_case_statement :: UseSingleCaseStatement , self :: use_single_var_declarator :: UseSingleVarDeclarator , self :: use_valid_typeof :: UseValidTypeof , self :: use_while :: UseWhile ,] } }
