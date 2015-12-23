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
        for y in (0..board.height) {
            let mut row = Vec::new();
            for x in (0..board.width) {
                row.push(false);
            }
            board.tiles.push(row);
        }
        board
    }

    fn print(&self) {
        for y in (0..self.height) {
            let mut row = "".to_owned();
            for x in (0..self.width) {
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
    let board = Board::new(10, 10);
    board.print();
}
