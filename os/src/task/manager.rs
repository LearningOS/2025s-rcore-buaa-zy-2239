//!Implementation of [`TaskManager`]
use core::usize::MAX;

use super::{TaskControlBlock, TaskStatus};
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// pick the target process
    pub fn pick_next_task(&mut self) -> Option<Arc<TaskControlBlock>> {
        let mut min_stride = MAX;
        let mut next_task = None;
        for task in self.ready_queue.iter(){ 
            let task_inner = task.inner_exclusive_access();
            if task_inner.task_status == TaskStatus::Ready && task_inner.stride < min_stride 
            { 
                min_stride = task_inner.stride;
                next_task = Some(task.clone()); 
            } 
        }
        if let Some(index) = next_task {
            let index = self.ready_queue.iter().position(|t| Arc::ptr_eq(t, &index)).unwrap();
            let task = self.ready_queue.remove(index).unwrap();
            {
                let mut task_inner = task.inner_exclusive_access();
                task_inner.stride += task_inner.pass; 
            }
            Some(task)
        } else {
            None
        }
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue

/// pick the next process
pub fn pick_next_task()->Option<Arc<TaskControlBlock>>{
    TASK_MANAGER.exclusive_access().pick_next_task()
}