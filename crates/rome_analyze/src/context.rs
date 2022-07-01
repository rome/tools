use crate::{registry::RuleRoot, services::ServiceBag, Queryable, Rule};
use std::ops::Deref;

pub struct RuleContext<'a, R>
where
    R: ?Sized + Rule,
{
    query_result: &'a <<R as Rule>::Query as Queryable>::Output,
    root: &'a RuleRoot<R>,
    services: <<R as Rule>::Query as Queryable>::Services,
}

impl<'a, R> RuleContext<'a, R>
where
    R: Rule,
{
    pub fn new(
        query_result: &'a <<R as Rule>::Query as Queryable>::Output,
        root: &'a RuleRoot<R>,
        services: ServiceBag,
    ) -> Result<Self, <<<R as Rule>::Query as Queryable>::Services as TryFrom<ServiceBag>>::Error>
    {
        Ok(Self {
            query_result,
            root,
            services: services.try_into()?,
        })
    }

    pub fn query(&self) -> &<<R as Rule>::Query as Queryable>::Output {
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
    type Target = <<R as Rule>::Query as Queryable>::Services;

    fn deref(&self) -> &Self::Target {
        &self.services
    }
}
