use std::{io, iter, thread};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use crate::intcode_computer::{IntcodeComputer, read_intcode_program};
use itertools::Itertools;

const NUM_CPUS: usize = 50;

/// A packet will be sent to this address with the puzzle answer.
const MAGIC_ADDR: i64 = 255;

pub fn main() {
    let prog = read_intcode_program(io::stdin().lock());

    // Each thread has its own queue of incoming packets.
    let channels: Vec<_> = iter::repeat_with(mpsc::channel).take(NUM_CPUS).collect();
    let (senders, receivers): (Vec<_>, Vec<_>) = channels.into_iter().unzip();

    // One extra channel, for reporting the puzzle answer.
    let (answer_sx, answer_rx) = mpsc::channel();

    let nics = receivers.into_iter().enumerate().map(|(id, rx)| {
        let cpu = IntcodeComputer::new(prog.clone());

        NIC { id, cpu, rx, network: senders.clone(), answer_sx: answer_sx.clone() }
    });

    for nic in nics {
        thread::spawn(move || {
            nic.run();
        });
    }

    let ans = answer_rx.recv().unwrap();
    println!("{}", ans);
}

struct NIC<'a> {
    id: usize,
    cpu: IntcodeComputer<'a>,
    rx: Receiver<Packet>,
    network: Vec<Sender<Packet>>,
    answer_sx: Sender<i64>,
}

struct Packet {
    x: i64,
    y: i64,
}

impl NIC<'_> {
    fn run(mut self) {
        let mut id = Some(self.id as i64);
        let mut inbound_packet_y = None;

        let mut input = move || {
            if let Some(id) = id.take() {
                // First input instruction is always the NIC's own id.
                id
            } else if let Some(y) = inbound_packet_y.take() {
                // If there's a partially-digested inbound packet, use that.
                y
            } else if let Ok(packet) = self.rx.try_recv() { // Note the _try_!
                // Receive a packet; save the second half for later.
                inbound_packet_y = Some(packet.y);
                packet.x
            } else {
                // Never block waiting for input.
                -1
            }
        };

        let mut outbound_packet = Vec::with_capacity(3);

        let mut output = move |i| {
            assert!(outbound_packet.len() < 3);
            outbound_packet.push(i);

            // Send it.
            if outbound_packet.len() == 3 {
                let (addr, x, y) = outbound_packet.drain(..).collect_tuple().unwrap();

                if 0 <= addr && addr < NUM_CPUS as i64 {
                    self.network[addr as usize].send(Packet { x, y }).unwrap();
                } else {
                    assert_eq!(addr, MAGIC_ADDR);
                    self.answer_sx.send(y).unwrap();
                }
            }
        };

        self.cpu.run_io(&mut input, &mut output);
    }
}
