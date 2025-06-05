//! fitgen — Generate structured FIT workout files in Rust.
//!
//! This crate provides types and functions to define and write
//! structured workout files in the Garmin `.fit` format.

use std::io::{Result, Write};
use byteorder::{LittleEndian, WriteBytesExt};

/// Represents a single workout step.
#[derive(Debug, Clone)]
pub struct WorkoutStep {
    pub duration_secs: u32,
    pub target_power_low: u16,
    pub target_power_high: u16,
    pub name: String,
}

/// A structured workout consisting of multiple steps.
#[derive(Debug, Clone)]
pub struct Workout {
    pub title: String,
    pub steps: Vec<WorkoutStep>,
}

impl Workout {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            steps: Vec::new(),
        }
    }

    pub fn add_step(
        &mut self,
        name: &str,
        duration_secs: u32,
        target_power_low: u16,
        target_power_high: u16,
    ) {
        self.steps.push(WorkoutStep {
            name: name.to_string(),
            duration_secs,
            target_power_low,
            target_power_high,
        });
    }
}

/// Writes the workout to a binary .fit file (simplified format).
pub struct FitWriter<W: Write> {
    writer: W,
    buffer: Vec<u8>, // Temporary buffer for messages
}

impl<W: Write> FitWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            buffer: Vec::new(),
        }
    }

    pub fn write_workout(&mut self, workout: &Workout) -> Result<()> {
        self.write_file_id_message()?;
        self.write_workout_message(workout)?;
        for step in &workout.steps {
            self.write_workout_step_message(step)?;
        }
        self.finalize()
    }

    fn write_file_id_message(&mut self) -> Result<()> {
        // Placeholder example: not yet a true FIT message
        self.buffer.write_u8(0x00)?; // fake record header
        self.buffer.write_all(b"FILEID")?;
        Ok(())
    }

    fn write_workout_message(&mut self, workout: &Workout) -> Result<()> {
        self.buffer.write_u8(0x01)?; // fake record header
        self.buffer.write_all(workout.title.as_bytes())?;
        Ok(())
    }

    fn write_workout_step_message(&mut self, step: &WorkoutStep) -> Result<()> {
        self.buffer.write_u8(0x02)?; // fake record header
        self.buffer.write_u32::<LittleEndian>(step.duration_secs)?;
        self.buffer.write_u16::<LittleEndian>(step.target_power_low)?;
        self.buffer.write_u16::<LittleEndian>(step.target_power_high)?;
        let name_bytes = step.name.as_bytes();
        self.buffer.write_u8(name_bytes.len() as u8)?;
        self.buffer.write_all(name_bytes)?;
        Ok(())
    }

    fn finalize(&mut self) -> Result<()> {
        let data_size = self.buffer.len() as u32;

        let mut header = Vec::with_capacity(14);
        header.write_u8(14)?; // header size
        header.write_u8(0x10)?; // protocol version
        header.write_u16::<LittleEndian>(100)?; // profile version
        header.write_u32::<LittleEndian>(data_size)?; // data size
        header.extend_from_slice(b".FIT");

        let crc_header = crc16(&header[..12]);
        header.write_u16::<LittleEndian>(crc_header)?;

        self.writer.write_all(&header)?;
        self.writer.write_all(&self.buffer)?;

        let crc_data = crc16(&self.buffer);
        self.writer.write_u16::<LittleEndian>(crc_data)?;

        Ok(())
    }
}

pub fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}


// Generated using standard polynomial 0x1021
const CRC_TABLE: [u16; 256] = {
    const fn generate() -> [u16; 256] {
        let mut table = [0u16; 256];
        let mut i = 0;
        while i < 256 {
            let mut crc = (i as u16) << 8;
            let mut j = 0;
            while j < 8 {
                if (crc & 0x8000) != 0 {
                    crc = (crc << 1) ^ 0x1021;
                } else {
                    crc <<= 1;
                }
                j += 1;
            }
            table[i] = crc;
            i += 1;
        }
        table
    }
    generate()
};
