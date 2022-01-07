use std::{io, iter, thread};
use std::sync::mpsc;
use crate::intcode_computer::read_intcode_program;
use crate::solutions::day23::cpu::{CPU, Network};
use crate::solutions::day23::nat::NAT;

mod cpu;
mod nat;
mod deadlock;

const NUM_CPUS: usize = 50;
const NAT_ADDR: i64 = 255;

#[derive(Copy, Clone)]
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
    let (activity_count, deadlock_signal) = deadlock::tracker(NUM_CPUS);

    let cpus = receivers.into_iter().enumerate().map(|(id, rx)| {
        CPU::new(id, rx, network.clone(), activity_count.clone())
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
        NAT::new(activity_count, deadlock_signal, nat_rx, network).run()
    }
}
