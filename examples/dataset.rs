use std::path::PathBuf;

use clap::Parser;
use perception_eval::{dataset, evaluation_task::EvaluationTask, frame_id::FrameID};

#[derive(Debug, Parser)]
struct Args {
    #[structopt(short, long)]
    version: String,
    #[structopt(short, long)]
    data_root: PathBuf,
}

fn main() {
    let Args { version, data_root } = Args::parse();
    let evaluation_task = EvaluationTask::Detection;
    let frame_id = FrameID::BaseLink;

    let frame_ground_truths =
        dataset::load_dataset(version, data_root, &evaluation_task, &frame_id);

    println!("Number of frames: {:?}", frame_ground_truths.unwrap().len());
}
