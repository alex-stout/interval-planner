# Interval Planner

CLI tool to generate Wahoo-compatible `.plan` workout files.

## Features

- Generate structured interval workouts in Wahoo's `.plan` format
- Customize:
  - Workout name
  - Number of intervals
  - Total duration
  - Warmup and cooldown inclusion
  - Output file name
- Default workout structure:
  - Warmup (10 minutes at 50–70% FTP)
  - Repeated Sweet Spot intervals (5 minutes @ 85–95% FTP, 3 minutes recovery @ 40–55% FTP)
  - Cooldown (10 minutes at 40–55% FTP)

## Installation

1. Clone the repository
2. Build with Cargo: `cargo build --release`
3. Binary will be located at: `target/release/interval-planner`

## Usage

`interval-planner [OPTIONS]`

### Options

- `-o, --output <FILE>` — Output file name (default: `output.plan`)
- `-n, --name <NAME>` — Name of the workout (default: `Interval workout`)
- `-i, --interval-count <N>` — Number of repeated Sweet Spot intervals (default: 3)
- `-w, --warmup / --no-warmup` — Include a 10-minute warmup (default: enabled)
- `-c, --cooldown / --no-cooldown` — Include a 10-minute cooldown (default: enabled)
- `-d, --duration <SECONDS>` — Total workout duration in seconds (default: 3600)

### Example

`interval-planner -o my_workout.plan -n "FTP Builder" -i 4 --no-warmup --cooldown -d 4500`

Generates a `.plan` file named `my_workout.plan` with:

- Name: FTP Builder
- 4 repeated intervals
- No warmup, includes cooldown
- Total duration: 4500 seconds

## Output Format

Generates a `.plan` file compatible with Wahoo cycling computers for structured interval training.

## License

MIT License
