use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, serde::ts_seconds};
use std::fmt::{Display, Formatter, Result};

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

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Display for Status {

    fn fmt(&self, f: &mut Formatter) -> Result{
        match self {
            Status::Todo => write!(f, "TODO"),
            Status::Done => write!(f, "DONE"),
            Status::InProgress => write!(f, "IN_PROGRESS"),
        }
    }
}
