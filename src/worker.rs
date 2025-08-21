// Note: want time to always increase and never pause, tokio pauses when thread is not running
use arrow::array::{Array, BooleanArray, Float64Array, Int64Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit::Microsecond};
use arrow::ipc::writer::FileWriter;
use arrow::record_batch::RecordBatch;
use std::fs::File;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

/// The worker is responsible for keeping track of all of a job's state, which
/// can be monitored by the supervisor.
#[derive(Debug)]
pub struct Worker {
    pub id: Uuid,
    pub task_id: Option<Uuid>,
    pub retry_count: u8,
    pub job_obtained_at: Option<Instant>,
    pub status: WorkerStatus,
    // FIXME: add payload after validating system
}

#[derive(Debug)]
pub enum WorkerStatus {
    WAITING = 1,
    PENDING = 2,
    RUNNING = 3,
    CANCELLED = 4,
    FAILED = 5,
    RETRYING = 6,
    COMPLETED = 7,
    CLEANING = 8,
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            id: Uuid::new_v4(),
            task_id: None,
            retry_count: 0,
            job_obtained_at: None,
            status: WorkerStatus::WAITING,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkerSchema {
    pub schema: Arc<Schema>,
}

impl WorkerSchema {
    pub fn new() -> Self {
        let schema = Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("retry_count", DataType::UInt8, false),
            Field::new("status", DataType::UInt8, false),
            Field::new("task_id", DataType::Utf8, true),
            Field::new("job_obtained_at", DataType::Duration(Microsecond), true),
        ]);

        Self {
            schema: Arc::new(schema),
        }
    }

    pub fn get_schema(&self) -> &Schema {
        &self.schema
    }

    pub fn create_record_batch(
        &self,
        data: Worker,
    ) -> Result<RecordBatch, Box<dyn std::error::Error>> {
        let batch = RecordBatch::try_new(
            self.schema.clone(),
            vec![
                Arc::new(data.id.to_string()),
                Arc::new(data.task_id),
                Arc::new(data.retry_count),
                Arc::new(data.job_obtained_at),
                Arc::new(data.status as u8),
            ],
        )?;
        Ok(batch)
    }
}
