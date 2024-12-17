use std::collections::HashMap;
use std::collections::VecDeque;
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
#################"; // 17*17, 11048

    let contents = include_str!("../input/day-16-input.txt");
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

    /*
    // this is an incorrect use of min heap. it is not meaningful, since we are taking tile with
    // min position (when we should be taking tile with min distance)
    let mut unvisited: BinaryHeap<Reverse<(usize, isize)>> = BinaryHeap::new(); // default is max heap
    unvisited.push(Reverse((start, 1))); // (curr pos, curr dir)
    */

    let mut unvisited: VecDeque<(usize, isize)> = VecDeque::new();
    unvisited.push_front((start, 1)); // (curr pos, curr dir)

    while !unvisited.is_empty() {
        unvisited // VecDeque cannot be sorted as-is
            .make_contiguous()
            .sort_by_key(|pos| distances.get(&pos.0));
        let curr_tile = unvisited.pop_front().unwrap();
        let (curr_pos, curr_dir) = curr_tile;

        // order of dirs doesn't matter
        for dir in [1, -1, -(cols as isize), cols as isize] {
            let neighbour_pos = curr_pos.checked_add_signed(dir).unwrap();

            if !distances.contains_key(&neighbour_pos) {
                // tile is wall
                continue;
            }

            let dist = 1 + 1000 * (if dir == curr_dir { 0 } else { 1 }) as usize;

            if dist < *distances.get(&neighbour_pos).unwrap() {
                println!(
                    "nei {:?} curr pos {} curr dir {} dist {}",
                    neighbour_pos, curr_pos, curr_dir, dist,
                );

                distances.insert(neighbour_pos, dist);
                prev.insert(neighbour_pos, Some(curr_pos));
                unvisited.push_back((neighbour_pos, dir));
            };
        }
    }

    let mut end_pos = flat_grid.iter().position(|c| *c == 'E').unwrap();

    assert!(prev.get(&end_pos).is_some());
    assert!(prev.values().unique().count() > 2);

    let mut sum = 1;
    let mut path = vec![];
    while let Some(Some(x)) = prev.get(&end_pos) {
        path.push(x);
        sum += distances.get(x).unwrap();
        end_pos = *x;
    }

    print_grid(&flat_grid, path, start, cols);

    println!("{:?}", sum);

    //
}
