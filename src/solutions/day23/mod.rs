use std::{io, iter, thread};
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, mpsc};
use crate::intcode_computer::read_intcode_program;
use crate::solutions::day23::cpu::{CPU, Network};
use crate::solutions::day23::nat::NAT;

mod cpu;
mod nat;

const NUM_CPUS: usize = 50;
const NAT_ADDR: i64 = 255;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Packet {
    x: i64,
    y: i64,
}

pub fn main() {
    let prog = read_intcode_program(io::stdin().lock());

    println!("{}", solve(prog.clone(), false));
    println!("{}", solve(prog, true));
}

fn solve(prog: Vec<i64>, part_2: bool) -> i64 {
    // Each thread has its own queue of incoming packets.
    let channels = iter::repeat_with(mpsc::channel).take(NUM_CPUS);
    let (senders, receivers): (Vec<_>, Vec<_>) = channels.into_iter().unzip();

    // One extra channel, for the NAT.
    let (nat_sx, nat_rx) = mpsc::channel();

    let network = Network { cpus: senders, nat: nat_sx };

    // This is the number of messages in flight plus the number of unblocked CPUs.
    // Initially, all CPUs are active.
    let activity_count = Arc::new(AtomicU64::new(NUM_CPUS as u64));

    let cpus = receivers.into_iter().enumerate().map(|(id, rx)| {
        let activity_count = Arc::clone(&activity_count);
        CPU::new(id, rx, network.clone(), activity_count)
    });

    // Note that we don't bother to join or kill the threads.
    for cpu in cpus {
        let prog = prog.clone();
        thread::spawn(move || {
            cpu.run(prog);
        });
    }

    if !part_2 {
        // Wait for the magic packet.
        nat_rx.recv().unwrap().y
    } else {
        NAT::new(activity_count, nat_rx, network).run()
    }
}
