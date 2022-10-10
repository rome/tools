use crate::{
    registry::RuleRoot, AnalyzerOptions, FromServices, Queryable, Rule, RuleContextDiagnostic,
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
    query_result: &'a RuleQueryResult<R>,
    root: &'a RuleRoot<R>,
    services: RuleServiceBag<R>,
    options: Option<R::Options>,
}

impl<'a, R> RuleContext<'a, R>
where
    R: Rule + Sized,
{
    pub fn new(
        query_result: &'a RuleQueryResult<R>,
        root: &'a RuleRoot<R>,
        services: &ServiceBag,
        options: &'a AnalyzerOptions,
    ) -> Result<Self, RuleContextDiagnostic> {
        let rule_key = RuleKey::rule::<R>();
        let options = options.configuration.rules.get_rule(&rule_key);
        let options = if let Some(options) = options {
            let value = options.value();
            serde_json::from_value(value.clone()).map_err(|error| {
                RuleContextDiagnostic::from_serde(rule_key.rule_name(), &value.to_string(), error)
            })?
        } else {
            None
        };

        Ok(Self {
            query_result,
            root,
            services: FromServices::from_services(&rule_key, services)?,
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
    pub fn options(&self) -> Option<&R::Options> {
        self.options.as_ref()
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
            .get_rule(&RuleKey::rule::<R>())
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
