use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Deref,
    sync::Arc,
};

use crate::CannotCreateServicesError;

#[derive(Default)]
pub struct ServiceBagData {
    services: HashMap<TypeId, Box<dyn Any>>,
}

impl ServiceBagData {
    pub fn insert<T: 'static + Clone>(&mut self, svc: T) {
        let id = TypeId::of::<T>();
        self.services.insert(id, Box::new(svc));
    }

    pub fn get<T: 'static + Clone>(&self) -> Option<T> {
        let id = TypeId::of::<T>();
        let svc = self.services.get(&id)?;
        svc.downcast_ref().cloned()
    }
}

#[derive(Clone)]
pub struct ServiceBag(Arc<ServiceBagData>);

impl ServiceBag {
    pub fn new(services: ServiceBagData) -> Self {
        Self(Arc::new(services))
    }
}

impl Deref for ServiceBag {
    type Target = ServiceBagData;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl TryFrom<ServiceBag> for () {
    type Error = CannotCreateServicesError;
    fn try_from(_: ServiceBag) -> Result<Self, Self::Error> {
        Ok(())
    }
}
