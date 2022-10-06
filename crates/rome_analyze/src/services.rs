use crate::{RuleKey, TextRange, TextSize};
use rome_diagnostics::v2::{Diagnostic, LineIndexBuf, Resource, SourceCode};
use serde_json::Error;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "internalError/io", tags(INTERNAL))]
pub struct RuleContextDiagnostic {
    #[message]
    message: String,
    #[description]
    description: String,
    #[location(resource)]
    path: Resource<&'static str>,
    #[location(span)]
    span: Option<TextRange>,
    #[location(source_code)]
    source_code: Option<SourceCode<String, LineIndexBuf>>,
}

impl RuleContextDiagnostic {
    pub fn from_serde(rule_name: &str, input: &str, error: Error) -> Self {
        let line_starts = LineIndexBuf::from_source_text(input);

        let line_index = error.line().checked_sub(1);
        let span = line_index.and_then(|line_index| {
            let line_start = line_starts.get(line_index)?;

            let column_index = error.column().checked_sub(1)?;
            let column_offset = TextSize::try_from(column_index).ok()?;

            let span_start = line_start + column_offset;
            Some(TextRange::at(span_start, TextSize::from(0)))
        });

        Self {
            message: format!("Errors emitted while attempting run the rule: {rule_name}"),
            description: error.to_string(),
            path: Resource::Memory,
            span,
            source_code: Some(SourceCode {
                text: input.to_string(),
                line_starts: Some(line_starts),
            }),
        }
    }

    pub fn from_services(rule_name: &str, missing_services: &'static [&'static str]) -> Self {
        let description = missing_services.join(", ");
        Self {
            message: format!("Errors emitted while attempting run the rule: {rule_name}"),
            description: format!("Missing services: {description}"),
            source_code: None,
            path: Resource::Memory,
            span: None,
        }
    }
}

pub trait FromServices: Sized {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, RuleContextDiagnostic>;
}

#[derive(Default)]
pub struct ServiceBag {
    services: HashMap<TypeId, Box<dyn Any>>,
}

impl ServiceBag {
    pub fn insert_service<T: 'static + Clone>(&mut self, service: T) {
        let id = TypeId::of::<T>();
        self.services.insert(id, Box::new(service));
    }

    pub fn get_service<T: 'static + Clone>(&self) -> Option<T> {
        let id = TypeId::of::<T>();
        let svc = self.services.get(&id)?;
        svc.downcast_ref().cloned()
    }
}

impl FromServices for () {
    fn from_services(_: &RuleKey, _: &ServiceBag) -> Result<Self, RuleContextDiagnostic> {
        Ok(())
    }
}
