use rome_analyze::{
    CannotCreateServicesError, Phase, Phases, QueryKey, QueryMatch, Queryable, ServiceBag,
};
use rome_js_semantic::SemanticModel;
use rome_js_syntax::JsLanguage;
use rome_rowan::AstNode;

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
            .get()
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
