use alloc::vec::Vec;
use alloc::string::String;
use kernel_common::services::*;

enum FdMessageKind {
    WriteFd(Vec<u8>),
}

pub struct FdMessage {
    fd: i32,
    kind: FdMessageKind,
}

impl FdMessage {
    pub fn fd_write(fd: i32, data: Vec<u8>) -> Self {
        Self {
            fd,
            kind: FdMessageKind::WriteFd(data),
        }
    }
}

impl Message for FdMessage {
    fn as_any(&self) -> &dyn core::any::Any { self }
    fn target(&self) -> &str { "fd_manager" }

    fn on_response(&self, response: ArcMessage) {
        // Route the message received by the manager
        service_manager().route_message(response);
    }
}

pub struct FileDescriptorManager {
    fd_list: Vec<i32>,
}

impl FileDescriptorManager {
    pub fn new() -> Self {
        Self {
            fd_list: Vec::new(),
        }
    }
}

impl Service for FileDescriptorManager {
    fn name(&self) -> String { String::from("fd_manager") }

    fn push_message(&self, message: ArcMessage) {
        if let Some(message) = message.as_any().downcast_ref::<FdMessage>() {
            panic!("downcasted");
        } else {
            panic!("not downcasted");
        }
    }
}
