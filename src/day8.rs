use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

pub fn main() {
    //
    let contents = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"; // 12*12

    let contents = fs::read_to_string("day-8-input.txt").unwrap(); // 50*50

    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();
    // println!("{:?}", rows);

    let antennae = contents
        .replace('\n', "")
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != '.')
        .fold(HashMap::new(), |mut map, (i, c)| {
            map.entry(c)
                .and_modify(|s: &mut HashSet<usize>| {
                    s.insert(i);
                })
                .or_insert(HashSet::from_iter(vec![i]));
            map
        });
    // println!("{:?}", antennae);

    fn get_row_diff(
        a: &usize,
        b: &usize,
        len: usize,
    ) -> usize {
        (a / len).abs_diff(b / len)
    }

    fn get_col_diff(
        a: &usize,
        b: &usize,
        len: usize,
    ) -> usize {
        (a % len).abs_diff(b % len)
    }

    // part 1
    let mut antinodes = HashSet::new();
    for positions in antennae.values() {
        positions.iter().combinations(2).for_each(|combi| {
            let raw_diff = combi[1].abs_diff(*combi[0]);

            let row_diff = get_row_diff(combi[1], combi[0], rows);
            let col_diff = get_col_diff(combi[1], combi[0], cols);

            let left = combi.iter().min().unwrap();
            let right = combi.iter().max().unwrap();

            if let Some(prev) = left.checked_sub(raw_diff) {
                if get_row_diff(left, &prev, rows) == row_diff
                    && get_col_diff(left, &prev, cols) == col_diff
                {
                    antinodes.insert(prev);
                }
            }

            if let Some(next) = right.checked_add(raw_diff) {
                if get_row_diff(right, &next, rows) == row_diff
                    && get_col_diff(right, &next, cols) == col_diff
                    && next < rows * cols
                {
                    antinodes.insert(next);
                }
            }
        });
    }
    println!("{:?}", antinodes.len());

    // part 2
    let mut antinodes = HashSet::new();
    for positions in antennae.values().cloned() {
        // include all antennae (without double counting)
        antinodes.extend(positions.clone());

        // mut only for sort
        positions.into_iter().combinations(2).for_each(|combi| {
            let raw_diff = combi[1].abs_diff(combi[0]);
            let row_diff = get_row_diff(&combi[1], &combi[0], rows);
            let col_diff = get_col_diff(&combi[1], &combi[0], cols);

            // repeat add/sub until oob

            let mut left = *combi.iter().min().unwrap();
            let mut right = *combi.iter().max().unwrap();

            while let Some(prev) = left.checked_sub(raw_diff) {
                if get_row_diff(&left, &prev, rows) == row_diff
                    && get_col_diff(&left, &prev, cols) == col_diff
                {
                    // println!("antenna {} formed left antinode {}", left, prev);
                    antinodes.insert(prev);
                    left = prev;
                } else {
                    break;
                }
            }

            while let Some(next) = right.checked_add(raw_diff) {
                if next >= rows * cols {
                    break;
                }
                if get_row_diff(&right, &next, rows) == row_diff
                    && get_col_diff(&right, &next, cols) == col_diff
                {
                    antinodes.insert(next);
                    right = next;
                } else {
                    break;
                }
            }
        });
    }
    println!("{:?}", antinodes.len());
}
