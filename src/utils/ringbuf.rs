use std::{cmp::min, sync::atomic::{AtomicUsize, Ordering}};
use std::default::Default;
use core::clone::Clone;
use std::sync::Arc;

struct CacheLine {
    inner: usize,
}

/// head: push
/// 
/// tail: push
/// 
/// Sacrifice a memory unit for full judgement
struct RingBuf<T> {
    head: CacheLine,
    tail: CacheLine,
    container: Vec::<T>,
    cap: usize,
}

struct Producer<T> {
    inner: Arc<RingBuf<T>>
}

struct Consumer<T> {
    inner: Arc<RingBuf<T>>
}

impl CacheLine {
    fn new(x: usize) -> Self {
        CacheLine {
            inner: x
        }
    }
}

impl<T: Clone + Default> RingBuf<T> {
    fn new() -> Self {
        RingBuf {
            head: CacheLine::new(0),
            tail: CacheLine::new(0),
            container: Vec::<T>::new(),
            cap: 0
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        assert!(capacity <= usize::MAX && capacity >= usize::MIN);

        let mut buffer = Vec::<T>::new();
        for _ in 0..capacity {
            buffer.push(T::default());
        }
        RingBuf {
            head: CacheLine::new(0),
            tail: CacheLine::new(0),
            container: buffer,
            cap: capacity - 1,
        }
    }

    fn vacant_read_size(&self) -> usize {
        if self.head.inner > self.tail.inner {
            self.head.inner - self.tail.inner
        } else if self.head.inner == self.tail.inner {
            0
        } else {
            self.head.inner + self.cap - self.tail.inner
        }
    }

    fn vacant_write_size(&self) -> usize {
        self.cap - self.vacant_read_size()
    }

    fn try_push(&mut self, data: T) -> Option<T> {
        let cur_head = self.head.inner;
        let mut head_next = cur_head + 1;
        
        if head_next >= self.cap {
            head_next = 0;
        }
        if head_next == self.tail.inner {
            println!("error: the ringbuffer is full!");
            Some(data)
        } else {
            self.container[cur_head] = data;
            self.head.inner = head_next;
            None
        }
    }

    fn push_batch(&mut self, src: &mut Vec<T>, batch_size: usize) -> usize {
        if batch_size <= 0 {
            println!("Error: the batch size is smaller than 1!");
            0
        } else {
            let actual_size = min(self.vacant_write_size(), batch_size);
            let drain = src.drain(0..actual_size);
            for data in drain {
                self.try_push(data);
            }
            actual_size
        }
        
    }

    fn rcv_betch(&mut self, dst: &mut Vec<T>, batch_size: usize) -> usize {
        if batch_size <= 0 {
            println!("Error: the batch size is smaller than 1!");
            0
        } else {
            let actual_size = min(self.vacant_read_size(), batch_size);
            for _ in 0..actual_size {
                let t = self.try_pop().expect("Error: pop wrong!");
                dst.push(t);
            }
            actual_size
        }
    }

    fn try_pop(&mut self) -> Option<T> {
        let cur_tail = self.tail.inner;
        let mut tail_next = cur_tail + 1;

        if tail_next >= self.cap {
            tail_next = 0;
        }
        if cur_tail == self.head.inner {
            println!("error: the ringbuffer is empty!");
            None
        } else {
            self.tail.inner = tail_next;
            Some(self.container[cur_tail].clone())
        }
    }
}


#[test]
fn push_pop_test() {
    let mut buffer = RingBuf::<usize>::with_capacity(10);
    for i in 0..10 {
        let result = buffer.try_push(i);
    }
    for i in 0..10 {
        let result = buffer.try_pop();
    }
}

#[test]
fn full_empty__check() {
    let mut buffer = RingBuf::<usize>::with_capacity(1);
    buffer.try_push(1);
    assert_eq!(buffer.try_push(1), Some(1));

    buffer.try_pop();
    assert_eq!(buffer.try_pop(), None);
}
