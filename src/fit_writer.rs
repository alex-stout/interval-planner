use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Write, Result};
use crate::workout::{Workout, WorkoutStep};
use crate::crc;

pub struct FitWriter<W: Write> {
    writer: W,
    buffer: Vec<u8>, // hold message payloads before final write
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
        // Write file_id message (type = 0)
        // TODO: Define message structure
        Ok(())
    }

    fn write_workout_message(&mut self, workout: &Workout) -> Result<()> {
        // Write workout definition and data
        // TODO: Follow FIT SDK definition
        Ok(())
    }

    fn write_workout_step_message(&mut self, step: &WorkoutStep) -> Result<()> {
        // Write workout_step definition and data
        Ok(())
    }

    fn finalize(&mut self) -> Result<()> {
        let data_size = self.buffer.len() as u32;

        // 1. Write header with CRC
        let mut header = Vec::with_capacity(14);
        header.write_u8(14)?; // header size
        header.write_u8(0x10)?; // protocol version
        header.write_u16::<LittleEndian>(100)?; // profile version
        header.write_u32::<LittleEndian>(data_size)?; // size of data section
        header.extend_from_slice(b".FIT");
        let crc = crc::crc16(&header[..12]);
        header.write_u16::<LittleEndian>(crc)?;
        self.writer.write_all(&header)?;

        // 2. Write messages
        self.writer.write_all(&self.buffer)?;

        // 3. Write CRC of data payload
        let crc = crc::crc16(&self.buffer);
        self.writer.write_u16::<LittleEndian>(crc)?;

        Ok(())
    }
}
