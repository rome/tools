use crate::{
    registry::RuleRoot, FromServices, Queryable, Rule, RuleKey, ServiceBag,
};
use rome_diagnostics::{Error, Result};
use std::ops::Deref;
use std::sync::Arc;

type RuleQueryResult<R> = <<R as Rule>::Query as Queryable>::Output;
type RuleServiceBag<R> = <<R as Rule>::Query as Queryable>::Services;

pub struct RuleContext<'a, R>
where
    R: ?Sized + Rule,
{
    query_result: &'a RuleQueryResult<R>,
    root: &'a RuleRoot<R>,
    services: RuleServiceBag<R>,
    options: Arc<R::Options>,
}

impl<'a, R> RuleContext<'a, R>
where
    R: Rule + Sized + 'static,
{
    pub fn new(
        query_result: &'a RuleQueryResult<R>,
        root: &'a RuleRoot<R>,
        services: &ServiceBag,
        options: Arc<R::Options>,
    ) -> Result<Self, Error> {
        let rule_key = RuleKey::rule::<R>();
        Ok(Self {
            query_result,
            root,
            services: FromServices::from_services(&rule_key, services)?,
            options
        })
    }

    pub fn query(&self) -> &RuleQueryResult<R> {
        self.query_result
    }

    /// Returns a clone of the AST root
    pub fn root(&self) -> RuleRoot<R> {
        self.root.clone()
    }

    /// It retrieves the options that belong to a rule, if they exist.
    ///
    /// In order to retrieve a typed data structure, you have to create a deserializable
    /// data structure and define it inside the generic type `type Options` of the [Rule]
    ///
    /// ## Examples
    ///
    /// ```rust,ignore
    /// use rome_analyze::{declare_rule, Rule, RuleCategory, RuleMeta, RuleMetadata};
    /// use rome_analyze::context::RuleContext;
    /// use serde::Deserialize;
    /// declare_rule! {    
    ///     /// Some doc
    ///     pub(crate) Name {
    ///         version: "0.0.0",
    ///         name: "name",
    ///         recommended: true,
    ///     }
    /// }
    ///
    /// #[derive(Deserialize)]
    /// struct RuleOptions {}
    ///
    /// impl Rule for Name {
    ///     const CATEGORY: RuleCategory = RuleCategory::Lint;
    ///     type Query = ();
    ///     type State = ();
    ///     type Signals = ();
    ///     type Options = RuleOptions;
    ///
    ///     fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    ///         if let Some(options) = ctx.options() {
    ///             // do something with the options now
    ///         }
    ///     }
    /// }
    /// ```
    pub fn options(&self) -> &R::Options {
        self.options.as_ref()
    }
}

impl<'a, R> Deref for RuleContext<'a, R>
where
    R: Rule,
{
    type Target = RuleServiceBag<R>;

    fn deref(&self) -> &Self::Target {
        &self.services
    }
}
