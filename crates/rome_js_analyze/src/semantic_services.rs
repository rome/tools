use std::mem::swap;

use rome_analyze::{
    AnalyzerContext, CannotCreateServicesError, Phase, Phases, QueryKey, QueryMatch, Queryable,
    ServiceBag, Visitor, VisitorContext,
};
use rome_js_semantic::{SemanticEventExtractor, SemanticModel, SemanticModelBuilder};
use rome_js_syntax::{JsAnyRoot, JsLanguage, WalkEvent};
use rome_rowan::{AstNode, SyntaxNode};

pub struct SemanticServices {
    model: SemanticModel,
}

impl SemanticServices {
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }
}

impl TryFrom<ServiceBag> for SemanticServices {
    type Error = CannotCreateServicesError;

    fn try_from(services: ServiceBag) -> Result<Self, Self::Error> {
        let model = services
            .get_service()
            .ok_or(CannotCreateServicesError::MissingServices(&[
                "SemanticModel",
            ]))?;
        Ok(Self { model })
    }
}

impl Phase for SemanticServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Query type usable by lint rules **that uses the semantic model** to match on specific [AstNode] types
#[derive(Clone)]
pub struct Semantic<N>(pub N);

impl<N> Queryable for Semantic<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Output = N;
    type Language = JsLanguage;
    type Services = SemanticServices;

    /// Match on [QueryMatch::Syntax] if the kind of the syntax node matches
    /// the kind set of `N`
    const KEY: QueryKey<Self::Language> = QueryKey::Syntax(N::KIND_SET);

    fn unwrap_match(query: &QueryMatch<Self::Language>) -> Self::Output {
        match query {
            QueryMatch::Syntax(node) => N::unwrap_cast(node.clone()),
            _ => panic!("tried to unwrap unsupported QueryMatch kind, expected Syntax"),
        }
    }
}

pub(crate) struct SemanticVisitor {
    extractor: SemanticEventExtractor,
    builder: SemanticModelBuilder,
}

impl SemanticVisitor {
    pub(crate) fn new(root: &JsAnyRoot) -> Self {
        Self {
            extractor: SemanticEventExtractor::default(),
            builder: SemanticModelBuilder::new(root.clone()),
        }
    }
}

impl Visitor for SemanticVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<JsLanguage>>,
        _ctx: VisitorContext<JsLanguage>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                self.builder.push_node(node);
                self.extractor.enter(node);
            }
            WalkEvent::Leave(node) => {
                self.extractor.leave(node);
            }
        }

        while let Some(e) = self.extractor.pop() {
            self.builder.push_event(e);
        }
    }

    fn finish(&mut self, ctx: &mut AnalyzerContext<JsLanguage>) {
        let mut builder = SemanticModelBuilder::new(ctx.root.clone());
        swap(&mut builder, &mut self.builder);

        let services = ctx
            .services
            .get_mut()
            .expect("service bag has outstanding references");

        let model = builder.build();
        services.insert_service(model);
    }
}
