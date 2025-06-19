mod workout;

use std::process::exit;

use clap::Parser;
use workout::{Interval, SubInterval, WorkoutPlan};

/// CLI to generate Wahoo-compatible .plan workout files
#[derive(Parser, Debug)]
#[command(name = "wahoo-plan-gen")]
#[command(version = "0.1.0")]
#[command(about = "Generate Wahoo .plan workout files", long_about = None)]
struct Cli {
    /// Output file name
    #[arg(short = 'o', long, default_value = "output.plan")]
    output: String,

    /// Name of the workout
    #[arg(short = 'n', long, default_value = "Interval workout")]
    name: String,

    /// Number of intervals
    #[arg(short = 'i', long, default_value_t = 3)]
    interval_count: u32,

    /// To start the workout with a warmup interval (10min)
    #[arg(short = 'w', long, default_value_t = true)]
    warmup: bool,

    /// To start the workout with a cooldown interval (10min)
    #[arg(short = 'c', long, default_value_t = true)]
    cooldown: bool,

    /// Total duration of workout in seconds (defaults to 1hr)
    #[arg(short = 'd', long, default_value_t = 3600)]
    duration: u32,
}

fn check_args(args: &Cli) {
    if args.interval_count <= 0 {
        println!("Interval count must be a positive integer");
        exit(1)
    }

    if args.duration <= 0 {
        println!("Duration count must be a positive integer");
        exit(1)
    }
}

fn generate_intervals(cli: &Cli) -> Vec<Interval> {
    let mut intervals: Vec<Interval> = Vec::new();

    if cli.warmup {
        intervals.push(Interval {
            name: Some("Warm up".to_string()),
            repeat: None,
            duration_sec: Some(600),
            percent_ftp_lo: Some(50),
            percent_ftp_hi: Some(70),
            cad_lo: None,
            cad_hi: None,
            sub_intervals: vec![],
        });
    }

    intervals.push(Interval {
        name: Some("Main Set".to_string()),
        repeat: Some(cli.interval_count),
        duration_sec: None,
        percent_ftp_lo: None,
        percent_ftp_hi: None,
        cad_lo: None,
        cad_hi: None,
        sub_intervals: vec![
            SubInterval {
                name: Some("Sweet Spot Interval".to_string()),
                duration_sec: 300,
                percent_ftp_lo: Some(85),
                percent_ftp_hi: Some(95),
                cad_lo: None,
                cad_hi: None,
            },
            SubInterval {
                name: Some("Recovery".to_string()),
                duration_sec: 180,
                percent_ftp_lo: Some(40),
                percent_ftp_hi: Some(55),
                cad_lo: None,
                cad_hi: None,
            },
        ],
    });

    if cli.cooldown {
        intervals.push(Interval {
            name: Some("Cool Down".to_string()),
            repeat: None,
            duration_sec: Some(600),
            percent_ftp_lo: Some(40),
            percent_ftp_hi: Some(55),
            cad_lo: None,
            cad_hi: None,
            sub_intervals: vec![],
        });
    }

    intervals
}

fn main() {
    let cli = Cli::parse();

    check_args(&cli);

    let intervals: Vec<Interval> = generate_intervals(&cli);

    let workout = WorkoutPlan {
        name: cli.name,
        description: vec![
            "Sweet spot intervals for FTP improvement.".to_string(),
            "Warm up, intervals, and cool down.".to_string(),
        ],
        duration: cli.duration,
        intervals: intervals
    };

    let contents = workout.to_plan_file();
    std::fs::write(&cli.output, contents).unwrap();
    println!("Workout written to: {}", &cli.output);
}
