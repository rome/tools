//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_async_promise_executor;
mod no_compare_neg_zero;
mod no_dead_code;
mod no_debugger;
mod no_delete;
mod no_double_equals;
mod no_empty_pattern;
mod no_extra_boolean_cast;
mod no_negation_else;
mod no_shadow_restricted_names;
mod no_sparse_array;
mod no_unnecessary_continue;
mod no_unsafe_negation;
mod no_unused_template_literal;
mod use_block_statements;
mod use_simplified_logic_expression;
mod use_single_case_statement;
mod use_single_var_declarator;
mod use_template;
mod use_valid_typeof;
mod use_while;
declare_group! { pub (crate) Js { name : "js" , rules : [no_async_promise_executor :: NoAsyncPromiseExecutor , no_compare_neg_zero :: NoCompareNegZero , no_dead_code :: NoDeadCode , no_debugger :: NoDebugger , no_delete :: NoDelete , no_double_equals :: NoDoubleEquals , no_empty_pattern :: NoEmptyPattern , no_extra_boolean_cast :: NoExtraBooleanCast , no_negation_else :: NoNegationElse , no_sparse_array :: NoSparseArray , no_unnecessary_continue :: NoUnnecessaryContinue , no_unsafe_negation :: NoUnsafeNegation , no_unused_template_literal :: NoUnusedTemplateLiteral , use_block_statements :: UseBlockStatements , use_simplified_logic_expression :: UseSimplifiedLogicExpression , use_single_case_statement :: UseSingleCaseStatement , use_single_var_declarator :: UseSingleVarDeclarator , use_template :: UseTemplate , use_valid_typeof :: UseValidTypeof , use_while :: UseWhile ,] } }
