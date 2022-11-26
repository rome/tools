use crate::prelude::*;
use crate::AsFormat;
use rome_js_syntax::JsLanguage;
use rome_rowan::AstNode;

/// Formats a node using its [`AsFormat`] implementation but falls back to printing the node as
/// it is in the source document if the formatting returns an [`FormatError`].
pub(crate) const fn format_or_verbatim<Node>(node: &Node) -> FormatNodeOrVerbatim<Node>
where
    Node: AstNode<Language = JsLanguage> + AsFormat<JsFormatContext>,
{
    FormatNodeOrVerbatim { node }
}

/// Formats a node or falls back to verbatim printing if formating this node fails.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FormatNodeOrVerbatim<'a, Node> {
    node: &'a Node,
}

impl<'a, Node> Format<JsFormatContext> for FormatNodeOrVerbatim<'a, Node>
where
    Node: AstNode<Language = JsLanguage> + AsFormat<JsFormatContext>,
{
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let snapshot = Formatter::state_snapshot(f);

        match self.node.format().fmt(f) {
            Ok(result) => Ok(result),

            Err(FormatError::SyntaxError) => {
                f.restore_state_snapshot(snapshot);

                // Lists that yield errors are formatted as they were suppressed nodes.
                // Doing so, the formatter formats the nodes/tokens as is.
                format_suppressed_node(self.node.syntax()).fmt(f)
            }
            Err(err) => Err(err),
        }
    }
}
