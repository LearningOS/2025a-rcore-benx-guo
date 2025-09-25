//! Syscall counter

/// Syscall counter
#[derive(Debug, Clone, Copy)]
pub struct SyscallCounter {
    /// The syscall ID
    syscall_id: usize,
    /// The count of the syscall
    count: usize,
}

/// Syscall counter implementation
impl SyscallCounter {
    /// Create a new syscall counter
    /// 
    /// # Arguments
    /// * `syscall_id` - The syscall ID to create the counter for
    /// 
    /// # Returns
    /// A new syscall counter
    pub fn new(syscall_id: usize) -> Self {
        Self { syscall_id, count: 0 }
    }

    /// Increment the syscall counter
    /// 
    /// # Arguments
    /// * `syscall_id` - The syscall ID to increment the counter for
    /// 
    /// # Returns
    /// A new syscall counter
    pub fn increment(&mut self) {
        self.count += 1;
    }

    /// Get the count of the syscall counter
    /// 
    /// # Returns
    /// The count of the syscall counter
    pub fn get_count(&self) -> usize {
        self.count
    }

    /// Get the syscall ID of the syscall counter
    /// 
    /// # Returns
    /// The syscall ID of the syscall counter
    pub fn get_syscall_id(&self) -> usize {
        self.syscall_id
    }

    /// Set the syscall ID of the syscall counter
    /// 
    /// # Arguments
    /// * `syscall_id` - The syscall ID to set the counter for
    pub fn set_syscall_id(&mut self, syscall_id: usize) {
        self.syscall_id = syscall_id;
    }
}
