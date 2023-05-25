use libc::syscall;

use crate::collections::ring::RingBuffer;

const BACKOFF_COUNTER_VAL: u64 = 240;
struct Cohort<'a, T> {
    sender: &'a RingBuffer<T>,
    receiver: &'a RingBuffer<T>,
}

impl<'a, T: Copy> Cohort<'a, T> {
    pub fn register(acc_address: usize, sender: &'a mut RingBuffer<T>, receiver: &'a mut RingBuffer<T>) -> Self {
        unsafe {
            libc::syscall(
                258,
                sender.get_front_ptr(),
                receiver.get_back_ptr(),
                acc_address as u64,
                BACKOFF_COUNTER_VAL,
            );
        }
        Cohort { sender, receiver }
    }

    /// Sends an element to the accelerator.
    pub fn push(elem: T) {
        todo!();
    }

    /// Reads an element from the accelerator.
    pub fn pop() -> T {
        todo!();
    }
}

impl<'a,T> Drop for Cohort<'a,T> {
    fn drop(&mut self) {
        // Unregister thorugh specific syscall.
    }
}
