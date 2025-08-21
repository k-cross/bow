use arrow::array::{DurationMicrosecondArray, StringArray, UInt8Array};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit::Microsecond};
use arrow::ipc::writer::FileWriter;
use arrow::record_batch::RecordBatch;
use std::fs::File;
use std::sync::Arc;
// Note: want time to always increase and never pause, tokio's `Instant` pauses when thread is not running
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

#[derive(Debug, Clone, Copy)]
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
            Field::new("task_id", DataType::Utf8, true),
            Field::new("status", DataType::UInt8, false),
            Field::new("job_obtained_at", DataType::Duration(Microsecond), true),
            Field::new("retry_count", DataType::UInt8, false),
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
        data: Vec<Worker>,
    ) -> Result<RecordBatch, Box<dyn std::error::Error>> {
        let id_array = StringArray::from(
            data.iter()
                .map(|x| x.id.to_string())
                .collect::<Vec<String>>(),
        );
        let task_id_array = StringArray::from(
            data.iter()
                .map(|x| x.task_id.and_then(|s| Some(s.to_string())))
                .collect::<Vec<Option<String>>>(),
        );
        let status_array = UInt8Array::from_iter(data.iter().map(|x| x.status as u8));
        let joa_array = DurationMicrosecondArray::from(
            data.iter()
                .map(|x| {
                    x.job_obtained_at
                        .and_then(|t| Some(t.elapsed().as_micros() as i64))
                })
                .collect::<Vec<Option<i64>>>(),
        );
        let retry_count_array = UInt8Array::from_iter(data.iter().map(|x| x.retry_count));
        let batch = RecordBatch::try_new(
            self.schema.clone(),
            vec![
                Arc::new(id_array),
                Arc::new(task_id_array),
                Arc::new(status_array),
                Arc::new(joa_array),
                Arc::new(retry_count_array),
            ],
        )?;
        Ok(batch)
    }
}
