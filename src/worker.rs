// Note: want time to always increase and never pause, tokio pauses when thread is not running
use std::time::Instant;
use uuid::Uuid;

/// The worker is responsible for keeping track of all of a job's state, which
/// can be monitored by the supervisor. All types likely to change to `Arrow`
/// compatible types.
#[derive(Debug)]
pub struct Worker {
    pub id: Uuid,
    pub task_id: Uuid,
    pub retry_count: u8,
    pub job_obtained_at: Option<Instant>,
    pub status: WorkerStatus,
    // FIXME: add payload when type is known
}

#[derive(Debug)]
pub enum WorkerStatus {
    WAITING,
    PENDING,
    RUNNING,
    CANCELLED,
    FAILED,
    RETRYING,
    COMPLETED,
    CLEANING,
}
