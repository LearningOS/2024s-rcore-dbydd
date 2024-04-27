//! Process management syscalls

use crate::{
    config::MAX_SYSCALL_NUM,
    syscall::{SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_TASK_INFO, SYSCALL_YIELD},
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER},
    timer::{get_time_ms, get_time_us},
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

pub fn sys_exit(exit_code: i32) -> ! {
    TASK_MANAGER.inc_call_times(SYSCALL_EXIT);

    trace!("[kernel] Application exited with code {}", exit_code);
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

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    TASK_MANAGER.inc_call_times(SYSCALL_GET_TIME);
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

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    TASK_MANAGER.inc_call_times(SYSCALL_TASK_INFO);
    trace!("kernel: sys_task_info");
    let fetch_task_info = TASK_MANAGER.fetch_task_info();
    unsafe {
        *ti = TaskInfo {
            status: fetch_task_info.task_status,
            syscall_times: fetch_task_info.syscall_times,
            time: get_time_ms() - fetch_task_info.time,
        }
    }

    0
}
