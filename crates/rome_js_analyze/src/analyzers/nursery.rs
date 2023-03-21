//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_assign_in_expressions;
mod no_banned_types;
mod no_comma_operator;
mod no_confusing_arrow;
mod no_confusing_labels;
mod no_duplicate_case;
mod no_duplicate_class_members;
mod no_duplicate_jsx_props;
mod no_extra_labels;
mod no_extra_semicolons;
mod no_global_object_calls;
mod no_inferrable_types;
mod no_inner_declarations;
mod no_invalid_constructor_super;
mod no_namespace;
mod no_parameter_properties;
mod no_prototype_builtins;
mod no_redundant_alt;
mod no_self_assign;
mod no_self_compare;
mod no_svg_without_title;
mod no_switch_declarations;
mod no_unreachable_super;
mod no_unsafe_optional_chaining;
mod no_unused_labels;
mod no_useless_catch;
mod no_useless_rename;
mod no_useless_switch_case;
mod no_with;
mod use_is_nan;
mod use_media_caption;
mod use_namespace_keyword;
mod use_yield;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_assign_in_expressions :: NoAssignInExpressions , self :: no_banned_types :: NoBannedTypes , self :: no_comma_operator :: NoCommaOperator , self :: no_confusing_arrow :: NoConfusingArrow , self :: no_confusing_labels :: NoConfusingLabels , self :: no_duplicate_case :: NoDuplicateCase , self :: no_duplicate_class_members :: NoDuplicateClassMembers , self :: no_duplicate_jsx_props :: NoDuplicateJsxProps , self :: no_extra_labels :: NoExtraLabels , self :: no_extra_semicolons :: NoExtraSemicolons , self :: no_global_object_calls :: NoGlobalObjectCalls , self :: no_inferrable_types :: NoInferrableTypes , self :: no_inner_declarations :: NoInnerDeclarations , self :: no_invalid_constructor_super :: NoInvalidConstructorSuper , self :: no_namespace :: NoNamespace , self :: no_parameter_properties :: NoParameterProperties , self :: no_prototype_builtins :: NoPrototypeBuiltins , self :: no_redundant_alt :: NoRedundantAlt , self :: no_self_assign :: NoSelfAssign , self :: no_self_compare :: NoSelfCompare , self :: no_svg_without_title :: NoSvgWithoutTitle , self :: no_switch_declarations :: NoSwitchDeclarations , self :: no_unreachable_super :: NoUnreachableSuper , self :: no_unsafe_optional_chaining :: NoUnsafeOptionalChaining , self :: no_unused_labels :: NoUnusedLabels , self :: no_useless_catch :: NoUselessCatch , self :: no_useless_rename :: NoUselessRename , self :: no_useless_switch_case :: NoUselessSwitchCase , self :: no_with :: NoWith , self :: use_is_nan :: UseIsNan , self :: use_media_caption :: UseMediaCaption , self :: use_namespace_keyword :: UseNamespaceKeyword , self :: use_yield :: UseYield ,] } }
