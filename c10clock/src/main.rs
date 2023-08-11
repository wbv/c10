// SPDX-License-Identifier: MIT OR Apache-2.0

//! A C10 "decimalized" clock for your terminal.

#![deny(warnings)]

use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crossterm::{cursor, terminal, ExecutableCommand, Result};

struct UI {
    stdout: std::io::Stdout,

    // timing variables for screen update
    clk_time: Instant,
    drifts_ns: [i64; 4],
    drifts_idx: usize,
}

impl UI {
    pub fn new() -> Self {
        let mut stdout = std::io::stdout();
        stdout.execute(cursor::Hide).unwrap();

        Self {
            stdout,
            clk_time: Instant::now(),
            drifts_ns: Default::default(),
            drifts_idx: Default::default(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            // clear the screen
            self.stdout
                .execute(terminal::Clear(terminal::ClearType::All))?;
            self.stdout.execute(cursor::MoveTo(0, 0))?;

            let c10now = c10::SystemTime::now();

            // write to the screen
            writeln!(self.stdout, "{c10now}")?;
            self.stdout.flush()?;

            // sleep until the next time should be printed
            self.sleep();
        }
    }

    fn sleep(&mut self) {
        // GOAL is one "tick" of duration (10^-6 of a day)
        const GOAL: Duration = Duration::from_micros(24 * 60 * 60);

        // "how long since the last loop iteration"
        let elapsed = self.clk_time.elapsed();
        self.clk_time += elapsed;

        // compute the average error from the "goal" sleep duration as measured by elapsed time
        let drift = (elapsed.as_nanos() as i64) - (GOAL.as_nanos() as i64);

        // store drifts in a rolling circular buffer
        self.drifts_ns[self.drifts_idx] = drift;
        self.drifts_idx = (self.drifts_idx + 1) % self.drifts_ns.len();

        let avg_drift: i64 = self.drifts_ns.iter().sum::<i64>() / self.drifts_ns.len() as i64;
        //eprintln!("avg_drift {:?}", avg_drift);

        // sleep for the adjusted amount of time accounting for average drift
        let computed_sleep = (GOAL.as_nanos() as i64) - avg_drift;
        //eprintln!("sleeping for: {:?}", computed_sleep);

        sleep(Duration::from_nanos(computed_sleep.try_into().unwrap()));
    }
}

// RAII cleanup of the terminal (undoes the init in UI::new())
impl Drop for UI {
    fn drop(&mut self) {
        std::io::stdout().execute(cursor::Show).unwrap();
    }
}

fn main() -> Result<()> {
    let mut ui = UI::new();
    ui.run()
}
