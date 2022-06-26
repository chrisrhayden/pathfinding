// from https://www.redblobgames.com/pathfinding/a-star/introduction.html
#![allow(dead_code)]

mod astar;
mod breadth_first;
mod breadth_first_alt;
mod dijkstra;
mod map;

use rand::{prelude::*, thread_rng};

// use breadth_first::BreadthFirst;
// use breadth_first_alt::{BreadthFirst, Coordinates};

// use dijkstra::Dijkstra;
// use dijkstra_heur::Dijkstra;

use astar::AStar;

use map::{get_map, MapData, MapPos, Tile};

type Point = (usize, usize);

fn heuristic(a: Point, b: Point) -> isize {
    (a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()
}

fn get_random_point(map_data: &MapData) -> (usize, usize, usize) {
    let mut rng = thread_rng();

    let mut x;
    let mut y;
    let mut index;

    loop {
        x = rng.gen_range(0..map_data.map_width);
        y = rng.gen_range(0..map_data.map_height);

        index = x + (y * map_data.map_width);

        if map_data.map[index] != Tile::Wall {
            break;
        }
    }

    (x, y, index)
}

fn get_index(point: Point, width: usize) -> usize {
    point.0 + (point.1 * width)
}

fn get_start_and_end(map_data: &MapData) -> (usize, usize) {
    let mut start;
    let mut end;

    loop {
        start = get_random_point(map_data);
        end = get_random_point(map_data);

        let h = heuristic((start.0, start.1), (end.0, end.1));

        if h > 50 {
            break;
        }
    }

    (start.2, end.2)
}

fn print_map_and_path(map_data: &MapData, path: &[MapPos]) {
    let mut char_map = vec![' '; map_data.map.len()];

    for (i, tile) in map_data.map.iter().enumerate() {
        if *tile == Tile::Wall {
            char_map[i] = '#';
        }
    }

    for step in path {
        char_map[*step] = '+';
    }

    let start = path.last().unwrap();
    let end = path.first().unwrap();

    char_map[*start] = 'S';
    char_map[*end] = 'E';

    let map_width = map_data.map_width;
    for (i, c) in (1..).zip(char_map) {
        if c == 'S' || c == 'E' {
            print!("\x1b[31m{}\x1b[0m", c)
        } else {
            print!("{}", c);
        }

        if i % map_width == 0 {
            println!();
        }
    }
}

fn main() {
    let width = 60;
    let height = 60;
    // let seed = 12345678910;
    let seed = 2739832984732098742;

    let map_data = get_map(width, height, seed);

    // let (start, end) = get_start_and_end(&map_data);
    // let end = get_random_point(&map_data);
    // let start = get_random_point(&map_data);
    // let end = get_random_point(&map_data);
    let start = 825;
    let end = 2712;

    println!("start {:?} end {:?}", start, end);
    let mut astar = AStar::default();

    let path = astar.run(&map_data, start, end);

    print_map_and_path(&map_data, &path);
}
