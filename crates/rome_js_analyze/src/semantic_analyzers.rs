//! Generated file, do not edit by hand, see `xtask/codegen`

mod correctness;
mod nursery;
mod style;
::rome_analyze::declare_category! { pub (crate) SemanticAnalyzers { kind : Lint , groups : [self :: correctness :: Correctness , self :: nursery :: Nursery , self :: style :: Style ,] } }
