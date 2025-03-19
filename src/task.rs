use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, serde::ts_seconds};

#[derive(Deserialize, Serialize)]
pub struct Task {
    id: u64,
    description: String,
    status: Status,

    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
enum Status {
    Todo,
    InProgress,
    Done,
}
