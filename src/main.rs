use bow::worker::{Worker, WorkerSchema};
use clap::Parser;

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
    let ws = WorkerSchema::new();
    let records = ws.create_record_batch((0..app_args.workers).map(|_| Worker::new()).collect());

    println!("Hello, workers: {:#?}", records);
    Ok(())
}
