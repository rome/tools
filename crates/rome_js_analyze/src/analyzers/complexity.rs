//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;

pub(crate) mod no_extra_boolean_cast;
pub(crate) mod no_multiple_spaces_in_regular_expression_literals;
pub(crate) mod no_useless_catch;
pub(crate) mod no_useless_constructor;
pub(crate) mod no_useless_label;
pub(crate) mod no_useless_rename;
pub(crate) mod no_useless_switch_case;
pub(crate) mod no_useless_type_constraint;
pub(crate) mod no_with;
pub(crate) mod use_flat_map;
pub(crate) mod use_optional_chain;
pub(crate) mod use_simplified_logic_expression;

declare_group! {
    pub (crate) Complexity {
        name : "complexity" ,
        rules : [
            self :: no_extra_boolean_cast :: NoExtraBooleanCast ,
            self :: no_multiple_spaces_in_regular_expression_literals :: NoMultipleSpacesInRegularExpressionLiterals ,
            self :: no_useless_catch :: NoUselessCatch ,
            self :: no_useless_constructor :: NoUselessConstructor ,
            self :: no_useless_label :: NoUselessLabel ,
            self :: no_useless_rename :: NoUselessRename ,
            self :: no_useless_switch_case :: NoUselessSwitchCase ,
            self :: no_useless_type_constraint :: NoUselessTypeConstraint ,
            self :: no_with :: NoWith ,
            self :: use_flat_map :: UseFlatMap ,
            self :: use_optional_chain :: UseOptionalChain ,
            self :: use_simplified_logic_expression :: UseSimplifiedLogicExpression ,
        ]
     }
}
