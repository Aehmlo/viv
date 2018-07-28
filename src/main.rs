extern crate termion;
#[macro_use]
extern crate viv;

use termion::{clear, color, cursor, style, terminal_size};
use viv::{Grid, Index};

use std::{
    fmt, io::{self, prelude::*}, thread::sleep, time::Duration,
};

// TODO: Use background coloring and spaces instead, probably.
const DEAD: &'static str = "◼️";
const LIVE: &'static str = "◻️";
const REFRESH_RATE: u64 = 1; // Hz

struct Viewport {
    width: u16,
    height: u16,
    grid: Option<Grid>,
}

impl Viewport {
    /// Constructs a new viewport showing the given number of rows and columns.
    fn new(columns: u16, rows: u16) -> Self {
        Self {
            width: columns,
            height: rows,
            grid: None,
        }
    }
    /// Returns the row-by-row output for this viewport.
    fn rows(&self) -> impl IntoIterator<Item = String> {
        let grid = self.grid.clone().unwrap();
        let axes = ((self.width / 2) as isize, (self.height / 2) as isize);
        let minima = (-axes.0, -axes.1);
        let maxima = match (self.width, self.height) {
            (x, y) if x % 2 == 0 && y % 2 == 0 => (axes.0 - 1, axes.1 - 1),
            (x, _) if x % 2 == 0 => (axes.0 - 1, axes.1),
            (_, y) if y % 2 == 0 => (axes.0, axes.1 - 1),
            (_, _) => axes,
        };
        let mut rows = Vec::with_capacity(self.height as usize);
        for y in minima.1..maxima.1 {
            let mut s = String::with_capacity((self.width as usize) * 2);
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
        if self.grid.is_none() {
            write!(
                f,
                "{}",
                DEAD.repeat(self.width as usize)
                    .repeat(self.height as usize)
            )
        } else {
            for row in self.rows() {
                writeln!(f, "{}", row)?;
            }
            Ok(())
        }
    }
}

fn main() {
    let (width, height) = terminal_size().unwrap_or((100, 40));
    let mut viewport = Viewport::new(width, height);
    let grid = Grid::generate();
    viewport.grid = Some(grid);
    print!("{}", cursor::Hide);
    loop {
        print!("{}", viewport);
        io::stdout().flush().expect("Failed to flush stdout.");
        sleep(Duration::from_millis(1_000 / REFRESH_RATE));
        viewport.grid = viewport.grid.map(|g| g.tick());
    }
}
