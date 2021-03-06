use std::io::{self, BufRead};
use std::vec::Vec;

const SLOPES: [[usize; 2]; 5] = [
    [1, 1],
    [3, 1],
    [5, 1],
    [7, 1],
    [1, 2]
];

fn next_move([row_move, col_move]: [usize; 2], loc: [usize; 2], map: &Vec<Vec<char>>) -> [usize; 2]{
    let mut next_x = loc[1] + row_move;
    // if next_x goes out of bounds, yeet back to start
    if next_x >= map[0].len() {
        next_x -= map[0].len();
    }
    let next_y = loc[0] + col_move;

    [next_y, next_x]
}

fn recursion(count: u16, iteration: u16, slope: [usize; 2], loc: [usize; 2], map: &Vec<Vec<char>>) -> (u16, u16) {
    let next_loc = next_move(slope, loc, map);
    if next_loc[0] < map.len() {
        match map[next_loc[0]][next_loc[1]] {
            '#' => recursion(count + 1, iteration + 1, slope, next_loc, map),
            '.' => recursion(count, iteration + 1, slope, next_loc, map),
            _ => panic!("character is neither '#' or '.'")
        }
    } else {
        (count, iteration)
    }
}

fn main() {
    let stdin = io::stdin();
    let map: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|line| {
            line
            .unwrap()
            .chars()
            .collect()
        }).collect::<Vec<Vec<char>>>();

    let count;
    if map[0][0] == '#' {
        count = 1;
    } else {
        count = 0
    }

    let mut results: [u32; SLOPES.len()] = [0; SLOPES.len()];

    for (i, slope) in SLOPES.iter().enumerate() {
        let result = recursion(count, 0, *slope, [0,0], &map);
        println!("count: {}, iterations: {}", result.0, result.1);
        results[i] = result.0 as u32;
    }

    println!("result: {}", results.iter().product::<u32>());
}
