use rome_analyze::{
    FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, QueryMatch, Queryable,
    RuleKey, ServiceBag,
};
use rome_aria::iso::{countries, is_valid_country, is_valid_language, languages};
use rome_aria::{AriaProperties, AriaRoles};
use rome_js_syntax::JsLanguage;
use rome_rowan::AstNode;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct AriaServices {
    pub(crate) roles: Arc<AriaRoles>,
    pub(crate) properties: Arc<AriaProperties>,
}

impl AriaServices {
    pub fn aria_roles(&self) -> &AriaRoles {
        &self.roles
    }

    pub fn aria_properties(&self) -> &AriaProperties {
        &self.properties
    }

    pub fn is_valid_iso_language(&self, language: &str) -> bool {
        is_valid_language(language)
    }

    pub fn is_valid_iso_country(&self, country: &str) -> bool {
        is_valid_country(country)
    }

    pub fn iso_country_list(&self) -> &'static [&'static str] {
        countries()
    }

    pub fn iso_language_list(&self) -> &'static [&'static str] {
        languages()
    }
}

impl FromServices for AriaServices {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic> {
        let roles: &Arc<AriaRoles> = services
            .get_service()
            .ok_or_else(|| MissingServicesDiagnostic::new(rule_key.rule_name(), &["AriaRoles"]))?;
        let properties: &Arc<AriaProperties> = services.get_service().ok_or_else(|| {
            MissingServicesDiagnostic::new(rule_key.rule_name(), &["AriaProperties"])
        })?;
        Ok(Self {
            roles: roles.clone(),
            properties: properties.clone(),
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
