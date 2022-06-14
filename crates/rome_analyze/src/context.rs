use std::sync::Arc;

use rome_js_syntax::{JsAnyRoot, JsLanguage};
use rome_rowan::{AstNode, Language};

use crate::{
    registry::{LanguageRoot, Rule},
    LanguageOfRule,
};

/// Specifies which services a language needs
/// to be able to run all possible lint rules.
///
/// Also instantiate all these services.
pub trait LanguageSpecificServiceBag: Language {
    type Services;

    fn new(root: LanguageRoot<Self>) -> Self::Services;
}

/// Easy access to which services a language needs to run all
/// lint rules.
type ServicesOf<Language> = <Language as LanguageSpecificServiceBag>::Services;

/// Carries all services a language needs to run all
/// lint rules
#[derive(Clone)]
pub struct RuleContextServiceBag<Language>
where
    Language: LanguageSpecificServiceBag,
{
    services: Arc<ServicesOf<Language>>,
}

impl<L> RuleContextServiceBag<L>
where
    L: LanguageSpecificServiceBag,
{
    pub fn new(root: LanguageRoot<L>) -> Self {
        Self {
            services: Arc::new(<L as LanguageSpecificServiceBag>::new(root)),
        }
    }
}

impl<Language> std::ops::Deref for RuleContextServiceBag<Language>
where
    Language: LanguageSpecificServiceBag,
{
    type Target = ServicesOf<Language>;

    fn deref(&self) -> &Self::Target {
        &self.services
    }
}

/// Gives lint Rules access to everything associated with
/// a lint analyze run, such as:
/// - Nodes and the parsed tree
/// - All services as specified for each language
pub(crate) struct RuleContext<TRule>
where
    TRule: Rule,
    LanguageOfRule<TRule>: LanguageSpecificServiceBag,
{
    query_result: <TRule as Rule>::Query,
    services: RuleContextServiceBag<LanguageOfRule<TRule>>,
}

impl<TRule> RuleContext<TRule>
where
    TRule: Rule,
    LanguageOfRule<TRule>: LanguageSpecificServiceBag,
{
    pub fn new(
        query_result: <TRule as Rule>::Query,
        services: RuleContextServiceBag<LanguageOfRule<TRule>>,
    ) -> Self {
        Self {
            query_result,
            services,
        }
    }

    pub fn query(&self) -> &<TRule as Rule>::Query {
        &self.query_result
    }
}

/// Specific methods for Javascript rules
pub trait JsRuleContext {
    fn root(&self) -> &JsAnyRoot;
}

impl<TRule> JsRuleContext for RuleContext<TRule>
where
    TRule: Rule,
    <TRule as Rule>::Query: AstNode<Language = JsLanguage>,
{
    fn root(&self) -> &JsAnyRoot {
        &self.services.0
    }
}

/// Specific services for Javascript Rules
impl LanguageSpecificServiceBag for JsLanguage {
    type Services = (JsAnyRoot,);

    fn new(root: LanguageRoot<Self>) -> Self::Services {
        (root,)
    }
}
