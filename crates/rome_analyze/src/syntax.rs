use rome_rowan::{Language, SyntaxNode, WalkEvent};

use crate::{ControlFlow, QueryMatch, Visitor, VisitorContext};

/// The [SyntaxVisitor] is the simplest form of visitor implemented for the
/// analyzer, it simply broadcast each [WalkEvent::Enter] as a query match
/// event for the [SyntaxNode] being entered
pub struct SyntaxVisitor<L: Language, F> {
    has_suppressions: F,
    /// If a subtree is currently being skipped by the visitor, for instance
    /// because it has a suppression comment, this stores the root [SyntaxNode]
    /// of that subtree. The visitor will then ignore all events until it
    /// receives a [WalkEvent::Leave] for the `skip_subtree` node
    skip_subtree: Option<SyntaxNode<L>>,
}

impl<L: Language, F> SyntaxVisitor<L, F>
where
    F: Fn(&SyntaxNode<L>) -> bool,
{
    pub fn new(has_suppressions: F) -> Self {
        Self {
            has_suppressions,
            skip_subtree: None,
        }
    }
}

impl<L: Language, F, B> Visitor<B> for SyntaxVisitor<L, F>
where
    F: Fn(&SyntaxNode<L>) -> bool,
{
    type Language = L;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        ctx: &mut VisitorContext<L, B>,
    ) -> ControlFlow<B> {
        let node = match event {
            WalkEvent::Enter(node) => node,
            WalkEvent::Leave(node) => {
                if let Some(skip_subtree) = &self.skip_subtree {
                    if skip_subtree == node {
                        self.skip_subtree = None;
                    }
                }

                return ControlFlow::Continue(());
            }
        };

        if self.skip_subtree.is_some() {
            return ControlFlow::Continue(());
        }

        if let Some(range) = ctx.range {
            if node.text_range().ordering(range).is_ne() {
                self.skip_subtree = Some(node.clone());
                return ControlFlow::Continue(());
            }
        }

        // TODO: Checking for suppression comments is currently incomplete,
        // it can only completely suppress linting and has a high performance
        // cost due to eagerly looking up the first token of each node
        if (self.has_suppressions)(node) {
            self.skip_subtree = Some(node.clone());
            return ControlFlow::Continue(());
        }

        let query = QueryMatch::Syntax(node.clone());
        ctx.match_query(&query)
    }
}

#[cfg(test)]
mod tests {
    use rome_rowan::{
        raw_language::{RawLanguage, RawLanguageKind, RawLanguageRoot, RawSyntaxTreeBuilder},
        AstNode,
    };

    use crate::{Analyzer, ControlFlow, Never, QueryMatch, SyntaxVisitor, VisitorContext};

    /// Checks the syntax visitor emits a [QueryMatch] for each node in the syntax tree
    #[test]
    fn syntax_visitor() {
        let root = {
            let mut builder = RawSyntaxTreeBuilder::new();

            builder.start_node(RawLanguageKind::ROOT);
            builder.start_node(RawLanguageKind::EXPRESSION_LIST);

            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
            builder.finish_node();

            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token(RawLanguageKind::NUMBER_TOKEN, "2");
            builder.finish_node();

            builder.finish_node();
            builder.finish_node();

            RawLanguageRoot::unwrap_cast(builder.finish())
        };

        let mut analyzer = Analyzer::empty();
        analyzer.add_visitor(SyntaxVisitor::new(|_| false));

        let mut nodes = Vec::new();
        let mut ctx: VisitorContext<RawLanguage, Never> = VisitorContext {
            file_id: 0,
            root,
            range: None,
            match_query: Box::new(|_, _, query_match| match query_match {
                QueryMatch::Syntax(node) => {
                    nodes.push(node.kind());
                    ControlFlow::Continue(())
                }
                _ => panic!("unexpected QueryMatch variant"),
            }),
        };

        let result = analyzer.run(&mut ctx);
        assert!(result.is_none());

        drop(ctx);

        assert_eq!(
            nodes.as_slice(),
            &[
                RawLanguageKind::ROOT,
                RawLanguageKind::EXPRESSION_LIST,
                RawLanguageKind::LITERAL_EXPRESSION,
                RawLanguageKind::LITERAL_EXPRESSION
            ]
        );
    }
}
