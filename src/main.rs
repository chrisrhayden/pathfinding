// from https://www.redblobgames.com/pathfinding/a-star/introduction.html
#![allow(dead_code)]

mod astar;
mod breadth_first;
mod breadth_first_alt;
mod dijkstra;
// mod dijkstra_heur;
mod map;

use std::collections::HashMap;

use rand::{prelude::*, thread_rng};

// use breadth_first::BreadthFirst;
// use breadth_first_alt::{BreadthFirst, Coordinates};

// use dijkstra::Dijkstra;
// use dijkstra_heur::Dijkstra;

use astar::AStar;

use map::{get_map, MapData, Tile};

type Point = (isize, isize);

fn print_vec(coordinates: &[(isize, isize)], size: usize, width: usize) {
    let mut map = vec!['#'; size];

    for (x, y) in coordinates {
        let i = *x as usize + (*y as usize * width);

        map[i] = ' ';
    }

    for (tile, i) in map.iter().zip(1..) {
        print!("{}", tile);

        if i % width == 0 {
            println!();
        }
    }
}

fn print_hashmap(
    coordinates: &HashMap<usize, usize>,
    map_data: &MapData,
    start: usize,
    end: usize,
) {
    let mut map = vec!['#'; map_data.map.len()];

    for (i, tile) in map_data.map.iter().enumerate() {
        if *tile == Tile::Floor {
            map[i] = ' ';
        }
    }

    let mut current = end;

    let mut path = vec![];

    while current != start {
        path.push(current);

        current = *coordinates.get(&current).unwrap();
    }

    path.push(start);

    for index in path.iter() {
        map[*index] = '+';
    }

    map[*path.first().unwrap()] = 'S';

    map[*path.last().unwrap()] = 'E';

    for (tile, i) in map.iter().zip(1..) {
        print!("{}", tile);

        if i % map_data.map_width == 0 {
            println!();
        }
    }
}

fn heuristic(a: (isize, isize), b: (isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn get_random_point(map_data: &MapData) -> (isize, isize) {
    let mut rng = thread_rng();

    let mut x;
    let mut y;

    loop {
        x = rng.gen_range(0..map_data.map_width);
        y = rng.gen_range(0..map_data.map_height);

        let index = x + (y * map_data.map_width);

        if map_data.map[index] != Tile::Wall {
            break;
        }
    }

    (x as isize, y as isize)
}

fn get_index(point: Point, width: usize) -> usize {
    let i = point.0 + (point.1 * width as isize);

    i as usize
}

fn get_start_and_end(map_data: &MapData) -> (usize, usize) {
    let start = get_random_point(map_data);

    let mut end;

    loop {
        end = get_random_point(map_data);

        let h = heuristic(start, end);

        println!("h {:?}", h);
        if h > 50 {
            break;
        }
    }

    let start = get_index(start, map_data.map_width);
    let end = get_index(end, map_data.map_width);

    (start, end)
}

fn main() {
    let width = 60;
    let height = 60;
    // let seed = 12345678910;
    let seed = 2739832984732098742;

    let map_data = get_map(width, height, seed);

    get_start_and_end(&map_data);
    let start = get_random_point(&map_data);
    let start = get_index(start, map_data.map_width);
    let end = get_random_point(&map_data);
    let end = get_index(end, map_data.map_width);

    let mut astar = AStar::default();

    let coordinates = astar.run(&map_data, start, end);

    // print_vec(&map, map_data.map.len(), map_data.map_width);
    print_hashmap(&coordinates, &map_data, start, end);
}
