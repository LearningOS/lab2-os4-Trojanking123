//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER, get_task_info_inner};
use crate::timer::get_time_us;
use crate::mm::translated_byte_buffer;
use crate::task::current_user_token;
use crate::mm::PageTable;
use crate::mm::VirtAddr;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}



// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    // let page_table = PageTable::from_token(current_user_token());
    // let mut start = _tz as usize;
    // let start_va = VirtAddr::from(start);
    // let   end_va = VirtAddr::from(start + core::mem::size_of::<TimeVal>()) ;
    // let mut vpn = start_va.floor();
    // let ppn = page_table.translate(vpn).unwrap().ppn();
    // let ts = &mut ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()];
    // let ts = ts.as_mut_ptr() as *mut TimeVal;
    // info!("tv virt ptr: {:?}", ts);
    // let us = get_time_us();
    // unsafe {
    //     *ts = TimeVal {
    //         sec: us / 1_000_000,
    //         usec: us % 1_000_000,
    //     };
    // }

    
    let mut v = translated_byte_buffer( current_user_token(), _ts as *const u8, core::mem::size_of::<TimeVal>());
    let us = get_time_us();
    let ts;
    if v.len() == 1 {
        let a = v[0].as_mut_ptr();
        //info!("a as ptr: {:?}", a);
        let a: *mut TimeVal  = unsafe { core::mem::transmute(a) };
        ts = a as *mut TimeVal;
        
        unsafe {
            *ts = TimeVal {
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            };
        }
    }else {
        error!("cross two page !!!!!");
    }

    
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    -1
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    -1
}

// YOUR JOB: 引入虚地址后重写 sys_task_info


pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    //info!("call task info api");
    //info!("ti virt ptr: {:?}", ti as usize);

    // let page_table = PageTable::from_token(current_user_token());
    // let mut start = ti as usize;
    // let start_va = VirtAddr::from(start);
    // let mut vpn = start_va.floor();
    // let ppn = page_table.translate(vpn).unwrap().ppn();
    // let ti = ppn.get_mut::<TaskInfo>() as *mut TaskInfo;
    // println!("ti ptr: {:?}", ti as usize);
    
     
    let ll = core::mem::size_of::<TaskInfo>();
    info!("ll: {:?}", ll);
    let mut v = translated_byte_buffer( current_user_token(), ti as *const u8, ll);
    if v.len() == 1 {
        //info!("len of task vec is 1 !");
        let a = v[0].as_mut_ptr();
        info!("taskinfo a ptr {:?}", a);
        let ti: *mut TaskInfo  = unsafe { core::mem::transmute(a) };
        //info!("taskinfo ti ptr {:?}", a);
        //info!("before inner");
        get_task_info_inner(ti);
    }else {
        error!("cross two page !!!!!");
        panic!("!!!!");
    }
    
    
    //get_task_info_inner(ti) ;
    
    0
}
