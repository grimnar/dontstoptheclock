// use std::ops::{Index, Deref};
use std::collections::VecDeque;

use rocket::serde::{Serialize};
use rocket::State;
use rocket::tokio::sync::Mutex;

pub type TimeStampList = Mutex<RingBuffer>;
pub type TimeStamps<'r> = &'r State<TimeStampList>;

#[derive(Copy, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TimeStamp { 
    pub id: u64,
    // UTC timestamp
    pub last_stop_ts: u64 
}

impl TimeStamp {
    pub fn new(id: u64, last_stop_ts: u64) -> Self {
        TimeStamp{id:id, last_stop_ts:last_stop_ts}
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RingBuffer {
    pub buf: VecDeque<TimeStamp>,
    capacity: usize,
    max_id: u64,
    index: usize
}

impl RingBuffer {
    pub fn new(capacity: usize, first: u64) -> Self {
        let mut result = RingBuffer{buf: VecDeque::new(), capacity: capacity, max_id: 0, index: 0};
        result.push(first);
        result
    }

    pub fn push(&mut self, value: u64) {
        while self.buf.len() > self.capacity - 1 {
            self.buf.pop_front();
        }
        self.buf.push_back(TimeStamp::new(self.max_id, value));
        self.max_id += 1;
    }

    pub fn last_item(&self) -> TimeStamp {
        let mut index: usize = self.max_id as usize;
        if self.max_id as usize > self.capacity {
            index = self.capacity;
        }
        self.buf[index - 1].clone()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ringbuffer_init() {
        let rbuf = RingBuffer::new(5, 123456789);
        assert_eq!(rbuf.len(), 1);
        assert_eq!(rbuf[0].id, 0);
        assert_eq!(rbuf[0].last_stop_ts, 123456789);
    }

    #[test]
    fn test_ringbuffer_push() {
        let mut rbuf = RingBuffer::new(5, 123456789);
        assert_eq!(rbuf.len(), 1);
        assert_eq!(rbuf[0].id, 0);
        assert_eq!(rbuf[0].last_stop_ts, 123456789);
        rbuf.push(987654321);
        assert_eq!(rbuf.len(), 2);
        assert_eq!(rbuf[0].id, 0);
        assert_eq!(rbuf[0].last_stop_ts, 123456789);
        assert_eq!(rbuf[1].id, 1);
        assert_eq!(rbuf[1].last_stop_ts, 987654321);
    }

    #[test]
    fn test_ringbuffer_last_item() {
        let mut rbuf = RingBuffer::new(5, 123456789);
        let last = rbuf.last_item();

        assert_eq!(rbuf.len(), 1);
        assert_eq!(last.id, 0);
        assert_eq!(last.last_stop_ts, 123456789);
        
    }

    #[test]
    fn test_ringbuffer_deref() {
        let mut rbuf = RingBuffer::new(5, 123456789);
        let res = &*rbuf;
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].last_stop_ts, 123456789);
    }

    #[test]
    fn test_ringbuffer_max_capacity() {
        let mut init_int = 987654321;
        let mut rbuf = RingBuffer::new(5, 123456789);
        for i in 1..5 {
            rbuf.push(init_int);
            init_int += 1;

        }
        assert_eq!(rbuf.len(), 5);
        assert_eq!(rbuf[0].last_stop_ts, 123456789);
        assert_eq!(rbuf[1].last_stop_ts, 987654321);
        assert_eq!(rbuf[2].last_stop_ts, 987654322);
        assert_eq!(rbuf[3].last_stop_ts, 987654323);
        assert_eq!(rbuf[4].last_stop_ts, 987654324);
        rbuf.push(init_int);
        assert_eq!(rbuf.buf.len(), 5);
        assert_eq!(rbuf[0].last_stop_ts, 987654321);
        assert_eq!(rbuf[1].last_stop_ts, 987654322);
        assert_eq!(rbuf[2].last_stop_ts, 987654323);
        assert_eq!(rbuf[3].last_stop_ts, 987654324);
        assert_eq!(rbuf[4].last_stop_ts, 987654325);

        init_int += 1;
        rbuf.push(init_int);
        init_int += 1;
        rbuf.push(init_int);
        assert_eq!(rbuf.buf.len(), 5);
        assert_eq!(rbuf[0].last_stop_ts, 987654323);
        assert_eq!(rbuf[1].last_stop_ts, 987654324);
        assert_eq!(rbuf[2].last_stop_ts, 987654325);
        assert_eq!(rbuf[3].last_stop_ts, 987654326);
        assert_eq!(rbuf[4].last_stop_ts, 987654327);
    }
}
