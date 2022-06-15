use crate::{registry::RuleRoot, Rule};

pub struct RuleContext<R>
where
    R: ?Sized + Rule,
{
    query_result: <R as Rule>::Query,
    root: RuleRoot<R>,
}

impl<R> RuleContext<R>
where
    R: Rule,
{
    pub fn new(query_result: <R as Rule>::Query, root: RuleRoot<R>) -> Self {
        Self { query_result, root }
    }

    pub fn query_result(&self) -> &<R as Rule>::Query {
        &self.query_result
    }

    pub fn root(&self) -> &RuleRoot<R> {
        &self.root
    }
}
