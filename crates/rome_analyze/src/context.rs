use crate::{
    registry::RuleRoot, AnalyzerOptions, CannotCreateServicesError, FromServices, Queryable, Rule,
    RuleKey, ServiceBag,
};
use serde::Deserialize;
use std::ops::Deref;

type RuleQueryResult<R> = <<R as Rule>::Query as Queryable>::Output;
type RuleServiceBag<R> = <<R as Rule>::Query as Queryable>::Services;

pub struct RuleContext<'a, R>
where
    R: ?Sized + Rule,
{
    rule_key: RuleKey,
    query_result: &'a RuleQueryResult<R>,
    root: &'a RuleRoot<R>,
    services: RuleServiceBag<R>,
    options: &'a AnalyzerOptions,
}

impl<'a, R> RuleContext<'a, R>
where
    R: ?Sized + Rule,
{
    pub fn new(
        rule_key: RuleKey,
        query_result: &'a RuleQueryResult<R>,
        root: &'a RuleRoot<R>,
        services: &ServiceBag,
        options: &'a AnalyzerOptions,
    ) -> Result<Self, CannotCreateServicesError> {
        Ok(Self {
            rule_key,
            query_result,
            root,
            services: FromServices::from_services(services)?,
            options,
        })
    }

    pub fn query(&self) -> &RuleQueryResult<R> {
        self.query_result
    }

    /// Returns a clone of the AST root
    pub fn root(&self) -> RuleRoot<R> {
        self.root.clone()
    }

    /// Returns the analyzer options
    pub fn options(&self) -> &AnalyzerOptions {
        self.options
    }

    /// It retrieves the options that belong to a rule, if they exist.
    ///
    /// In order to retrieve a typed data structure, the function has to accept a `FromType`, a
    /// `ToType` (this one, inferrable by the compiler) and a closure that does the mapping.
    ///
    /// Usually, options are a `serde::RawValue` and need to be mapped to a sized type.
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
    /// struct RuleSettings {}
    ///
    /// impl Rule for Name {
    ///     const CATEGORY: RuleCategory = RuleCategory::Lint;
    ///     type Query = ();
    ///     type State = ();
    ///     type Signals = ();
    ///
    ///     fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    ///         let options = ctx.rule_settings::<RuleSettings>();
    ///     }
    /// }
    /// ```
    pub fn rule_settings<'de, ToType: Deserialize<'de>>(&'de self) -> Option<ToType> {
        self.options
            .configuration
            .rules
            .get_rule(&self.rule_key)
            .map(|options| serde_json::from_str::<ToType>(options.value()))
            // TODO: ignore the error for now, it should be handled differently https://github.com/rome/tools/issues/3346
            .and_then(|result| result.ok())
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
