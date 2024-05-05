//! Process management syscalls

use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE},
    mm::{self, translate},
    syscall::{
        SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_MMAP, SYSCALL_MUNMAP, SYSCALL_TASK_INFO,
        SYSCALL_YIELD,
    },
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
        TASK_MANAGER,
    },
    timer::get_time_ms,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code

pub fn sys_exit(_exit_code: i32) -> ! {
    TASK_MANAGER.inc_call_times(SYSCALL_EXIT);

    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    TASK_MANAGER.inc_call_times(SYSCALL_YIELD);
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    TASK_MANAGER.inc_call_times(SYSCALL_GET_TIME);
    trace!("kernel: sys_get_time");
    -1
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    TASK_MANAGER.inc_call_times(SYSCALL_TASK_INFO);
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    TASK_MANAGER.with_task_info(|inner, manager| {
        let translated = translate(manager.get_current_token(), ti);
        translated.status = inner.task_status;
        translated.syscall_times = inner.syscall_times;
        translated.time = get_time_ms() - inner.time;
    });

    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    TASK_MANAGER.inc_call_times(SYSCALL_MMAP);
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if start % PAGE_SIZE != 0 || port & !0x7 != 0 || port & 0x7 == 0 {
        return -1;
    }
    mm::alloc_virtual_memory(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    TASK_MANAGER.inc_call_times(SYSCALL_MUNMAP);
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if start % PAGE_SIZE != 0 {
        return -1;
    }
    mm::free_virtual_memory(start, len)
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
