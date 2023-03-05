use alloc::{
    vec::Vec,
    sync::Arc,
    boxed::Box,
    string::String,
};
use crossbeam_queue::SegQueue;
use conquer_once::spin::OnceCell;
use hashbrown::HashMap;
use spin::Mutex;

static SERVICE_MANAGER: OnceCell<ServiceManager> = OnceCell::uninit();

pub fn service_manager() -> &'static ServiceManager {
    SERVICE_MANAGER.get_or_init(|| ServiceManager::new())
}

pub trait Message: Send + Sync {
    fn target(&self) -> &str;

    /// Called by the service that handled the message.
    fn on_response(&self, response: Box<dyn Message>);
}

pub type ArcMessage = Arc<Box<dyn Message>>;
pub type MessageQueue = SegQueue<ArcMessage>;

pub trait Service: Send + Sync {
    /// Used for service identification. Should remain constant while the service is running!
    fn name(&self) -> String;
    /// Specify the services this service depends on.
    /// If you attempt to start this service, it should try to start
    /// it's dependencies first. Make sure not to specify circular dependencies!
    fn dependencies(&self) -> Vec<Box<dyn Service>>;
    fn message_queue(&self) -> MessageQueue;
}

/// A handle to a running service
pub struct ServiceHandle {
    service: Arc<Box<dyn Service>>,
}

impl ServiceHandle {
    pub(crate) fn from_service(service: Arc<Box<dyn Service>>) -> Self {
        Self {
            service,
        }
    }

    pub fn push_message(&self, message: ArcMessage) {
        self.service.message_queue().push(message);
    }
}

pub struct ServiceManager {
    services: Mutex< HashMap<String, Arc<Box<dyn Service>>> >,
}

impl ServiceManager {
    pub(crate) fn new() -> Self {
        Self {
            services: Mutex::new(HashMap::new()),
        }
    }

    /// Returns a service based on its name. If the service is not running, returns None.
    pub fn service_handle_from_name(&self, name: impl AsRef<str>) -> Option<ServiceHandle> {
        let name = name.as_ref();
        let lock = self.services.lock();
        let service = lock.get(name).map(|service| ServiceHandle::from_service(service.clone()));
        drop(lock);
        service
    }

    /// Add a service to the service manager.
    /// Adds all the dependencies of this service first.
    /// Softlocks if a service contains a circular dependency.
    /// If a service already exists, returns a handle to the existing service instead.
    pub fn add_service(&self, service: Box<dyn Service>) -> ServiceHandle {
        let service_name = service.name();
        if let Some(handle) = self.service_handle_from_name(&service_name) {
            return handle;
        }

        for dependency in service.dependencies() {
            self.add_service(dependency);
        }

        let arc_service = Arc::new(service);
        let mut lock = self.services.lock();
        lock.insert(service_name, arc_service.clone());
        drop(lock);
        ServiceHandle::from_service(arc_service)
    }

    /// Routes a message to the right service.
    /// Errors if:
    /// - The target service can't be found;
    pub fn route_message(&self, message: ArcMessage) -> Result<(), RouteMessageError> {
        let target_name = message.target();
        let service_handle = self.service_handle_from_name(target_name).ok_or(RouteMessageError::ServiceNotFound)?;
        service_handle.push_message(message);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RouteMessageError {
    ServiceNotFound,
}
