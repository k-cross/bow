use bow::worker::{Worker, WorkerStatus};
use clap::Parser;
use uuid::Uuid;

#[derive(Debug, Parser)]
#[clap(about, version, long_about=None)]
struct BowArgs {
    /// The number of independent workers to start on the system
    #[clap(short, long)]
    workers: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_args = BowArgs::parse();
    let mut workers: Vec<Worker> = (0..app_args.workers)
        .map(|_| Worker {
            id: Uuid::new_v4(),
            task_id: Uuid::new_v4(),
            retry_count: 0,
            status: WorkerStatus::WAITING,
            job_obtained_at: None,
        })
        .collect();

    println!("Hello, workers: {:#?}", workers);
    Ok(())
}
