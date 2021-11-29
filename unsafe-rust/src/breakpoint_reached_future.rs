//! # Task
//! breakpoint reached future
//!
//!

use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::ptr::NonNull;
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
    fut: ManuallyDrop<Pin<ChannelReceivedFuture>>,
    receiver: NonNull<Receiver<()>>,
}

impl BreakpointReachedFuture {
    pub fn new(mut receiver: Receiver<()>) -> Self {
        let boxed = Box::new(receiver);
        let mut ptr = unsafe { NonNull::new_unchecked(Box::into_raw(boxed)) };

        let receiver_ref: &'static mut Receiver<()> = unsafe { ptr.as_mut() };

        let fut = receiver_ref.recv();
        let fut = Box::pin(fut) as Pin<ChannelReceivedFuture>;

        Self {
            fut: ManuallyDrop::new(fut),
            receiver: ptr,
        }
    }
}

impl Drop for BreakpointReachedFuture {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.fut);
            drop(Box::from_raw(self.receiver.as_ptr()));
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
