use std::{
    thread,
    time::{Duration, Instant},
};

use quick_xml::{events::Event, Reader};

use crate::{Client, Error, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskStatus {
    Queued,
    PreRunning,
    Running,
    Success,
    Error,
    Aborted,
    Canceled,
    Unknown,
}

impl TaskStatus {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Success | Self::Error | Self::Aborted | Self::Canceled
        )
    }
}

impl From<&str> for TaskStatus {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "queued" => Self::Queued,
            "prerunning" => Self::PreRunning,
            "running" => Self::Running,
            "success" => Self::Success,
            "error" => Self::Error,
            "aborted" => Self::Aborted,
            "canceled" | "cancelled" => Self::Canceled,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub href: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub operation: Option<String>,
    pub status: TaskStatus,
    pub error_message: Option<String>,
}

impl Task {
    pub fn from_xml(xml: &str) -> Result<Self> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        let mut task = Self {
            href: None,
            id: None,
            name: None,
            operation: None,
            status: TaskStatus::Unknown,
            error_message: None,
        };

        loop {
            match reader.read_event()? {
                Event::Start(element) | Event::Empty(element) => {
                    let name = element.name();
                    if name.as_ref().ends_with(b"Task") {
                        for attr in element.attributes().flatten() {
                            let value = attr.unescape_value()?.into_owned();
                            match attr.key.as_ref() {
                                b"href" => task.href = Some(value),
                                b"id" => task.id = Some(value),
                                b"name" => task.name = Some(value),
                                b"operation" => task.operation = Some(value),
                                b"status" => task.status = TaskStatus::from(value.as_str()),
                                _ => {}
                            }
                        }
                    } else if name.as_ref().ends_with(b"Error") {
                        for attr in element.attributes().flatten() {
                            if attr.key.as_ref() == b"message" {
                                task.error_message = Some(attr.unescape_value()?.into_owned());
                            }
                        }
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }

        Ok(task)
    }

    pub fn refresh(&self, client: &Client) -> Result<Self> {
        let href = self.href.as_deref().ok_or(Error::Missing("task href"))?;
        client.get(href).send()?.task()
    }

    pub fn wait(&self, client: &Client, timeout: Duration, interval: Duration) -> Result<Self> {
        let deadline = Instant::now() + timeout;
        let mut task = self.clone();
        while !task.status.is_terminal() && Instant::now() < deadline {
            thread::sleep(interval);
            task = task.refresh(client)?;
        }

        match task.status {
            TaskStatus::Success => Ok(task),
            TaskStatus::Error | TaskStatus::Aborted | TaskStatus::Canceled => {
                Err(Error::TaskFailed(
                    task.error_message
                        .clone()
                        .unwrap_or_else(|| format!("task ended with status {:?}", task.status)),
                ))
            }
            _ => Err(Error::TaskFailed("task wait timed out".to_owned())),
        }
    }
}
