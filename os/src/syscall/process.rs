//! Process management syscalls
use crate::{
    syscall::{SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_TRACE, SYSCALL_YIELD}, task::{exit_current_and_run_next, get_syscall_count, suspend_current_and_run_next,add_syscall_count}, timer::get_time_us
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    add_syscall_count(SYSCALL_EXIT);
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    add_syscall_count(SYSCALL_YIELD);
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    add_syscall_count(SYSCALL_GET_TIME);
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    add_syscall_count(SYSCALL_TRACE);
   if trace_request==0{
        unsafe { return *(id as *const u8) as isize; }
   }else if trace_request==1{
    unsafe { *(id as *mut u8) = data as u8;} 
        return 0;
     }else if trace_request==2{
        return get_syscall_count(id);
     }else{
        return -1;
    }
}
