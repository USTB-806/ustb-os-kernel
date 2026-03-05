//! Process management syscalls
use alloc::sync::Arc;
use crate::{
    config::PAGE_SIZE, 
    app_loader::get_app_data_by_name, 
    mm::{copy_to_virt, translated_refmut, translated_str}, 
    task::{
        add_task, current_task, current_user_token, exit_current_and_run_next, mmap, munmap, 
        suspend_current_and_run_next,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct TaskInfo {
    pub status: usize,
    pub syscall_times: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("kernel:pid[{}] sys_exit", current_task().unwrap().pid.0);
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel:pid[{}] sys_yield", current_task().unwrap().pid.0);
    suspend_current_and_run_next();
    0
}

pub fn sys_getpid() -> isize {
    trace!("kernel: sys_getpid pid:{}", current_task().unwrap().pid.0);
    current_task().unwrap().pid.0 as isize
}

pub fn sys_fork() -> isize {
    trace!("kernel:pid[{}] sys_fork", current_task().unwrap().pid.0);
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    trace!("kernel:pid[{}] sys_exec", current_task().unwrap().pid.0);
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    trace!("kernel::pid[{}] sys_waitpid [{}]", current_task().unwrap().pid.0, pid);

    todo!("Lab 5: implement sys_waitpid")
}


pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!(
        "kernel:pid[{}] sys_get_time(ts: 0x{ts:x?})",
        current_task().unwrap().pid.0
    );
    let us = crate::timer::get_time_us();
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    copy_to_virt(&time_val, ts);
    0
}

pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!(
        "kernel:pid[{}] sys_mmap)(start: 0x{start:x}, len: 0x{len:x}, port: 0x{port:x})",
        current_task().unwrap().pid.0
    );
    const PORT_MASK: usize = 0b111;
     
    let aligned_start = start % PAGE_SIZE == 0;
    let port_valid = (port & !PORT_MASK) == 0;
    let port_not_none = (port & PORT_MASK) != 0;
     
    trace!("each condition: aligned_start={}, port_valid={}, port_not_none={}", aligned_start, port_valid, port_not_none);
    if aligned_start && port_valid && port_not_none {
        return mmap(start, len, port)
    }
    -1
}


pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!(
        "kernel:pid[{}] sys_munmap(start: 0x{start:x}, len: 0x{len:x})",
        current_task().unwrap().pid.0
    );
    let aligned_start = start % PAGE_SIZE == 0;
    if aligned_start {
        return munmap(start, len)
    }
    -1
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel:pid[{}] sys_sbrk", current_task().unwrap().pid.0);
    if let Some(old_brk) = current_task().unwrap().change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

pub fn sys_spawn(path: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_spawn(path: 0x{path:x?})",
        current_task().unwrap().pid.0
    );
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap().spwan(data);
        let new_pid = task.getpid();
        add_task(task);
        new_pid as isize
    } else {
        -1
    }
}


pub fn sys_set_priority(prio: isize) -> isize {
    debug!(
        "kernel:pid[{}] sys_set_priority(prio: {})",
        current_task().unwrap().pid.0,
        prio
    );
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if prio >= 2 {
        inner.priority = prio as usize;
        prio
    } else {
        -1
    }
}
