use std::{
    cell::RefCell,
    pin::Pin,
    rc::Rc,
};

use cohort::Cohort;

use crate::{
    runtime::{
        fail::Fail,
        queue::{
            IoQueue,
            IoQueueTable,
        },
    },
    QDesc,
    QType,
};

//======================================================================================================================
// Constants
//======================================================================================================================

/// Capacity of the ring buffer, in bytes.
/// This does not correspond to the effective number of bytes that may be stored in the ring buffer due to layout and
/// padding. Still, this is intentionally set so as the effective capacity is large enough to hold 16 KB of data.
const RING_BUFFER_CAPACITY: usize = 65536;

//======================================================================================================================
// Structures
//======================================================================================================================

struct CohortQueue {
    cohort: Pin<Box<Cohort<u64>>>,
}

pub struct CattailLibOS {
    qtable: Rc<RefCell<IoQueueTable<CohortQueue>>>,
}

//======================================================================================================================
// Trait Implementations
//======================================================================================================================

impl IoQueue for CohortQueue {
    fn get_qtype(&self) -> QType {
        QType::CohortQueue
    }
}

//======================================================================================================================
// Associated Functions
//======================================================================================================================

impl CohortQueue {
    fn new(cohort: Pin<Box<Cohort<u64>>>) -> Self {
        CohortQueue { cohort }
    }
}

impl CattailLibOS {
    /// Instantiates a new LibOS.
    pub fn new() -> Self {
        CattailLibOS {
            qtable: Rc::new(RefCell::new(IoQueueTable::new())),
        }
    }

    /// Creates a new cohort queue.
    pub fn create_cohort(&mut self, c_id: u8) -> Result<QDesc, Fail> {
        trace!("create_cohort()");
        let cohort = Cohort::register(c_id, RING_BUFFER_CAPACITY);
        let qd = self.qtable.borrow_mut().alloc(CohortQueue::new(cohort));
        Ok(qd)
    }

    /// Pushes data to cohort.
    pub fn push(&mut self, qd: QDesc, elem: u64) -> Result<(), Fail> {
        trace!("push() qd={:?}", qd);
        match self.qtable.borrow().get(&qd) {
            Some(queue) => {
                queue.cohort.push(elem);
                Ok(())
            },
            None => {
                let cause: String = format!("invalid queue descriptor (qd={:?})", qd);
                error!("push(): {}", cause);
                Err(Fail::new(libc::EBADF, &cause))
            },
        }
    }

    /// Pops data from cohort.
    pub fn pop(&mut self, qd: QDesc) -> Result<u64, Fail> {
        trace!("pop() qd={:?}", qd);
        match self.qtable.borrow().get(&qd) {
            Some(queue) => Ok(queue.cohort.pop()),
            None => {
                let cause: String = format!("invalid queue descriptor (qd={:?})", qd);
                error!("push(): {}", cause);
                Err(Fail::new(libc::EBADF, &cause))
            },
        }
    }

    /// Closes the cohort.
    pub fn close_cohort(&mut self, qd: QDesc) -> Result<(), Fail> {
        match self.qtable.borrow_mut().free(&qd) {
            Some(queue) => Ok(()),
            None => {
                let cause: String = format!("invalid queue descriptor (qd={:?})", qd);
                error!("close_cohort(): {}", cause);
                Err(Fail::new(libc::EBADF, &cause))
            },
        }
    }
}
