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
