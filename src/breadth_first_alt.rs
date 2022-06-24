// breadth first

use std::collections::HashMap;

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

pub type Coordinates = HashMap<(isize, isize), (isize, isize)>;

pub struct BreadthFirst {
    frontier: Vec<(isize, isize)>,
    came_from: Coordinates,
}

impl BreadthFirst {
    pub fn new() -> Self {
        Self {
            frontier: vec![],
            came_from: HashMap::new(),
        }
    }

    pub fn run(
        &mut self,
        map: &MapData,
        start: (isize, isize),
    ) -> HashMap<(isize, isize), (isize, isize)> {
        let map_size = map.map.len() as isize;

        self.frontier.push(start);

        self.came_from.insert(start, (-1, -1));

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

                if self.came_from.get(&(n_x, n_y)).is_none() {
                    self.frontier.push((n_x, n_y));
                    self.came_from.insert((n_x, n_y), (c_x, c_y));
                }
            }
        }

        self.came_from.clone()
    }
}
