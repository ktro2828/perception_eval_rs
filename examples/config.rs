use clap::Parser;
use perception_eval::config::PerceptionEvaluationConfig;
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
    println!("Config: {:?}", config);
    Ok(())
}
