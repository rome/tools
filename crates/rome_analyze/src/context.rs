use crate::{
    registry::RuleRoot, AnalyzerOptions, CannotCreateServicesError, FromServices, Queryable, Rule,
    ServiceBag,
};
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
    options: &'a AnalyzerOptions,
}

impl<'a, R> RuleContext<'a, R>
where
    R: ?Sized + Rule,
{
    pub fn new(
        query_result: &'a RuleQueryResult<R>,
        root: &'a RuleRoot<R>,
        services: &ServiceBag,
        options: &'a AnalyzerOptions,
    ) -> Result<Self, CannotCreateServicesError> {
        Ok(Self {
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
