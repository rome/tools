//! Generated file, do not edit by hand, see `xtask/codegen`

mod a11y;
mod correctness;
mod nursery;
mod react;
mod style;
::rome_analyze::declare_category! { pub (crate) SemanticAnalyzers { kind : Lint , groups : [self :: a11y :: A11y , self :: correctness :: Correctness , self :: nursery :: Nursery , self :: react :: React , self :: style :: Style ,] } }
