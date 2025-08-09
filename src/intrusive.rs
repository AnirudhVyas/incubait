use bytemuck::{Pod, Zeroable};
use std::ptr::null_mut;

/// Data Self organizes -> data knows what it links to.
pub static mut TASK_POOL: [Option<Task>; 1024] = [None; 1024];
#[repr(C, align(64))]
#[derive(Debug, Copy, Clone)]
pub struct Task {
    pub id: i32,
    pub link: *mut Task,
}
unsafe impl Pod for Task {}
unsafe impl Zeroable for Task {}
unsafe impl Send for Task {}
unsafe impl Sync for Task {}
impl Task {
    pub fn new() -> Self {
        Self {
            id: 0,
            link: std::ptr::null_mut(),
        }
    }
}

pub struct TaskQueue {
    pub head: *mut Task,
}
impl TaskQueue {
    pub fn allocate_task(&self, task: *mut Task) -> Option<&'static mut Task> {
        unsafe {
            for i in (0..1024) {
                if TASK_POOL[i].is_some() && TASK_POOL[i].unwrap().id == (*task).id {
                    tracing::warn!("task is already allocated");
                    return None;
                }
                if TASK_POOL[i].is_none() {
                    let content = std::mem::replace(&mut *task, Task::new());
                    TASK_POOL[i] = Some(content);
                    return TASK_POOL[i].as_mut();
                }
            }
        }
        None
    }
    pub fn new() -> Self {
        TaskQueue {
            head: std::ptr::null_mut(),
        }
    }
    pub fn push(&mut self, task: *mut Task) {
        if self.head == null_mut() {
            if let Some(t) = self.allocate_task(task) {
                self.head = t as *mut Task;
            }
        } else {
            if let Some(t) = self.allocate_task(task) {
                t.link = self.head;
                self.head = t
            }
        }
    }

    pub fn push_end(&mut self, task: *mut Task) {

    }
}
