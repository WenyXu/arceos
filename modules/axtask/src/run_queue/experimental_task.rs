use crate::{AxRunQueue, IDLE_TASK, RUN_QUEUE};

impl AxRunQueue {
    pub fn new() -> SpinNoIrq<Self> {
        // 内核线程的page_table_token默认为0
        let gc_task = TaskInner::new(
            gc_entry,
            "gc",
            axconfig::TASK_STACK_SIZE,
            KERNEL_PROCESS_ID,
            0,
        );
        info!("gc task id: {}", gc_task.id().as_u64());
        unsafe { CurrentTask::init_current(gc_task) }
        let scheduler = Scheduler::new();
        // scheduler.add_task(gc_task);
        SpinNoIrq::new(Self { scheduler })
    }

    /// 仅用于exec时清除其他后台线程
    pub fn remove_task(&mut self, task: &AxTaskRef) {
        debug!("task remove: {}", task.id_name());
        // 当前任务不予清除
        assert!(task.is_running());
        assert!(!task.is_idle());
        self.scheduler.remove_task(task);
        task.set_state(TaskState::Exited);
        EXITED_TASKS.lock().push_back(task.clone());
    }

    pub fn resched_inner(&mut self, preempt: bool) {
        let prev = crate::current();
        let next = self.scheduler.pick_next_task().unwrap_or_else(|| unsafe {
            // Safety: IRQs must be disabled at this time.
            IDLE_TASK.current_ref_raw().get_unchecked().clone()
        });
        // 需要先把next取出来再取出prev，否则会出现prev和next一致导致死循环
        if prev.is_running() {
            prev.set_state(TaskState::Ready);
            if !prev.is_idle() {
                self.scheduler.put_prev_task(prev.clone(), preempt);
            }
        }
        self.switch_to(prev, next);
    }
}

pub(crate) fn init() {
    let idle_task = TaskInner::new(
        || run_idle(),
        "idle",
        IDLE_TASK_STACK_SIZE,
        KERNEL_PROCESS_ID,
        0,
    );
    idle_task.set_leader(true);
    IDLE_TASK.with_current(|i| i.init_by(idle_task.clone()));
    RUN_QUEUE.init_by(AxRunQueue::new());
}
