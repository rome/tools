pub use crate::generated::JsonSyntaxFactory;
use rome_json_syntax::JsonLanguage;
use rome_rowan::TreeBuilder;

mod generated;

// Re-exported for tests
#[doc(hidden)]
pub use rome_json_syntax as syntax;

pub type JsonSyntaxTreeBuilder = TreeBuilder<'static, JsonLanguage, JsonSyntaxFactory>;

pub use generated::node_factory as make;
