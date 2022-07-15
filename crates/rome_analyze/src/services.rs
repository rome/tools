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

#[derive(Clone)]
pub struct ServiceBag(Arc<ServiceBagData>);

impl Default for ServiceBag {
    fn default() -> Self {
        let services = ServiceBagData::default();
        ServiceBag::new(services)
    }
}

impl ServiceBag {
    pub fn new(services: ServiceBagData) -> Self {
        Self(Arc::new(services))
    }

    pub fn get_mut(&mut self) -> Option<&mut ServiceBagData> {
        let ServiceBag(data) = self;
        Arc::get_mut(data)
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
