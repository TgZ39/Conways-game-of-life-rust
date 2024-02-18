use crate::cgof::Cell::{Alive, Dead};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

pub struct Board {
    pub cells: Vec<Cell>,
    length: u32,
}

impl Board {
    pub fn new(length: u32) -> Self {
        let mut cells = vec![];
        let mut rng = rand::thread_rng();

        for _ in 1..=(length * length) {
            cells.push(rng.gen())
        }

        Self { cells, length }
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<Cell> {
        self.cells.get((self.length * y + x) as usize).cloned()
    }

    pub fn set_cell(&mut self, x: u32, y: u32, alive: Cell) {
        if self.cells.get((self.length * y + x) as usize).is_some() {
            self.cells[(self.length * y + x) as usize] = alive
        }
    }

    // there is probably a better way to do this but idc
    pub fn count_neighbours(&self, x: u32, y: u32) -> u32 {
        let x = x as i32;
        let y = y as i32;

        let mut count = 0;

        for x_offset in [-1, 0, 1] {
            if x + x_offset < 0 {
                continue;
            }
            for y_offset in [-1, 0, 1] {
                if y + y_offset < 0
                    || (x_offset, y_offset) == (0, 0)
                    || self
                        .get_cell((x + x_offset) as u32, (y + y_offset) as u32)
                        .is_none()
                    || self
                        .get_cell((x + x_offset) as u32, (y + y_offset) as u32)
                        .unwrap()
                        == Dead
                {
                    continue;
                }
                count += 1;
            }
        }

        count
    }

    pub fn print(&self) {
        for x in 0..self.length {
            let mut out = String::new();

            for y in 0..self.length {
                out += match self.get_cell(x, y) {
                    None => "  ",
                    Some(cell) if { cell == Alive } => "x ",
                    _ => "  ",
                }
            }

            println!("{}", out);
        }
    }

    pub fn step(&mut self) {
        let mut new_board = Board::new(self.length);

        for x in 0..self.length {
            for y in 0..self.length {
                new_board.set_cell(
                    x,
                    y,
                    match (
                        self.get_cell(x, y).unwrap_or(Dead),
                        self.count_neighbours(x, y),
                    ) {
                        (Alive, 0..=1) => Dead,
                        (Alive, 2..=3) => Alive,
                        (Alive, 4..=8) => Dead,
                        (Dead, 3) => Alive,
                        (_, _) => Dead,
                    },
                );
            }
        }

        self.cells = new_board.cells;
    }
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0..=1) {
            0 => Cell::Alive,
            _ => Dead,
        }
    }
}
