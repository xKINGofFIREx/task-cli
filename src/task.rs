extern crate chrono;

pub mod task {
    use chrono::{DateTime, Utc};

    struct Task {
        id: i64,
        description: String,
        status: Status,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    enum Status {
        Todo,
        InProgress,
        Done,
    }
}
