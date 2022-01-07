use std::sync::mpsc::Receiver;
use crate::solutions::day23::cpu::Network;
use crate::solutions::day23::deadlock::ActivityCount;
use crate::solutions::day23::Packet;

pub struct NAT {
    activity_count: ActivityCount,
    /// Receives a signal whenever a deadlock is detected.
    deadlock: Receiver<()>,
    incoming_msgs: Receiver<Packet>,
    network: Network,
}

impl NAT {
    pub fn new(activity_count: ActivityCount, deadlock: Receiver<()>, incoming_msgs: Receiver<Packet>, network: Network) -> Self {
        Self {
            activity_count,
            deadlock,
            incoming_msgs,
            network,
        }
    }

    pub fn run(mut self) -> i64 {
        let mut most_recent_packet = None;
        let mut prev_outgoing_y = None;

        loop {
            // Wait for deadlock.
            self.deadlock.recv().unwrap();

            // Catch up on recent messages.
            while let Ok(packet) = self.incoming_msgs.try_recv() {
                most_recent_packet = Some(packet);
            }
            let packet = most_recent_packet.expect("Deadlocked before NAT received any packets.");

            // Same y twice in a row?
            if prev_outgoing_y == Some(packet.y) {
                return packet.y;
            }
            prev_outgoing_y = Some(packet.y);

            // Hey! Wake up!
            self.activity_count.incr(); // +1 message in flight.
            self.network.cpus[0].send(packet).unwrap();
        }
    }
}
