use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub enum CannotCreateServicesError {
    /// List the missing services necessary to create the service bag
    MissingServices(&'static [&'static str]),
}

pub trait FromServices: Sized {
    fn from_services(services: &ServiceBag) -> Result<Self, CannotCreateServicesError>;
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
    fn from_services(_: &ServiceBag) -> Result<Self, CannotCreateServicesError> {
        Ok(())
    }
}
