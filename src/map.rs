use std::cmp::{max, min};

use rand::{prelude::*, Rng};

#[derive(Debug)]
pub struct MapRect {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl MapRect {
    pub fn new(x1: usize, y1: usize, width: usize, height: usize) -> Self {
        Self {
            x1,
            y1,
            x2: x1 + width,
            y2: y1 + height,
        }
    }

    pub fn intersects(&self, other: &MapRect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }

    pub fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
}

pub struct MapData {
    pub seed: u64,
    pub max_rooms: usize,
    pub max_room_size: usize,
    pub min_room_size: usize,
    pub map_width: usize,
    pub map_height: usize,
    pub map: Vec<Tile>,
}

impl MapData {
    pub fn new(map_width: usize, map_height: usize, seed: u64) -> Self {
        let size: usize = map_width as usize * map_height as usize;

        let map: Vec<Tile> = vec![Tile::Wall; size];

        Self {
            seed,
            max_rooms: 30,
            max_room_size: 8,
            min_room_size: 4,
            map_width,
            map_height,
            map,
        }
    }

    pub fn gen(&mut self) {
        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut rooms: Vec<MapRect> = Vec::new();

        let room_width = rng.gen_range(self.min_room_size..=self.max_room_size);
        let room_height =
            rng.gen_range(self.min_room_size..=self.max_room_size);

        let x = rng.gen_range(1..(self.map_width - room_width - 1));
        let y = rng.gen_range(1..(self.map_height - room_height - 1));

        let room = MapRect::new(x, y, room_width, room_height);

        self.carve_out_room(&room);

        rooms.push(room);

        'make_rooms: for _ in 0..self.max_rooms {
            let room_width =
                rng.gen_range(self.min_room_size..=self.max_room_size);
            let room_height =
                rng.gen_range(self.min_room_size..=self.max_room_size);

            let x = rng.gen_range(1..(self.map_width - room_width - 1));
            let y = rng.gen_range(1..(self.map_height - room_height - 1));

            let room = MapRect::new(x, y, room_width, room_height);

            for r in &rooms {
                if r.intersects(&room) {
                    continue 'make_rooms;
                }
            }

            self.carve_out_room(&room);

            let last_room = rooms.last().unwrap();
            self.carve_out_hallway(&mut rng, &room, last_room);

            rooms.push(room);
        }
    }

    fn carve_out_room(&mut self, room: &MapRect) {
        for y in room.y1..=room.y2 {
            for x in room.x1..=room.x2 {
                let index = x + (y * self.map_width);

                self.map[index as usize] = Tile::Floor;
            }
        }
    }

    fn carve_out_hallway(
        &mut self,
        rng: &mut StdRng,
        room: &MapRect,
        past_room: &MapRect,
    ) {
        let (c_x, c_y) = room.center();
        let (p_x, p_y) = past_room.center();

        // start from either the past room or current room
        let (sx, sy) = if rng.gen_bool(0.5) {
            (c_x, p_y)
        } else {
            (p_x, c_y)
        };

        let min_x = min(p_x, c_x);
        let max_x = max(p_x, c_x);

        for x in min_x..=max_x {
            let index = x + (sy * self.map_width);

            self.map[index as usize] = Tile::Floor;
        }

        let min_y = min(p_y, c_y);
        let max_y = max(p_y, c_y);

        for y in min_y..=max_y {
            let index = sx + (y * self.map_width);

            self.map[index as usize] = Tile::Floor;
        }
    }
}

pub fn get_map(width: usize, height: usize, seed: u64) -> MapData {
    let mut map = MapData::new(width, height, seed);

    map.gen();

    map
}

#[allow(dead_code)]
pub fn print_map(map: &MapData) {
    let width = map.map_width as usize;

    for (tile, i) in map.map.iter().zip(1..) {
        let num = if *tile == Tile::Wall { 5 } else { 0 };

        print!("{}", num);

        if i % width == 0 {
            println!();
        } else {
            print!(", ");
        }
    }
}
