use std::{io, iter, thread};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use crate::intcode_computer::{IntcodeComputer, read_intcode_program};
use itertools::Itertools;

const NUM_CPUS: usize = 50;
const MAGIC_ADDR: i64 = 255;

pub fn main() {
    let prog = read_intcode_program(io::stdin().lock());

    println!("{}", part_1(prog.clone()));
    // println!("{}", part_2(prog));
}

fn part_1(prog: Vec<i64>) -> i64 {
    // Each thread has its own queue of incoming packets.
    let channels: Vec<_> = iter::repeat_with(mpsc::channel).take(NUM_CPUS).collect();
    let (senders, receivers): (Vec<_>, Vec<_>) = channels.into_iter().unzip();

    // One extra channel, for the NAT.
    let (nat_sx, nat_rx) = mpsc::channel();

    let network = Network { cpus: senders, nat: nat_sx };

    let cpus = receivers.into_iter().enumerate().map(|(id, rx)| {
        let cpu = IntcodeComputer::new(prog.clone());
        CPU { id, cpu, rx, network: network.clone() }
    });

    // Note that we don't bother to join or kill the threads.
    for cpu in cpus {
        thread::spawn(move || {
            cpu.run();
        });
    }

    // Wait for the magic packet.
    nat_rx.recv().unwrap().y
}

/// A networked computer, running the NIC software.
struct CPU<'a> {
    id: usize,
    cpu: IntcodeComputer<'a, 'a>,

    network: Network,
    rx: Receiver<Packet>,
}

/// Handles to send packets to each computer.
#[derive(Clone)]
struct Network {
    cpus: Vec<Sender<Packet>>,
    nat: Sender<Packet>,
}

struct Packet {
    x: i64,
    y: i64,
}

impl<'a> CPU<'a> {
    fn run(mut self) {
        self.cpu.io(
            &mut Self::cpu_input(self.id, &mut self.rx),
            &mut Self::cpu_output(&mut self.network),
        ).run();
    }

    fn cpu_input<'b>(id: usize, rx: &'b mut Receiver<Packet>) -> impl FnMut() -> i64 + 'b {
        let mut id = Some(id as i64);
        let mut inbound_packet_y = None;

        move || {
            if let Some(id) = id.take() {
                // First input instruction is always the CPU's own id.
                id
            } else if let Some(y) = inbound_packet_y.take() {
                // If there's a partially-digested inbound packet, use that.
                y
            } else if let Ok(packet) = rx.try_recv() { // Note the _try_!
                // Receive a packet; save the second half for later.
                inbound_packet_y = Some(packet.y);
                packet.x
            } else {
                // Never block waiting for input.
                -1
            }
        }
    }

    fn cpu_output<'b>(network: &'b mut Network) -> impl FnMut(i64) + 'b {
        let mut outbound_packet = Vec::with_capacity(3);

        move |val| {
            debug_assert!(outbound_packet.len() < 3);
            outbound_packet.push(val);

            // Finished packet; send it.
            if outbound_packet.len() == 3 {
                let (addr, x, y) = outbound_packet.drain(..).collect_tuple().unwrap();

                let target = if 0 <= addr && addr < NUM_CPUS as i64 {
                    &mut network.cpus[addr as usize]
                } else {
                    assert_eq!(addr, MAGIC_ADDR);
                    &mut network.nat
                };

                target.send(Packet { x, y }).unwrap();
            }
        }
    }
}
