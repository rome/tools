use crate::generated::CssSyntaxFactory;
use rome_css_syntax::CssLanguage;
use rome_rowan::TreeBuilder;

mod generated;

// Re-exported for tests
#[doc(hidden)]
pub use rome_css_syntax as syntax;

pub type CssSyntaxTreeBuilder = TreeBuilder<'static, CssLanguage, CssSyntaxFactory>;
