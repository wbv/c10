use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use crossterm::{ExecutableCommand, terminal, cursor, Result};

const UPDATE_RATE_HZ: u64 = 30;
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
        for i in 1..=100 {
            // clear the screen
            self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;
            self.stdout.execute(cursor::MoveTo(0,0))?;

            // write to the screen
            write!(self.stdout, "{i:>8}\n")?;

            // wait until next update
            sleep(UPDATE_PERIOD);
        }

        return Ok(());
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
