use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, mpsc};
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{Receiver, Sender};

/// Returns an ActivityCount, and a channel where we'll signal deadlocks.
///
/// Clone the ActivityCount once per CPU to share the tracker between threads.
pub fn tracker(num_cpus: usize) -> (ActivityCount, Receiver<()>) {
    let (deadlock_sx, deadlock_rx) = mpsc::channel();

    // Note that all CPUs are initially active.
    //
    // If we relied on the CPUs to activate themselves, there would be a race condition
    // where one of the CPUs blocks waiting for input before sending anything, and we
    // detect a deadlock right away. (Even though there are other CPUs who will start sending
    // messages once they boot up.)
    let activity_count = ActivityCount {
        count: Arc::new(AtomicUsize::new(num_cpus)),
        deadlock: deadlock_sx,
    };

    (activity_count, deadlock_rx)
}

/// This tracks the number of messages in flight, plus the number of active (unblocked) CPUs.
///
/// If this ever hits 0, we have detected a deadlock: all CPUs are blocked waiting for messages, and
/// there are no messages in flight. Each time this happens, we send one message to the `deadlock` channel.
#[derive(Clone)]
pub struct ActivityCount {
    count: Arc<AtomicUsize>,
    deadlock: Sender<()>,
}

impl ActivityCount {
    /// Track one message send, or unblocked CPU.
    pub fn incr(&mut self) {
        self.count.fetch_add(1, SeqCst);
    }

    /// Track one message receive, or blocked CPU.
    pub fn decr(&mut self) {
        let prev = self.count.fetch_sub(1, SeqCst);
        if prev == 1 {
            // Signal a deadlock.
            // Ignore receiver hangups.
            let _ = self.deadlock.send(());
        }
    }
}
