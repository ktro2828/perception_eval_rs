use clap::Parser;
use perception_eval::{config::PerceptionEvaluationConfig, manager::PerceptionEvaluationManager};
use std::error::Error;

#[derive(Parser)]
struct Args {
    #[clap(
        short = 's',
        long = "scenario",
        default_value = "tests/config/perception.yaml"
    )]
    scenario: String,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let Args { scenario } = Args::parse();

    let result_dir = &format!(
        "./work_dir/{}",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );

    let config = PerceptionEvaluationConfig::from(&scenario, result_dir, false)?;

    let mut manager = PerceptionEvaluationManager::from(&config)?;

    let mut frames = manager.frame_ground_truths.clone();
    for (_, frame) in frames.iter_mut().enumerate() {
        let frame_ground_truth = manager.get_frame_ground_truth(&frame.timestamp);
        match frame_ground_truth {
            Some(frame_gt) => manager.add_frame_result(&frame.objects, &frame_gt)?,
            None => continue,
        }
    }

    let score = manager.get_metrics_score()?;
    println!("{}", score);

    Ok(())
}
