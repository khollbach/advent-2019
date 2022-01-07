use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use itertools::Itertools;
use crate::intcode_computer::IntcodeComputer;
use crate::solutions::day23::{NAT_ADDR, NUM_CPUS, Packet};
use crate::solutions::day23::deadlock::ActivityCount;

/// A networked computer, running the NIC software.
pub struct CPU {
    id: usize,

    inbound_msgs: Receiver<Packet>,
    network: Network,

    /// Shared count of the number of messages in flight plus the number of unblocked CPUs.
    /// If this hits 0, the system would appear to be deadlocked.
    activity_count: ActivityCount,
    /// How many times in a row has this CPU polled its message queue, without receiving anything.
    /// If this goes high enough, we'll assume that this CPU is currently blocked, and won't
    /// send any more messages until it receives one first.
    num_consecutive_polls: u64,
}

/// Handles to send packets to each computer.
#[derive(Clone)]
pub struct Network {
    pub cpus: Vec<Sender<Packet>>,
    pub nat: Sender<Packet>,
}

impl CPU {
    pub fn new(id: usize, inbound_msgs: Receiver<Packet>, network: Network, activity_count: ActivityCount) -> Self {
        let num_consecutive_polls = 0;
        Self { id, inbound_msgs, network, activity_count, num_consecutive_polls }
    }

    pub fn run(self, nic_program: Vec<i64>) {
        // CPU input and output both need `&mut self`.
        // (They won't be called concurrently by the IntcodeComputer, so this is fine.)
        let self1 = Rc::new(RefCell::new(self));
        let self2 = Rc::clone(&self1);

        IntcodeComputer::new(nic_program).io(
            Self::provide_cpu_input(self1),
            Self::handle_cpu_output(self2),
        ).run();
    }

    /// Returns a closure that gets called when the CPU requests input.
    ///
    /// Handles incoming messages.
    fn provide_cpu_input(self_: Rc<RefCell<Self>>) -> impl FnMut() -> i64 {
        let mut id = Some(self_.borrow().id as i64);
        let mut inbound_packet_y = None;

        move || {
            let mut self_ = self_.borrow_mut();

            if let Some(id) = id.take() {
                // First input instruction is always the CPU's own id.
                id
            } else if let Some(y) = inbound_packet_y.take() {
                // If there's a partially-digested inbound packet, use that.
                y
            } else if let Ok(packet) = self_.inbound_msgs.try_recv() {
                // Note that we unblock the CPU (possibly incrementing activity_count) _before_
                // "consuming" the message (decrementing activity_count).
                //
                // This ensures the activity_count doesn't hit 0 accidentally,
                // when the system isn't actually deadlocked.
                self_.unblock();
                self_.activity_count.decr(); // -1 message in flight.

                // Receive a packet; save the second half for later.
                inbound_packet_y = Some(packet.y);
                packet.x
            } else {
                self_.block();
                -1
            }
        }
    }

    /// How many consecutive polls until we consider the CPU blocked.
    ///
    /// By unscientific trial-and-error, the correct threshold appears to be 2.
    const INACTIVE_THRESH: u64 = 2;

    /// Called when the CPU polls the message queue and receives nothing.
    ///
    /// See `Self.num_consecutive_polls`.
    fn block(&mut self) {
        self.num_consecutive_polls += 1;

        // At this point, we declare the CPU to be blocked.
        if self.num_consecutive_polls == Self::INACTIVE_THRESH {
            self.activity_count.decr(); // -1 active CPU (this one).
        }
    }

    /// Called when the CPU receives a message.
    ///
    /// See `Self::block`.
    fn unblock(&mut self) {
        if self.num_consecutive_polls >= Self::INACTIVE_THRESH {
            self.activity_count.incr(); // +1 active CPU (this one).
        }

        self.num_consecutive_polls = 0;
    }

    /// Returns a closure that gets called when the CPU produces output.
    ///
    /// Handles outgoing messages.
    fn handle_cpu_output(self_: Rc<RefCell<Self>>) -> impl FnMut(i64) {
        let mut outbound_packet = Vec::with_capacity(3);

        move |val| {
            let mut self_ = self_.borrow_mut();
            assert!(self_.num_consecutive_polls < Self::INACTIVE_THRESH,
                    "CPU {} sent a message when we thought it was blocked.", self_.id);

            debug_assert!(outbound_packet.len() < 3);
            outbound_packet.push(val);

            // Finished packet; send it.
            if outbound_packet.len() == 3 {
                let (addr, x, y) = outbound_packet.drain(..).collect_tuple().unwrap();

                let dest = if 0 <= addr && addr < NUM_CPUS as i64 {
                    // Note that messages to the NAT don't count towards `activity_count`.
                    self_.activity_count.incr(); // +1 message in flight.

                    &mut self_.network.cpus[addr as usize]
                } else {
                    assert_eq!(addr, NAT_ADDR);
                    &mut self_.network.nat
                };

                dest.send(Packet { x, y }).unwrap();
            }
        }
    }
}
