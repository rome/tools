use rome_analyze::{
    FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, QueryMatch, Queryable,
    RuleKey, ServiceBag,
};
use rome_aria::{AriaProperties, AriaRoles};
use rome_js_syntax::JsLanguage;
use rome_rowan::AstNode;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct AriaServices {
    roles: Arc<AriaRoles>,
    properties: Arc<AriaProperties>,
}

impl AriaServices {
    pub fn aria_roles(&self) -> &AriaRoles {
        &self.roles
    }

    pub fn aria_properties(&self) -> &AriaProperties {
        &self.properties
    }
}

impl FromServices for AriaServices {
    fn from_services(
        _rule_key: &RuleKey,
        _services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic> {
        Ok(Self {
            roles: Arc::new(AriaRoles::default()),
            properties: Arc::new(AriaProperties::default()),
        })
    }
}

impl Phase for AriaServices {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules **that uses the semantic model** to match on specific [AstNode] types
#[derive(Clone)]
pub(crate) struct Aria<N>(pub N);

impl<N> Queryable for Aria<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Output = N;
    type Language = JsLanguage;
    type Services = AriaServices;

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
