use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone, Default)]
pub struct State {
    inner: Arc<StateInner>,
}

impl Deref for State {
    type Target = StateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Default)]
pub struct StateInner {
    // TODO: here we save the packets
}
