use libc::syscall;

type Addr = u64;
type Len = u32;
type Ptr = Len;
type ElSize = u32;

#[repr(packed)]
pub struct Meta {
    addr: Addr,
    size: ElSize,
    len: Len,
}

#[repr(C)]
pub struct FifoCtrl {
    pub fifo_length: u32,
    pub element_size: u32,
    pub head_ptr: *mut *mut usize,
    pub tail_ptr: *mut *mut usize,
    pub meta: *mut Meta,
    pub data_array: *mut (),
}

unsafe fn cohort_register(acc_address: *mut(), sw_to_cohort_fifo: &FifoCtrl, cohort_to_sw_fifo: &FifoCtrl, backoff_counter_val: u32) -> i64 {
    unsafe {
        syscall(258, sw_to_cohort_fifo.head_ptr as u64, cohort_to_sw_fifo.head_ptr as u64, acc_address as u64, backoff_counter_val)
    }
}