use std::{fmt, iter};
use std::ops::{Index, IndexMut};

pub struct Memory {
    buf: Vec<i64>,
}

impl Memory {
    pub fn new(program: Vec<i64>) -> Self {
        Self { buf: program }
    }

    /// Extend the buffer with 0s so that addr is in range.
    fn extend(&mut self, addr: usize) {
        let n = self.buf.len();

        if addr >= n {
            let extra_amount = addr - n + 1;
            self.buf.extend(iter::repeat(0).take(extra_amount));
        }
    }
}

impl Index<i64> for Memory {
    type Output = i64;

    fn index(&self, addr: i64) -> &i64 {
        let addr: usize = addr.try_into().unwrap();

        if addr < self.buf.len() {
            &self.buf[addr]
        } else {
            &0
        }
    }
}

impl IndexMut<i64> for Memory {
    fn index_mut(&mut self, addr: i64) -> &mut i64 {
        let addr: usize = addr.try_into().unwrap();

        self.extend(addr);
        &mut self.buf[addr]
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.buf)
    }
}
