use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::usize;

use itertools::Itertools;

pub fn main() {
    let contents = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"; // 15*15, 7036

    let contents = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"; // 17*17

    // let contents = include_str!("../input/day-16-input.txt");
    let cols = contents.lines().count();

    let flat_grid = contents.replace('\n', "").chars().collect::<Vec<char>>();
    // println!("{:?}", flat_grid.len());

    fn print_grid(
        flat_grid: &[char],
        path: Vec<&usize>,
        curr_pos: usize,
        cols: usize,
    ) {
        let mut g = flat_grid.to_owned().clone();
        g[curr_pos] = 'X';
        for p in path {
            g[*p] = 'X';
        }
        for l in g.chunks(cols) {
            println!("{}", l.iter().collect::<String>());
        }
    }

    // naive BFS is easy to implement, but is totally intractable for the full input
    // https://sqlpad.io/tutorial/python-maze-solver/

    // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
    // https://www.codingdrills.com/tutorial/introduction-to-graph-algorithms/dijkstra-pseudocode

    let start = flat_grid.iter().position(|c| *c == 'S').unwrap();
    println!("start: {:?}", start);

    // tile, distance (taking into account turns; should only be 1|1001)
    let mut distances: HashMap<usize, usize> = HashMap::new();
    let mut prev: HashMap<usize, Option<usize>> = HashMap::new();
    for (i, c) in flat_grid.iter().enumerate() {
        if c != &'#' {
            distances.insert(i, usize::MAX);
            prev.insert(i, None);
        }
    }
    distances.insert(start, 0);

    let mut unvisited: BinaryHeap<Reverse<(usize, isize)>> = BinaryHeap::new(); // max heap
    unvisited.push(Reverse((start, 1))); // (pos, dir)

    while !unvisited.is_empty() {
        let curr_pos = unvisited.pop().unwrap();

        for dir in [-1, 1, -(cols as isize), cols as isize] {
            let neighbour_pos = curr_pos.0 .0.checked_add_signed(dir).unwrap();

            if !distances.contains_key(&neighbour_pos) {
                // tile is wall
                continue;
            }

            let dist = 1 + 1000
                * (if neighbour_pos as isize - curr_pos.0 .0 as isize == curr_pos.0 .1 {
                    0
                } else {
                    1
                }) as usize;

            if dist < *distances.get(&neighbour_pos).unwrap() {
                distances.insert(neighbour_pos, dist);
                prev.insert(neighbour_pos, Some(curr_pos.0 .0));
                unvisited.push(Reverse((neighbour_pos, dir)));
            };
        }

        // println!("next visits: {:?}", unvisited);
    }

    // println!("{:?}", distances);
    // println!("{:?}", prev);

    let mut end_pos = flat_grid.iter().position(|c| *c == 'E').unwrap();
    println!("end: {:?}", end_pos);

    assert!(prev.get(&end_pos).is_some());
    assert!(prev.values().unique().count() > 2);

    let mut sum = 0;
    let mut path = vec![];
    while let Some(Some(x)) = prev.get(&end_pos) {
        // println!("{:?}", x);
        path.push(x);
        // println!("{:?}", distances.get(x));
        sum += distances.get(x).unwrap();
        end_pos = *x;
    }

    // path.reverse();
    // for w in path.windows(2) {
    //     let dir = match *w[1] as isize - *w[0] as isize {
    //         1 => "right",
    //         -1 => "left",
    //         a if a == cols as isize => "down",
    //         a if a == -(cols as isize) => "up",
    //         _ => unreachable!(),
    //     };
    //     println!("{} {} {:?}", w[0], dir, w[1]);
    // }

    print_grid(&flat_grid, path, start, cols);

    println!("{:?}", sum);

    //
}
