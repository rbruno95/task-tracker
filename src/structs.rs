use chrono::{DateTime, Utc};

pub(crate) struct Task {
    pub(crate) id: usize,
    pub(crate) description: String,
    pub(crate) status: Status,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl Task {
    pub(crate) fn new(description: String) -> Self {
        Self {
            id: 1,
            description,
            status: Status::Todo,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

pub(crate) enum Status {
    Todo,
    InProgress,
    Done,
}
