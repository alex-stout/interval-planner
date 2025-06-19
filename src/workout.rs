use std::fmt::Write;

#[derive(Debug)]
pub struct WorkoutPlan {
    pub name: String,
    pub description: Vec<String>,
    pub duration: u32,
    pub intervals: Vec<Interval>,
}

#[derive(Debug)]
pub struct Interval {
    pub name: Option<String>,
    pub repeat: Option<u32>,
    pub duration_sec: Option<u32>,
    pub percent_ftp_lo: Option<u32>,
    pub percent_ftp_hi: Option<u32>,
    pub cad_lo: Option<f32>,
    pub cad_hi: Option<f32>,
    pub sub_intervals: Vec<SubInterval>,
}

#[derive(Debug)]
pub struct SubInterval {
    pub name: Option<String>,
    pub duration_sec: u32,
    pub percent_ftp_lo: Option<u32>,
    pub percent_ftp_hi: Option<u32>,
    pub cad_lo: Option<f32>,
    pub cad_hi: Option<f32>,
}

impl WorkoutPlan {
    pub fn to_plan_file(&self) -> String {
        let mut output = String::new();

        writeln!(output, "=HEADER=").unwrap();
        writeln!(output, "# Provider: WAHOO").unwrap();
        writeln!(output).unwrap();

        writeln!(output, "NAME={}", self.name).unwrap();
        writeln!(output, "# Duration: {} mins", self.duration / 60).unwrap();
        writeln!(output, "DURATION={}", self.duration).unwrap();
        writeln!(output).unwrap();

        writeln!(output, "# PLAN_TYPE=STRUCTURED_WORKOUT").unwrap();
        writeln!(output, "PLAN_TYPE=0").unwrap();
        writeln!(output).unwrap();

        writeln!(output, "# WORKOUT_TYPE=BIKE").unwrap();
        writeln!(output, "WORKOUT_TYPE=0").unwrap();

        writeln!(output).unwrap();

        for desc in &self.description {
            writeln!(output, "DESCRIPTION={}", desc).unwrap();
        }

        writeln!(output, "\n\n=STREAM=").unwrap();

        for interval in &self.intervals {
            writeln!(output, "\n=INTERVAL=").unwrap();

            if let Some(name) = &interval.name {
                writeln!(output, "INTERVAL_NAME={}", name).unwrap();
            }
            if let Some(repeat) = interval.repeat {
                writeln!(output, "REPEAT={}", repeat).unwrap();
                writeln!(output, "MESG_DURATION_SEC>=0?EXIT").unwrap();
            }
            if let Some(duration) = interval.duration_sec {
                writeln!(output, "MESG_DURATION_SEC>={}?EXIT", duration).unwrap();
            }
            if let Some(lo) = interval.percent_ftp_lo {
                writeln!(output, "PERCENT_FTP_LO={}", lo).unwrap();
            }
            if let Some(hi) = interval.percent_ftp_hi {
                writeln!(output, "PERCENT_FTP_HI={}", hi).unwrap();
            }
            if let Some(cad_lo) = interval.cad_lo {
                writeln!(output, "CAD_LO={:.1}", cad_lo).unwrap();
            }
            if let Some(cad_hi) = interval.cad_hi {
                writeln!(output, "CAD_HI={:.1}", cad_hi).unwrap();
            }

            for sub in &interval.sub_intervals {
                writeln!(output, "\n=SUBINTERVAL=").unwrap();

                if let Some(name) = &sub.name {
                    writeln!(output, "INTERVAL_NAME={}", name).unwrap();
                }
                writeln!(output, "MESG_DURATION_SEC>={}?EXIT", sub.duration_sec).unwrap();
                if let Some(lo) = sub.percent_ftp_lo {
                    writeln!(output, "PERCENT_FTP_LO={}", lo).unwrap();
                }
                if let Some(hi) = sub.percent_ftp_hi {
                    writeln!(output, "PERCENT_FTP_HI={}", hi).unwrap();
                }
                if let Some(cad_lo) = sub.cad_lo {
                    writeln!(output, "CAD_LO={:.1}", cad_lo).unwrap();
                }
                if let Some(cad_hi) = sub.cad_hi {
                    writeln!(output, "CAD_HI={:.1}", cad_hi).unwrap();
                }
            }
        }

        output
    }
}
