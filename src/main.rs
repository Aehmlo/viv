#[macro_use]
extern crate viv;

#[cfg(feature = "tui")]
extern crate termion;

#[cfg(feature = "web")]
extern crate stdweb;

#[cfg(feature = "tui")]
mod term {
    use std::{
        fmt, io::{self, prelude::*}, thread::sleep, time::Duration,
    };

    use termion::{cursor, terminal_size};
    use viv::{Grid, Index};

    const DEAD: &str = "◼️";
    const LIVE: &str = "◻️";
    const REFRESH_RATE: u64 = 1; // Hz

    struct Viewport {
        pub grid: Grid,
    }

    impl Viewport {
        /// Creates a new viewport.
        fn new() -> Self {
            Self {
                grid: Grid::generate(),
            }
        }

        /// Returns the row-by-row output for this viewport.
        fn rows(&self) -> impl IntoIterator<Item = String> {
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

    pub fn main() {
        let mut viewport = Viewport::new();
        loop {
            print!("{}", viewport);
            io::stdout().flush().expect("Failed to flush stdout.");
            sleep(Duration::from_millis(1_000 / REFRESH_RATE));
            viewport.grid = viewport.grid.tick();
        }
    }
}

#[cfg(feature = "web")]
mod web {
    use stdweb::{
        traits::*, unstable::TryInto,
        web::{document, html_element::*, window, CanvasRenderingContext2d},
    };

    use viv::{Grid, Index};

    const WIDTH: f64 = 20.0;
    const HEIGHT: f64 = WIDTH;

    pub fn main() {
        tick(Grid::generate())();
    }

    fn tick(grid: Grid) -> impl FnOnce() {
        move || {
            let canvas: CanvasElement = document()
                .get_element_by_id("canvas")
                .unwrap()
                .try_into()
                .unwrap();
            canvas.set_width(window().inner_width() as u32);
            canvas.set_height(window().inner_height() as u32);
            let ctx = canvas.get_context::<CanvasRenderingContext2d>().unwrap();
            ctx.set_fill_style_color("black");
            let width = canvas.width().into();
            let height = canvas.height().into();
            ctx.fill_rect(0.0, 0.0, width, height);

            let width = (width / WIDTH) as isize;
            let height = (height / HEIGHT) as isize;
            let axes = (width / 2, height / 2);
            let minima = (-axes.0, -axes.1);
            let maxima = match (width, height) {
                (x, y) if x % 2 == 0 && y % 2 == 0 => (axes.0 - 1, axes.1 - 1),
                (x, _) if x % 2 == 0 => (axes.0 - 1, axes.1),
                (_, y) if y % 2 == 0 => (axes.0, axes.1 - 1),
                (_, _) => axes,
            };
            let grid = grid.tick();
            ctx.set_fill_style_color("white");
            for x in minima.0..maxima.0 {
                for y in minima.1..maxima.1 {
                    if grid.is_living(&index!(x, y)) {
                        ctx.fill_rect(WIDTH * ((x - minima.0) as f64), HEIGHT * ((y - minima.1) as f64), WIDTH, HEIGHT);
                    }
                }
            }

            window().set_timeout(tick(grid), 200);
        }
    }
}

fn main() {
    #[cfg(feature = "tui")]
    term::main();
    #[cfg(feature = "web")]
    web::main();
}

#[cfg(all(feature = "tui", feature = "web"))]
compile_error!(
    "The terminal UI and web UI configurations are mutually exlusive. \
     Please use either `tui` or `web`, not both, in a single compilation."
);
