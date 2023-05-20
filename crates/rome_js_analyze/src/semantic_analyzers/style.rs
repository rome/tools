//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_arguments;
mod no_parameter_assign;
mod no_restricted_globals;
mod no_shouty_constants;
mod no_var;
mod use_const;
mod use_fragment_syntax;
declare_group! { pub (crate) Style { name : "style" , rules : [self :: no_arguments :: NoArguments , self :: no_parameter_assign :: NoParameterAssign , self :: no_restricted_globals :: NoRestrictedGlobals , self :: no_shouty_constants :: NoShoutyConstants , self :: no_var :: NoVar , self :: use_const :: UseConst , self :: use_fragment_syntax :: UseFragmentSyntax ,] } }
