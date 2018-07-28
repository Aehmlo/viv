#[macro_use]
extern crate viv;
#[cfg(feature = "tui")]
extern crate termion;

use std::{
    io::{self, prelude::*}, thread::sleep, time::Duration,
};

#[cfg(feature = "tui")]
mod term {
    use std::fmt;

    use termion::{cursor, terminal_size};
    use viv::{Grid, Index};

    const DEAD: &'static str = "◼️";
    const LIVE: &'static str = "◻️";
    pub const REFRESH_RATE: u64 = 1; // Hz

    pub struct Viewport {
        pub grid: Grid,
    }

    impl Viewport {
        /// Creates a new viewport.
        pub fn new() -> Self {
            Self {
                grid: Grid::generate(),
            }
        }

        /// Returns the row-by-row output for this viewport.
        pub fn rows(&self) -> impl IntoIterator<Item = String> {
            let grid = self.grid.clone();
            let (width, height) = terminal_size().unwrap_or((100, 40));
            let axes = ((width / 2) as isize, (height / 2) as isize);
            let minima = (-axes.0, -axes.1);
            let maxima = match (width, height) {
                (x, y) if x % 2 == 0 && y % 2 == 0 => (axes.0 - 1, axes.1 - 1),
                (x, _) if x % 2 == 0 => (axes.0 - 1, axes.1),
                (_, y) if y % 2 == 0 => (axes.0, axes.1 - 1),
                (_, _) => axes,
            };
            let mut rows = Vec::with_capacity(height as usize);
            for y in minima.1..maxima.1 {
                let mut s = String::with_capacity((width as usize) * 2);
                for x in minima.0..maxima.0 {
                    let idx = index!(x, y);
                    s.push_str(if grid.is_living(&idx) { LIVE } else { DEAD });
                }
                rows.push(s);
            }
            rows
        }
    }

    impl fmt::Display for Viewport {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            print!("{}", cursor::Hide);
            for row in self.rows() {
                writeln!(f, "{}", row)?;
            }
            Ok(())
        }
    }
}

fn main() {
    #[cfg(feature = "tui")]
    {
        let mut viewport = term::Viewport::new();
        loop {
            print!("{}", viewport);
            io::stdout().flush().expect("Failed to flush stdout.");
            sleep(Duration::from_millis(1_000 / term::REFRESH_RATE));
            viewport.grid = viewport.grid.tick();
        }
    }
}
