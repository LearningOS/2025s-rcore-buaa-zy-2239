//! Process management syscalls



use crate::{config::PAGE_SIZE, mm::{ translated_byte_buffer, PTEFlags, PageTable, VirtAddr}, task::{change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_count, insert_framed_area, suspend_current_and_run_next}, timer::get_time_us};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us=get_time_us();
    let new_ts=TimeVal{
        sec: us/1_000_000,
        usec: us%1_000_000,
    };
    let buffers = translated_byte_buffer(current_user_token(), ts as *const u8, core::mem::size_of::<TimeVal>());
    let timeval_bytes = unsafe {
        core::slice::from_raw_parts(
            &new_ts as *const TimeVal as *const u8,
            core::mem::size_of::<TimeVal>()
        )
    };

    let mut offset = 0;
    for buffer in buffers {
        let len = buffer.len().min(timeval_bytes.len() - offset);
        buffer[..len].copy_from_slice(&timeval_bytes[offset..offset + len]);
        offset += len;
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    let token=current_user_token();
    let page_table=PageTable::from_token(token);
    let addr = id as *const u8;
    let va = VirtAddr::from(addr as usize);
    let vpn = va.floor();
    let pte = match page_table.translate(vpn) {
        Some(pte) => pte,  // 成功找到 PTE
        None => {
            return -1;
        }
    };
    if trace_request==0{
        if (!pte.readable()) || ((pte.flags() & PTEFlags::U) == PTEFlags::empty()){
            return -1;
        }
        let buffers=translated_byte_buffer(token,addr, 1);
        if buffers.is_empty(){
            return -1;
        }
        return buffers[0][0] as isize;
   }else if trace_request==1{
        if (!pte.writable()) || ((pte.flags() & PTEFlags::U) == PTEFlags::empty()){
            return -1;
        }
        let mut buffers=translated_byte_buffer(token, addr, 1);
        if buffers.is_empty(){
            return -1;
        }
        buffers[0][0]=data as u8;
        return 0;
     }else if trace_request==2{
        return get_syscall_count(id);
     }else{
        return -1;
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let page_table=PageTable::from_token(current_user_token());
    if start%PAGE_SIZE!=0{
        return -1;
    }
    if (prot & !0x7!=0) || prot &0x7==0{
        return -1;
    }
    let num_pages=(len+PAGE_SIZE-1)/PAGE_SIZE;
    if num_pages==0{
        return 0;
    }
    for i in 0..num_pages{
        let vpn=VirtAddr::from(start+i*PAGE_SIZE).floor();
        if page_table.translate(vpn).is_some(){
            //println!("{:?} have mapped into {:?}",vpn,page_table.translate(vpn).unwrap().ppn());
            if page_table.translate(vpn).unwrap().is_valid(){
                 return -1;
            }
           
        }
    }
    insert_framed_area(start, len, prot);
    return 0;
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let mut page_table=PageTable::from_token(current_user_token());
    if start%PAGE_SIZE!=0{
        return -1;
    }
    let num_pages=(len+PAGE_SIZE-1)/PAGE_SIZE;
    if num_pages==0{
        return 0;
    }
    for i in 0..num_pages {
        let vpn = VirtAddr::from(start + i * PAGE_SIZE).into();
        if page_table.translate(vpn).is_none() || !page_table.translate(vpn).unwrap().is_valid(){
            return -1; 
        }
    }
    for i in 0..num_pages{
        let vpn=VirtAddr::from(start+i*PAGE_SIZE).into();
        page_table.unmap(vpn);
    }
    0
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
