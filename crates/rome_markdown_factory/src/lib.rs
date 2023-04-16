pub use crate::generated::MdSyntaxFactory;
use rome_rowan::TreeBuilder;
mod generated;

// Re-exported for tests
#[doc(hidden)]
pub use rome_markdown_syntax as syntax;

pub type MdSyntaxTreeBuilder = TreeBuilder<'static, MdLanguage, MdSyntaxFactory>;

pub use generated::node_factory as make;
use rome_markdown_syntax::MdLanguage;
