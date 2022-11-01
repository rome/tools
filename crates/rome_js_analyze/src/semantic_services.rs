use rome_analyze::{
    FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, QueryMatch, Queryable,
    RuleKey, ServiceBag, Visitor, VisitorContext, VisitorFinishContext,
};
use rome_js_semantic::{SemanticEventExtractor, SemanticModel, SemanticModelBuilder};
use rome_js_syntax::{AnyJsRoot, JsLanguage, WalkEvent};
use rome_rowan::{AstNode, SyntaxNode};

pub struct SemanticServices {
    model: SemanticModel,
}

impl SemanticServices {
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }
}

impl FromServices for SemanticServices {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic> {
        let model = services.get_service().ok_or_else(|| {
            MissingServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"])
        })?;
        Ok(Self { model })
    }
}

impl Phase for SemanticServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// The [SemanticServices] types can be used as a queryable to get an instance
/// of the whole [SemanticModel] without matching on a specific AST node
impl Queryable for SemanticServices {
    type Output = SemanticModel;
    type Language = JsLanguage;
    type Services = Self;

    const KEY: QueryKey<Self::Language> = QueryKey::SemanticModel;

    fn unwrap_match(services: &ServiceBag, query: &QueryMatch<Self::Language>) -> Self::Output {
        match query {
            QueryMatch::SemanticModel(..) => services
                .get_service::<SemanticModel>()
                .expect("SemanticModel service is not registered"),
            _ => panic!("tried to unwrap unsupported QueryMatch kind, expected SemanticModel"),
        }
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

    fn unwrap_match(_: &ServiceBag, query: &QueryMatch<Self::Language>) -> Self::Output {
        match query {
            QueryMatch::Syntax(node) => N::unwrap_cast(node.clone()),
            _ => panic!("tried to unwrap unsupported QueryMatch kind, expected Syntax"),
        }
    }
}

pub(crate) struct SemanticModelBuilderVisitor {
    extractor: SemanticEventExtractor,
    builder: SemanticModelBuilder,
}

impl SemanticModelBuilderVisitor {
    pub(crate) fn new(root: &AnyJsRoot) -> Self {
        Self {
            extractor: SemanticEventExtractor::default(),
            builder: SemanticModelBuilder::new(root.clone()),
        }
    }
}

impl Visitor for SemanticModelBuilderVisitor {
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

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        let model = self.builder.build();
        ctx.services.insert_service(model);
    }
}

pub(crate) struct SemanticModelVisitor;

impl Visitor for SemanticModelVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<JsLanguage>>,
        mut ctx: VisitorContext<JsLanguage>,
    ) {
        let root = match event {
            WalkEvent::Enter(node) => {
                if node.parent().is_some() {
                    return;
                }

                node
            }
            WalkEvent::Leave(_) => return,
        };

        let text_range = root.text_range();
        ctx.match_query(QueryMatch::SemanticModel(text_range));
    }
}
