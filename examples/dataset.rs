use std::path::{Path, PathBuf};

use clap::Parser;
use perception_eval::{
    dataset, evaluation_task::EvaluationTask, frame_id::FrameID, logger::configure_logger,
};

#[derive(Debug, Parser)]
struct Args {
    #[structopt(short, long)]
    version: String,
    #[structopt(short, long)]
    data_root: PathBuf,
}

fn main() {
    let log_dir_name = format!("./data/{}", chrono::Local::now().format("%Y%m%d_%H%M%S"));
    let log_dir = Path::new(&log_dir_name);
    let _ret = configure_logger(log_dir, log::Level::Debug);
    let Args { version, data_root } = Args::parse();
    let evaluation_task = EvaluationTask::Detection;
    let frame_id = FrameID::BaseLink;

    let frame_ground_truths =
        dataset::load_dataset(version, data_root, &evaluation_task, &frame_id);

    println!("Number of frames: {:?}", frame_ground_truths.unwrap().len());
}
