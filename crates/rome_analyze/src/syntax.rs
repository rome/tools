use rome_rowan::{Language, SyntaxNode, WalkEvent};

use crate::{QueryMatch, Visitor, VisitorContext};

#[derive(Default)]
/// The [SyntaxVisitor] is the simplest form of visitor implemented for the
/// analyzer, it simply broadcast each [WalkEvent::Enter] as a query match
/// event for the [SyntaxNode] being entered
pub struct SyntaxVisitor<L: Language> {
    /// If a subtree is currently being skipped by the visitor, for instance
    /// because it has a suppression comment, this stores the root [SyntaxNode]
    /// of that subtree. The visitor will then ignore all events until it
    /// receives a [WalkEvent::Leave] for the `skip_subtree` node
    skip_subtree: Option<SyntaxNode<L>>,
}

impl<L: Language> Visitor for SyntaxVisitor<L> {
    type Language = L;

    fn visit(&mut self, event: &WalkEvent<SyntaxNode<Self::Language>>, mut ctx: VisitorContext<L>) {
        let node = match event {
            WalkEvent::Enter(node) => node,
            WalkEvent::Leave(node) => {
                if let Some(skip_subtree) = &self.skip_subtree {
                    if skip_subtree == node {
                        self.skip_subtree = None;
                    }
                }

                return;
            }
        };

        if self.skip_subtree.is_some() {
            return;
        }

        if let Some(range) = ctx.range {
            if node.text_range().ordering(range).is_ne() {
                self.skip_subtree = Some(node.clone());
                return;
            }
        }

        ctx.match_query(QueryMatch::Syntax(node.clone()));
    }
}

#[cfg(test)]
mod tests {

    use rome_diagnostics::location::FileId;
    use rome_rowan::{
        raw_language::{RawLanguage, RawLanguageKind, RawLanguageRoot, RawSyntaxTreeBuilder},
        AstNode,
    };

    use crate::{
        matcher::MatchQueryParams, registry::Phases, Analyzer, AnalyzerContext, AnalyzerOptions,
        AnalyzerSignal, ControlFlow, MetadataRegistry, Never, QueryMatch, QueryMatcher, ServiceBag,
        SyntaxVisitor,
    };

    #[derive(Default)]
    struct BufferMatcher {
        nodes: Vec<RawLanguageKind>,
    }

    impl<'a> QueryMatcher<RawLanguage> for &'a mut BufferMatcher {
        fn match_query(&mut self, params: MatchQueryParams<RawLanguage>) {
            match params.query {
                QueryMatch::Syntax(node) => {
                    self.nodes.push(node.kind());
                }
                _ => unreachable!(),
            }
        }
    }

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

        let mut matcher = BufferMatcher::default();
        let mut emit_signal =
            |_: &dyn AnalyzerSignal<RawLanguage>| -> ControlFlow<Never> { unreachable!() };

        let metadata = MetadataRegistry::default();

        let mut analyzer = Analyzer::new(
            &metadata,
            &mut matcher,
            |_| unreachable!(),
            |_, _, _| unreachable!(),
            &mut emit_signal,
        );

        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default());

        let ctx: AnalyzerContext<RawLanguage> = AnalyzerContext {
            file_id: FileId::zero(),
            root,
            range: None,
            services: ServiceBag::default(),
            options: &AnalyzerOptions::default(),
        };

        let result: Option<Never> = analyzer.run(ctx);
        assert!(result.is_none());

        assert_eq!(
            matcher.nodes.as_slice(),
            &[
                RawLanguageKind::ROOT,
                RawLanguageKind::EXPRESSION_LIST,
                RawLanguageKind::LITERAL_EXPRESSION,
                RawLanguageKind::LITERAL_EXPRESSION
            ]
        );
    }
}
