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

    fn text(&self) -> &str {
        match &self.kind {
            FdMessageKind::WriteFd(data) => core::str::from_utf8(&data).unwrap(),
            _ => unimplemented!(),
        }
    }
}

impl Message for FdMessage {
    fn target(&self) -> &str { "fd_manager" }

    fn on_response(&self, response: ArcMessage) {
        // write result back into the original Message
        // we need the result from where we routed the message from!
    }
}

#[derive(Clone)]
pub enum FileDescriptor {
    Stdin,
    Stdout,
    Stderr,
}

impl FileDescriptor {
    pub fn write(&self, data: &[u8]) {
        let message = match self {
            Self::Stdout => super::StdoutSyslogMessage::new(core::str::from_utf8(data).unwrap()),
            Self::Stderr => super::StdoutSyslogMessage::new_err(core::str::from_utf8(data).unwrap()),
            _ => unimplemented!(),
        };
        service_manager().route_message(message);
    }
}

pub struct FileDescriptorManager {
    fd_list: Vec<FileDescriptor>,
}

impl FileDescriptorManager {
    pub fn new() -> Self {
        Self {
            fd_list: Vec::new(),
        }
    }

    fn get_fd(&self, fd: i32) -> Option<FileDescriptor> {
        if fd < 0 { return None; }
        match fd {
            0 => Some(FileDescriptor::Stdin),
            1 => Some(FileDescriptor::Stdout),
            2 => Some(FileDescriptor::Stderr),
            _ => self.fd_list.get((fd - 2) as usize).map(|fd| fd.clone())
        }
    }
}

impl Service for FileDescriptorManager {
    fn name(&self) -> String { String::from("fd_manager") }

    fn push_message(&self, message: ArcMessage) {
        if let Some(message) = message.as_any().downcast_ref::<FdMessage>() {
            if let Some(fd) = self.get_fd(message.fd) {
                match &message.kind {
                    FdMessageKind::WriteFd(data) => {
                        fd.write(&data);
                    },
                    _ => unimplemented!(),
                }
            } else {
                panic!("Fd not found!")
            }
        } else {
            panic!("Unsupported message type!");
        }
    }
}
