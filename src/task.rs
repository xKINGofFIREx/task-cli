use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, serde::ts_seconds};

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub status: Status,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}
