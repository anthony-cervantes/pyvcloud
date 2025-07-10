//! Task monitoring utilities mirroring the Python SDK.

use crate::types::TaskStatus;
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Representation of a vCloud Director task.
#[derive(Debug, Clone)]
pub struct Task {
    pub href: String,
    pub status: TaskStatus,
    pub error: Option<String>,
}

/// Trait used by [`TaskMonitor`] to fetch updated task information.
pub trait TaskClient {
    fn get_task(&self, href: &str) -> Task;
}

/// Monitor that waits for tasks to reach a desired status.
#[derive(Debug, Clone)]
pub struct TaskMonitor<C: TaskClient> {
    client: C,
}

impl<C: TaskClient> TaskMonitor<C> {
    /// Create a new monitor using the given client.
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Wait for the task to complete successfully.
    pub fn wait_for_success(
        &self,
        task_href: &str,
        timeout: Duration,
        poll_frequency: Duration,
    ) -> Result<Task, TaskMonitorError> {
        self.wait_for_status(
            task_href,
            timeout,
            poll_frequency,
            &[TaskStatus::Error, TaskStatus::Canceled, TaskStatus::Aborted],
            &[TaskStatus::Success],
            None::<fn(&Task)>,
        )
    }

    /// Wait for the task to reach one of the expected statuses.
    pub fn wait_for_status<F>(
        &self,
        task_href: &str,
        timeout: Duration,
        poll_frequency: Duration,
        fail_on_statuses: &[TaskStatus],
        expected_target_statuses: &[TaskStatus],
        mut callback: Option<F>,
    ) -> Result<Task, TaskMonitorError>
    where
        F: FnMut(&Task),
    {
        let start = Instant::now();
        loop {
            let task = self.client.get_task(task_href);
            if let Some(ref mut cb) = callback {
                cb(&task);
            }
            if expected_target_statuses.contains(&task.status) {
                return Ok(task);
            }
            if fail_on_statuses.contains(&task.status) {
                return Err(TaskMonitorError::Failed(task.status));
            }
            if start.elapsed() > timeout {
                return Err(TaskMonitorError::Timeout);
            }
            sleep(poll_frequency);
        }
    }
}

/// Errors returned by [`TaskMonitor`].
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TaskMonitorError {
    #[error("task timed out")]
    Timeout,
    #[error("task failed with status {0}")]
    Failed(TaskStatus),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct DummyClient {
        statuses: Arc<Mutex<Vec<TaskStatus>>>,
    }

    impl DummyClient {
        fn new(statuses: Vec<TaskStatus>) -> Self {
            Self {
                statuses: Arc::new(Mutex::new(statuses)),
            }
        }
    }

    impl TaskClient for DummyClient {
        fn get_task(&self, _href: &str) -> Task {
            let mut statuses = self.statuses.lock().unwrap();
            let status = if statuses.is_empty() {
                TaskStatus::Running
            } else {
                statuses.remove(0)
            };
            Task {
                href: "task".into(),
                status,
                error: None,
            }
        }
    }

    #[test]
    fn waits_for_success() {
        let client = DummyClient::new(vec![
            TaskStatus::Queued,
            TaskStatus::Running,
            TaskStatus::Success,
        ]);
        let monitor = TaskMonitor::new(client);
        let task = monitor
            .wait_for_success("task", Duration::from_secs(1), Duration::from_millis(1))
            .unwrap();
        assert_eq!(task.status, TaskStatus::Success);
    }

    #[test]
    fn fails_on_error() {
        let client = DummyClient::new(vec![TaskStatus::Queued, TaskStatus::Error]);
        let monitor = TaskMonitor::new(client);
        let err = monitor
            .wait_for_success("task", Duration::from_secs(1), Duration::from_millis(1))
            .unwrap_err();
        assert_eq!(err, TaskMonitorError::Failed(TaskStatus::Error));
    }

    #[test]
    fn times_out() {
        let client = DummyClient::new(vec![TaskStatus::Running, TaskStatus::Running]);
        let monitor = TaskMonitor::new(client);
        let err = monitor
            .wait_for_success("task", Duration::from_millis(5), Duration::from_millis(1))
            .unwrap_err();
        assert_eq!(err, TaskMonitorError::Timeout);
    }
}
