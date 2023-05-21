use clap::Parser;
use perception_eval::{
    config::{get_evaluation_params, PerceptionEvaluationConfig},
    evaluation_task::EvaluationTask,
    manager::PerceptionEvaluationManager,
};
use std::error::Error;

#[derive(Parser)]
struct Args {
    #[structopt(short, long)]
    version: String,
    #[structopt(short, long)]
    data_root: String,
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let Args { version, data_root } = Args::parse();

    let result_dir = format!(
        "./work_dir/{}",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );

    let (filter_params, metrics_params) = get_evaluation_params(
        &vec!["Car", "Bus", "Pedestrian"],
        100.0,
        100.0,
        Some(0),
        None,
        1.0,
        2.0,
        0.5,
        0.5,
    )?;

    let config = PerceptionEvaluationConfig::new(
        &version,
        &data_root,
        EvaluationTask::Detection,
        &"BaseLink".to_string(),
        &result_dir,
        filter_params,
        metrics_params,
        false,
    );

    let mut manager = PerceptionEvaluationManager::new(&config)?;

    let mut frames = manager.frame_ground_truths.clone();
    for (_, frame) in frames.iter_mut().enumerate() {
        let frame_ground_truth = manager.get_frame_ground_truth(&frame.timestamp);
        match frame_ground_truth {
            Some(mut frame_gt) => manager.add_frame_result(&mut frame.objects, &mut frame_gt)?,
            None => continue,
        }
    }

    let score = manager.get_scene_score()?;
    println!("{}", score);

    Ok(())
}