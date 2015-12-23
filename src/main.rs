extern crate piston_window;

use piston_window::*;

struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<bool>>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let mut board = Board {
            width: width,
            height: height,
            tiles: vec![vec![false; width]; height],
        };
        for y in 0..board.height {
            let mut row = Vec::new();
            for x in 0..board.width {
                row.push(false);
            }
            board.tiles.push(row);
        }
        board
    }

    fn print(&self) {
        for y in 0..self.height {
            let mut row = "".to_owned();
            for x in 0..self.width {
                if self.get(x, y) {
                    row = row + "#";
                } else {
                    row = row + ".";
                }
            }
            println!("{}", row);
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.tiles[y][x]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.tiles[y][x] = value;
    }
}

fn main() {
    let mut board = Board::new(10, 20);
    board.set(1, 1, true);
    let window: PistonWindow =
        WindowSettings::new("Tetris", [board.width as u32 * 16, board.height as u32 * 16])
        .exit_on_esc(true).build().unwrap();
    for e in window {
        e.draw_2d(|c, g| {
            clear([0.0; 4], g);
            for y in 0..board.height {
                for x in 0..board.width {
                    if board.get(x, y) {
                        rectangle([1.0, 0.0, 0.0, 1.0], // red
                                  [x as f64 * 16.0, y as f64 * 16.0, 16.0, 16.0],
                                  c.transform, g);
                    }
                }
            }
        });
    }
}
