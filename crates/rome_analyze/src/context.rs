use crate::{
    registry::RuleRoot, CannotCreateServicesError, FromServices, Queryable, Rule, ServiceBag,
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
}

impl<'a, R> RuleContext<'a, R>
where
    R: ?Sized + Rule,
{
    pub fn new(
        query_result: &'a RuleQueryResult<R>,
        root: &'a RuleRoot<R>,
        services: &ServiceBag,
    ) -> Result<Self, CannotCreateServicesError> {
        Ok(Self {
            query_result,
            root,
            services: FromServices::from_services(services)?,
        })
    }

    pub fn query(&self) -> &RuleQueryResult<R> {
        self.query_result
    }

    pub fn root(&self) -> RuleRoot<R> {
        self.root.clone()
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
