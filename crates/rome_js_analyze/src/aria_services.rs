use rome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, Queryable,
    RuleKey, ServiceBag, SyntaxVisitor,
};
use rome_aria::iso::{countries, is_valid_country, is_valid_language, languages};
use rome_aria::{AriaProperties, AriaRoles};
use rome_js_syntax::{AnyJsRoot, AnyJsxAttribute, JsLanguage, JsSyntaxNode, JsxAttributeList};
use rome_rowan::AstNode;
use std::collections::HashMap;
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

    /// Extracts attributes as HashMap (key: attribute name, value: attribute values).
    pub fn extract_attributes(
        &self,
        attribute_list: &JsxAttributeList,
    ) -> Option<HashMap<String, Vec<String>>> {
        let mut defined_attributes: HashMap<String, Vec<String>> = HashMap::new();
        for attribute in attribute_list {
            if let AnyJsxAttribute::JsxAttribute(attr) = attribute {
                let name = attr.name().ok()?.syntax().text_trimmed();
                let name = name.to_string().to_lowercase();
                // handle name only attribute e.g. `<img aria-hidden alt="photo" />`
                let Some(initializer) = attr.initializer() else {
                    defined_attributes.entry(name).or_insert(vec!["true".to_string()]);
                    continue
                };
                let initializer = initializer.value().ok()?;
                let text = initializer.inner_text_value().ok()??;
                let text = text.to_lowercase();
                // handle multiple values e.g. `<div role="button checkbox">`
                let values = text.split(' ');
                let values = values.map(|s| s.to_string()).collect::<Vec<String>>();
                defined_attributes.entry(name).or_insert(values);
            }
        }
        Some(defined_attributes)
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
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = AriaServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
