//! ## What is unsafe Rust?
//! - Dereference a raw pointer
//! - Call an unsafe function or method
//! - Access or modify a mutable static variable
//! - Implement an unsafe trait
//! - Access fields of unions
//!
//! ## Examples
//! - [ ] Box<T>
//! - [ ] Mutex<T>
//! - [ ] BreakpointReachedFuture
//!
//! ## Other topics
//! - [ ] soundness
//! - [ ] Pinning

#![allow(dead_code, unused_variables, unused_mut, unreachable_code)]

mod boxed;
mod mutex;
mod breakpoint_reached_future;

/*#[test]
fn test_box() {
    use boxed::Box;

    let mut b = Box::new(42);
    assert_eq!(dbg!(*b), 42);
    *b = 5;
    assert_eq!(dbg!(*b), 5);
}*/

/*#[test]
fn test_mutex() {
    use mutex::Mutex;

    let m = Mutex::new(42);
    assert_eq!(dbg!(*m.lock()), 42);
    *m.lock() = 5;
    assert_eq!(dbg!(*m.lock()), 5);
}*/


/*#[tokio::test]
async fn test_breakpoint_reached_future() {
    use std::sync::Arc;
    use breakpoint_reached_future::{Breakpoint, BreakpointReachedFuture};

    let breakpoint = Arc::new(Breakpoint::dummy());
    let reached_future = breakpoint.subscribe();

    let bp = Arc::clone(&breakpoint);
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(5));
        bp.notify_reached();
    });

    reached_future.await;
    eprintln!("breakpoint was reached!");
}*/
