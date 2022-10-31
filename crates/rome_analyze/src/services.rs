use crate::{RuleKey, TextRange};
use rome_diagnostics::{Diagnostic, LineIndexBuf, Resource, Result, SourceCode};
use std::{
    any::{Any, TypeId},
    collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher},
};

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "internalError/io", tags(INTERNAL))]
pub struct MissingServicesDiagnostic {
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

impl MissingServicesDiagnostic {
    pub fn new(rule_name: &str, missing_services: &'static [&'static str]) -> Self {
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
    #[allow(clippy::result_large_err)]
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic>;
}

#[derive(Default)]
pub struct ServiceBag {
    services: HashMap<u64, Box<dyn Any>>,
}

fn get_id_of(id: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    hasher.finish()
}

impl ServiceBag {
    pub fn insert_service<T: 'static + Clone>(&mut self, service: T) {
        let id = TypeId::of::<T>();
        self.services.insert(get_id_of(&id), Box::new(service));
    }

    pub fn insert_service_with_id<H: Hash, T: 'static + Clone>(&mut self, id: &H, service: T) {
        self.services.insert(get_id_of(&id), Box::new(service));
    }

    pub fn get_service<T: 'static + Clone>(&self) -> Option<T> {
        let id = TypeId::of::<T>();
        let id = get_id_of(&id);
        let svc = self.services.get(&id)?;
        svc.downcast_ref().cloned()
    }

    pub fn get_service_by_id<H: Hash, T: 'static + Clone>(&self, id: &H) -> Option<T> {
        let id = get_id_of(&id);
        let svc = self.services.get(&id)?;
        svc.downcast_ref().cloned()
    }
}

impl FromServices for () {
    fn from_services(_: &RuleKey, _: &ServiceBag) -> Result<Self, MissingServicesDiagnostic> {
        Ok(())
    }
}
