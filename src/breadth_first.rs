// breadth first

use crate::map::{MapData, Tile};

const NEIGHBORS: &[(isize, isize); 8] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, 1),
];

pub struct BreadthFirst {
    frontier: Vec<(isize, isize)>,
    reached: Vec<(isize, isize)>,
}

impl BreadthFirst {
    pub fn new() -> Self {
        Self {
            frontier: vec![],
            reached: vec![],
        }
    }

    pub fn run(&mut self, map: &MapData) -> Vec<(isize, isize)> {
        for (i, tile) in map.map.iter().enumerate() {
            if *tile == Tile::Floor {
                let x = (i % map.map_width) as isize;
                let y = (i / map.map_width) as isize;

                self.frontier.push((x, y));
                self.reached.push((x, y));

                break;
            }
        }

        let map_size = map.map.len() as isize;

        while !self.frontier.is_empty() {
            let (c_x, c_y) = self.frontier.pop().unwrap();

            for (offset_x, offset_y) in NEIGHBORS {
                let n_x = c_x + offset_x;
                let n_y = c_y + offset_y;

                let n_index = n_x + (n_y * map.map_width as isize);

                if n_index < 0 || n_index >= map_size {
                    continue;
                }

                let tile = &map.map[n_index as usize];

                if *tile == Tile::Wall {
                    continue;
                }

                if !self.reached.contains(&(n_x, n_y)) {
                    self.frontier.push((n_x, n_y));
                    self.reached.push((n_x, n_y));
                }
            }
        }

        self.reached.clone()
    }
}
