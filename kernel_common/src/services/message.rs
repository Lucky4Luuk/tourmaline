use alloc::{
    sync::Arc,
    boxed::Box,
};
use as_any::AsAny;

pub trait Message: Send + Sync + AsAny {
    fn target(&self) -> &str;
    fn data(&self) -> &[u8] { unimplemented!("Message::data") }
    /// Optional method to encode the data as a &str.
    /// By default, it's implemented to just return None.
    fn data_as_str(&self) -> Option<&str> { None }
    /// Called by the service that handled the message.
    /// The default implementation simply does nothing.
    fn on_response(&self, _response: ArcMessage) {}
}

pub struct ArcMessage {
    inner: Arc<Box<dyn Message>>,
}

impl ArcMessage {
    pub fn new(msg: Box<dyn Message>) -> Self {
        Self {
            inner: Arc::new(msg),
        }
    }
}

impl core::ops::Deref for ArcMessage {
    type Target = Arc<Box<dyn Message>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ArcMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
