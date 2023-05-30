use std::path::{Path, PathBuf};

use clap::Parser;
use perception_eval::{
    dataset::{self, get_current_frame},
    evaluation_task::EvaluationTask,
    frame_id::FrameID,
    utils::logger::configure_logger,
};
use std::error::Error;

#[derive(Parser)]
struct Args {
    #[clap(short = 'v', long = "version", default_value = "annotation")]
    version: String,
    #[clap(short = 'd', long = "data-root", default_value = "./tests/sample_data")]
    data_root: PathBuf,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let log_dir_name = format!(
        "./work_dir/{}",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );
    let log_dir = Path::new(&log_dir_name);
    configure_logger(log_dir, log::Level::Debug)?;
    let Args { version, data_root } = Args::parse();

    let frame_ground_truths = dataset::load_dataset(
        &version,
        &data_root,
        &EvaluationTask::Detection,
        &FrameID::BaseLink,
    )?;

    let num_frames = frame_ground_truths.len();
    println!("Number of frames: {:?}", num_frames);

    for i in 0..num_frames {
        let gt = get_current_frame(
            &frame_ground_truths.as_ref(),
            &frame_ground_truths[i].timestamp,
        )
        .unwrap();

        println!(
            "[Frame {}]\n(Current GT):  {}\n(Corresponding GT): {}",
            &i, &frame_ground_truths[i], &gt
        );

        assert_eq!(
            &gt, &frame_ground_truths[i],
            "Current GT is not same, but got : {} and {}",
            &gt, &frame_ground_truths[i]
        );
    }

    Ok(())
}
