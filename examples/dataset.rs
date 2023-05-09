use std::path::{Path, PathBuf};

use clap::Parser;
use perception_eval::{
    dataset::{self, get_current_frame},
    evaluation_task::EvaluationTask,
    frame_id::FrameID,
    logger::configure_logger,
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
        dataset::load_dataset(version, data_root, &evaluation_task, &frame_id).unwrap();

    let num_frames = frame_ground_truths.len();
    println!("Number of frames: {:?}", num_frames);

    for i in 0..num_frames {
        let gt = get_current_frame(
            &frame_ground_truths.as_ref(),
            &frame_ground_truths[i].timestamp,
        )
        .unwrap();

        println!("Frame [{}]: {}", i, frame_ground_truths[i]);
        println!("Corresponding GT: {}", &gt);

        assert_eq!(
            &gt, &frame_ground_truths[i],
            "Current GT is not same, but got : {} and {}",
            &gt, &frame_ground_truths[i]
        );
    }
}
