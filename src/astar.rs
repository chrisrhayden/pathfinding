// a*

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

// from https://stackoverflow.com/a/39950148
#[derive(Copy, Clone, PartialEq)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// this was taken form the BinaryHeap docs
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: MinNonNan,
    pos: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // other.cost.cmp(&self.cost)
        // .then_with(|| self.pos.cmp(&other.pos))
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct AStar {
    frontier: BinaryHeap<State>,
    came_from: HashMap<usize, usize>,
    cost_so_far: HashMap<usize, f64>,
}

impl AStar {
    pub fn run(
        &mut self,
        map_data: &MapData,
        start: usize,
        end: usize,
    ) -> HashMap<usize, usize> {
        let map_width = map_data.map_width;

        let start_state = State {
            cost: MinNonNan(0.0),
            pos: start,
        };

        self.frontier.push(start_state);

        // its more likely that 0 is a valid rathern the usize::MAX
        self.came_from.insert(start, usize::MAX);

        self.cost_so_far.insert(start, 0.0);

        while !self.frontier.is_empty() {
            // we know there is data from the !.is_empty()
            let current = self.frontier.pop().unwrap();

            if current.pos == end {
                break;
            }

            for (o_x, o_y) in NEIGHBORS {
                let new_pos =
                    offset_position(current.pos, *o_x, *o_y, map_width);

                if new_pos < 0 || new_pos >= map_data.map.len() as isize {
                    continue;
                }

                let new_pos = new_pos as usize;

                let tile_cost = if map_data.map[new_pos] == Tile::Wall {
                    10.0
                } else {
                    1.0
                };

                let new_cost =
                    self.cost_so_far.get(&current.pos).unwrap() + tile_cost;

                if !self.cost_so_far.contains_key(&new_pos)
                    || new_cost < *self.cost_so_far.get(&new_pos).unwrap()
                {
                    self.cost_so_far.insert(new_pos, new_cost);

                    let priority =
                        new_cost + heuristic(end, new_pos, map_width);

                    let new_state = State {
                        cost: MinNonNan(priority),
                        pos: new_pos,
                    };

                    self.frontier.push(new_state);

                    self.came_from.insert(new_pos, current.pos);
                }
            }
        }

        self.came_from.clone()
    }
}

fn heuristic(a: usize, b: usize, width: usize) -> f64 {
    let a_x = (a % width) as f64;
    let a_y = (a / width) as f64;

    let b_x = (b % width) as f64;
    let b_y = (b / width) as f64;

    (a_x - b_x).abs() + (a_y - b_y).abs()
}

fn offset_position(
    current: usize,
    offset_x: isize,
    offset_y: isize,
    width: usize,
) -> isize {
    let c_x = (current % width) as isize;
    let c_y = (current / width) as isize;

    let n_x = c_x + offset_x;
    let n_y = c_y + offset_y;

    n_x + (n_y * width as isize)
}
