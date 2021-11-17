//! # Task
//! breakpoint reached future
//!
//!

use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::broadcast::{channel, Receiver, Sender};

pub struct Breakpoint {
    reached_tx: Sender<()>,
}

impl Breakpoint {
    pub fn dummy() -> Self {
        let (reached_tx, _) = channel(1);
        Self { reached_tx }
    }

    pub fn notify_reached(&self) {
        self.reached_tx.send(()).unwrap();
    }

    pub fn subscribe(&self) -> BreakpointReachedFuture {
        BreakpointReachedFuture::new(self.reached_tx.subscribe())
    }
}

pub struct BreakpointReachedFuture {
    // todo
}

impl BreakpointReachedFuture {
    pub fn new(receiver: Receiver<()>) -> Self {
        Self {
            // todo
        }
    }
}

impl std::future::Future for BreakpointReachedFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
