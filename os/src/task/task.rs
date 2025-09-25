//! Types related to task management

use super::TaskContext;
use crate::syscall::{SYSCALL_COUNT, SyscallCounter};

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The count of syscalls
    pub syscall_times: [SyscallCounter; SYSCALL_COUNT],
}

impl TaskControlBlock {
    /// Find the syscall counter for the given syscall id
    /// 
    /// # Arguments
    /// * `syscall_id` - The syscall ID to find the counter for
    /// 
    /// # Returns
    /// A mutable reference to the syscall counter
    pub fn find_syscall_counter(&mut self, syscall_id: usize) -> &mut SyscallCounter {
        if let Some(index) = self
            .syscall_times
            .iter()
            .position(|counter| counter.get_syscall_id() == syscall_id)
        {
            return &mut self.syscall_times[index];
        }
        if let Some(index) = self
            .syscall_times
            .iter()
            .position(|counter| counter.get_syscall_id() == 0)
        {
            self.syscall_times[index].set_syscall_id(syscall_id);
            return &mut self.syscall_times[index];
        }

        panic!("Syscall counter not found for syscall id: {}", syscall_id);
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
