use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

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

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Dijkstra {
    frontier: BinaryHeap<State>,
    came_from: HashMap<usize, usize>,
    cost_so_far: HashMap<usize, usize>,
}

impl Dijkstra {
    pub fn new() -> Self {
        Self {
            frontier: BinaryHeap::new(),
            came_from: HashMap::new(),
            cost_so_far: HashMap::new(),
        }
    }

    pub fn run(
        &mut self,
        map_data: &MapData,
        start: usize,
        end: usize,
    ) -> HashMap<usize, usize> {
        let start_state = State {
            cost: 0,
            position: start,
        };

        self.frontier.push(start_state);
        self.came_from.insert(start, 0);
        self.cost_so_far.insert(start, 0);

        while !self.frontier.is_empty() {
            let current = self.frontier.pop().unwrap();

            if current.position == end {
                break;
            }

            for (x, y) in NEIGHBORS {
                let c_x = current.position % map_data.map_width;
                let c_y = current.position / map_data.map_width;

                let n_x = c_x as isize + x;
                let n_y = c_y as isize + y;

                let n_index = n_x + (n_y * map_data.map_width as isize);

                if n_index < 0 || n_index >= map_data.map.len() as isize {
                    continue;
                }

                let index = n_index as usize;

                let tile = &map_data.map[index];

                let tile_cost = if *tile == Tile::Wall { 10 } else { 1 };

                let new_cost = self.cost_so_far[&current.position] + tile_cost;

                if !self.cost_so_far.contains_key(&index)
                    || new_cost < self.cost_so_far[&index]
                {
                    self.cost_so_far.insert(index, new_cost);
                    let new_state = State {
                        cost: new_cost,
                        position: index,
                    };
                    self.frontier.push(new_state);
                    self.came_from.insert(index, current.position);
                }
            }
        }

        self.came_from.clone()
    }
}
