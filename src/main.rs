use std::fs::File;
use std::io::BufWriter;

use fitgen::{FitWriter, Workout};

fn main() -> std::io::Result<()> {
    // Create a workout
    let mut workout = Workout::new("Sweet Spot 3x12");

    workout.add_step("Warmup", 600, 140, 160);
    for i in 1..=3 {
        workout.add_step(&format!("Sweet Spot {}", i), 720, 223, 235);
        workout.add_step("Recovery", 300, 120, 140);
    }
    workout.add_step("Cool Down", 300, 130, 150);

    // Open output file
    let file = File::create("sweetspot.fit")?;
    let buf_writer = BufWriter::new(file);

    // Write workout to .fit file
    let mut fit_writer = FitWriter::new(buf_writer);
    fit_writer.write_workout(&workout)?;

    println!("Workout saved to sweetspot.fit");

    Ok(())
}
