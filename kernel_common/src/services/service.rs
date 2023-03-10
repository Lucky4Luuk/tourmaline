use alloc::{
    vec::Vec,
    boxed::Box,
    string::String,
};
use super::ArcMessage;

pub trait Service: Send + Sync {
    /// Used for service identification. Should remain constant while the service is running!
    fn name(&self) -> String;
    /// Specify the services this service depends on.
    /// If you attempt to start this service, it should try to start
    /// it's dependencies first. Make sure not to specify circular dependencies!
    fn dependencies(&self) -> Vec<Box<dyn Service>> { Vec::new() }
    /// Called by the ServiceManager to push a message to this service.
    /// Response to the message can be sent with `[Message::on_response]`
    fn push_message(&self, message: ArcMessage);
}
