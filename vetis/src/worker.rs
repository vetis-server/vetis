use crate::VetisResult;
use crossfire::{MAsyncTx, mpmc::Array, oneshot};

#[derive(Clone)]
/// ScriptWorkerLink
pub struct WorkerLink<T>
where
    T: 'static,
{
    name: String,
    sender: MAsyncTx<Array<(T, oneshot::TxOneshot<T>)>>,
}

impl<T> WorkerLink<T> {
    /// Create a new link instance
    pub fn new(name: &str, sender: MAsyncTx<Array<(T, oneshot::TxOneshot<T>)>>) -> Self {
        Self { name: name.into(), sender }
    }

    /// Returns link name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns link sender
    pub fn sender(&self) -> &MAsyncTx<Array<(T, oneshot::TxOneshot<T>)>> {
        &self.sender
    }
}

///Worker trait
pub trait Worker {
    /// Worker message type
    type MessageType;

    /// Return worker id
    fn id(&self) -> usize;
    /// Return link
    fn link(&self) -> &WorkerLink<Self::MessageType>;
    /// Run worker
    fn run(&self) -> VetisResult<()>;
}
