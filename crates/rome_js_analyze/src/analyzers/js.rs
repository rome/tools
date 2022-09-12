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
mod no_new_symbol;
mod no_shadow_restricted_names;
mod no_sparse_array;
mod no_unnecessary_continue;
mod no_unsafe_negation;
mod no_unused_template_literal;
mod use_block_statements;
mod use_optional_chain;
mod use_simplified_logic_expression;
mod use_single_case_statement;
mod use_single_var_declarator;
mod use_template;
mod use_valid_typeof;
mod use_while;
declare_group! { pub (crate) Js { name : "js" , rules : [self :: no_async_promise_executor :: NoAsyncPromiseExecutor , self :: no_compare_neg_zero :: NoCompareNegZero , self :: no_dead_code :: NoDeadCode , self :: no_debugger :: NoDebugger , self :: no_delete :: NoDelete , self :: no_double_equals :: NoDoubleEquals , self :: no_empty_pattern :: NoEmptyPattern , self :: no_extra_boolean_cast :: NoExtraBooleanCast , self :: no_negation_else :: NoNegationElse , self :: no_new_symbol :: NoNewSymbol , self :: no_shadow_restricted_names :: NoShadowRestrictedNames , self :: no_sparse_array :: NoSparseArray , self :: no_unnecessary_continue :: NoUnnecessaryContinue , self :: no_unsafe_negation :: NoUnsafeNegation , self :: no_unused_template_literal :: NoUnusedTemplateLiteral , self :: use_block_statements :: UseBlockStatements , self :: use_optional_chain :: UseOptionalChain , self :: use_simplified_logic_expression :: UseSimplifiedLogicExpression , self :: use_single_case_statement :: UseSingleCaseStatement , self :: use_single_var_declarator :: UseSingleVarDeclarator , self :: use_template :: UseTemplate , self :: use_valid_typeof :: UseValidTypeof , self :: use_while :: UseWhile ,] } }
