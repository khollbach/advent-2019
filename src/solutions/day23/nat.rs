use std::collections::HashSet;
use std::hint;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::mpsc::Receiver;
use crate::solutions::day23::cpu::Network;
use crate::solutions::day23::Packet;

pub struct NAT {
    /// Used to detect deadlocks. See `CPU.activity_count`.
    activity_count: Arc<AtomicU64>,
    incoming_msgs: Receiver<Packet>,
    most_recent_packet: Option<Packet>,
    outgoing_y_values: HashSet<i64>,
    network: Network,
}

impl NAT {
    pub fn new(activity_count: Arc<AtomicU64>, incoming_msgs: Receiver<Packet>, network: Network) -> Self {
        NAT {
            activity_count,
            incoming_msgs,
            most_recent_packet: None,
            outgoing_y_values: HashSet::new(),
            network,
        }
    }

    pub fn run(mut self) -> i64 {
        loop {
            // Busy wait, checking for deadlock.
            // todo: use a cond var instead.
            while self.activity_count.load(SeqCst) != 0 {
                hint::spin_loop();
            }

            // Catch up on recent messages.
            while let Ok(packet) = self.incoming_msgs.try_recv() {
                self.most_recent_packet = Some(packet);
            }

            // Repeated y?
            let packet = self.most_recent_packet.expect("Deadlocked before NAT received any packets.");
            if !self.outgoing_y_values.insert(packet.y) {
                return packet.y;
            }

            // Hey! Wake up!
            self.activity_count.fetch_add(1, SeqCst); // +1 message in flight.
            self.network.cpus[0].send(packet).unwrap();
        }
    }
}
