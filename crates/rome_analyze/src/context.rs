use crate::{registry::RuleRoot, Rule};

pub struct RuleContext<'a, R>
where
    R: ?Sized + Rule,
{
    query_result: &'a <R as Rule>::Query,
    root: &'a RuleRoot<R>,
}

impl<'a, R> RuleContext<'a, R>
where
    R: Rule,
{
    pub fn new(query_result: &'a <R as Rule>::Query, root: &'a RuleRoot<R>) -> Self {
        Self { query_result, root }
    }

    pub fn query(&self) -> &<R as Rule>::Query {
        self.query_result
    }

    pub fn root(&self) -> RuleRoot<R> {
        self.root.clone()
    }
}
