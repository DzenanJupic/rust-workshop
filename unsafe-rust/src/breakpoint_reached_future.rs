//! # Task
//! breakpoint reached future
//!
//!

use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::broadcast::{channel, error::RecvError, Receiver, Sender};

type ChannelReceivedFuture = Box<dyn std::future::Future<Output=Result<(), RecvError>>>;

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
    fut: Pin<ChannelReceivedFuture>,
}

impl BreakpointReachedFuture {
    pub fn new(mut receiver: Receiver<()>) -> Self {
        // let fut = Box::pin(receiver.recv()) as Pin<ChannelReceivedFuture>;
        //           ---------^^^^^^^^^^^^^^^-------------------------------
        //           |        |
        //           |        borrowed value does not live long enough
        //           type annotation requires that `receiver` is borrowed for `'static`
        //       ...
        //
        //  `receiver` dropped here while still borrowed

        Self {
            fut: todo!(),
        }
    }
}

impl std::future::Future for BreakpointReachedFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.fut
            .as_mut()
            .poll(cx)
            .map(|_| ())
    }
}
