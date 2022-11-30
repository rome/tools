//! Generated file, do not edit by hand, see `xtask/codegen`

mod a11y;
mod complexity;
mod correctness;
mod nursery;
mod perf;
mod style;
mod suspicious;
::rome_analyze::declare_category! { pub (crate) Analyzers { kind : Lint , groups : [self :: a11y :: A11y , self :: complexity :: Complexity , self :: correctness :: Correctness , self :: nursery :: Nursery , self :: perf :: Perf , self :: style :: Style , self :: suspicious :: Suspicious ,] } }
