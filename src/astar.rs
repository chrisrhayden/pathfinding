// a*

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::map::{MapData, MapPos, Tile};

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

// this was taken form the BinaryHeap docs
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    pos: MapPos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
        // self.cost.cmp(&other.cost)
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
    came_from: HashMap<MapPos, MapPos>,
    cost_so_far: HashMap<MapPos, isize>,
}

impl AStar {
    pub fn run(
        &mut self,
        map_data: &MapData,
        start: MapPos,
        end: MapPos,
    ) -> Vec<MapPos> {
        let map_width = map_data.map_width;

        let start_state = State {
            cost: 0,
            pos: start,
        };

        self.frontier.push(start_state);

        // its more likely that 0 is a valid rather then usize::MAX
        self.came_from.insert(start, usize::MAX);

        self.cost_so_far.insert(start, 0);

        while let Some(current) = self.frontier.pop() {
            if current.pos == end {
                break;
            }

            for (o_x, o_y) in NEIGHBORS {
                let new_pos =
                    offset_position(*o_x, *o_y, current.pos, map_width);

                if new_pos < 0 || new_pos >= map_data.map.len() as isize {
                    continue;
                }

                let new_pos = new_pos as MapPos;

                let tile_cost = if map_data.map[new_pos] == Tile::Wall {
                    100
                } else {
                    1
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
                        cost: priority,
                        pos: new_pos,
                    };

                    self.frontier.push(new_state);

                    self.came_from.insert(new_pos, current.pos);
                }
            }
        }

        // make_path(&self.came_from, start, end, map_width)
        make_path(&self.came_from, start, end)
    }
}

// Manhattan
fn heuristic(a: MapPos, b: MapPos, width: usize) -> isize {
    let a_x = (a % width) as isize;
    let a_y = (a / width) as isize;

    let b_x = (b % width) as isize;
    let b_y = (b / width) as isize;

    (a_x - b_x).abs() + (a_y - b_y).abs()
}

// fn heuristic(a: MapPos, b: MapPos, width: usize) -> isize {
//     let a_x = (a % width) as isize;
//     let a_y = (a / width) as isize;
//
//     let b_x = (b % width) as isize;
//     let b_y = (b / width) as isize;
//
//     let dx = (a_x - b_x).abs();
//     let dy = (a_y - b_y).abs();
//
//     // 10 * (dx + dy) + 14 * std::cmp::min(dx, dy)
//     if dx > dy {
//         14 * dy + 10 * (dx - dy)
//     } else {
//         14 * dx + 10 * (dy - dx)
//     }
// }

fn offset_position(
    offset_x: isize,
    offset_y: isize,
    current: MapPos,
    width: usize,
) -> isize {
    let c_x = (current % width) as isize;
    let c_y = (current / width) as isize;

    let n_x = c_x + offset_x;
    let n_y = c_y + offset_y;

    n_x + (n_y * width as isize)
}

// fn smooth_path(
//     current: (isize, isize),
//     next: (isize, isize),
// ) -> (usize, usize) {
//     let mut ret_x = current.0;
//     let mut ret_y = current.1;
//
//     for (o_x, o_y) in NEIGHBORS {
//         let cn_x = current.0 + o_x;
//         let cn_y = current.1 + o_y;
//
//         if cn_x == next.0 && cn_y == next.1 {
//             break;
//         }
//     }
//
//     (ret_x as usize, ret_y as usize)
// }

fn make_path(
    coordinates: &HashMap<MapPos, MapPos>,
    start: MapPos,
    end: MapPos,
    // map_width: usize,
) -> Vec<MapPos> {
    let mut path = vec![];

    let mut current = *coordinates.get(&end).unwrap();

    while current != start {
        path.push(current);
        current = *coordinates.get(&current).unwrap();
        // let next = *coordinates.get(&current).unwrap();

        // let c_x = current % map_width;
        // let c_y = current / map_width;

        // let n_x = next % map_width;
        // let n_y = next / map_width;

        // let new_current = smooth_path(
        //     (c_x as isize, c_y as isize),
        //     (n_x as isize, n_y as isize),
        // );

        // let new_pos = (new_current.0 + (new_current.1 * map_width)) as usize;

        // path.push(new_pos);

        // current = next;
    }

    path.push(start);

    path
}
