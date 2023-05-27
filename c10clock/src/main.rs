// SPDX-License-Identifier: MIT OR Apache-2.0

//! A C10 "decimalized" clock for your terminal.

#![deny(warnings)]

use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use crossterm::{ExecutableCommand, terminal, cursor, Result};

const UPDATE_RATE_HZ: u64 = 60;
const UPDATE_PERIOD: Duration = Duration::from_micros(1_000_000/UPDATE_RATE_HZ);

struct UI {
    stdout: std::io::Stdout,
}

impl UI {
    pub fn new() -> Self {
        let mut stdout = std::io::stdout();
        stdout.execute(cursor::Hide).unwrap();

        Self {
            stdout
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            // clear the screen
            self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;
            self.stdout.execute(cursor::MoveTo(0,0))?;

            let now = c10::SystemTime::now();

            // write to the screen
            write!(self.stdout, "{now}\n")?;
            self.stdout.flush()?;

            // wait until next update
            sleep(UPDATE_PERIOD);
        }
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
