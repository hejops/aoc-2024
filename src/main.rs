use std::fs;

fn main() { day2(); }

// https://adventofcode.com/2024/day/1
fn day1() {
    //{{{
    let contents = fs::read_to_string("day-1-input.txt").unwrap();

    // split lines into 2 vecs
    // sort each vec
    // zip vecs and diff

    let mut lefts: Vec<_> = contents
        .lines()
        .map(|l| {
            l.split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    let mut rights: Vec<_> = contents
        .lines()
        .map(|l| {
            l.split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    // // part 1
    // lefts.sort();
    // rights.sort();
    // use std::iter::zip;
    // let sum = zip(lefts, rights)
    //     .map(|(l, r)| l.abs_diff(r))
    //     .sum::<usize>();

    // part 2
    let sum = lefts
        .iter()
        .map(|l| rights.iter().filter(move |r| *r == l).count() * *l)
        .sum::<usize>();

    println!("{:#?}", sum);
} //}}}

// https://adventofcode.com/2024/day/2
fn day2() {
    let contents = fs::read_to_string("day-2-input.txt").unwrap();

    // // part 1: happy path
    // let safe = contents
    //     .lines()
    //     .map(|line| {
    //         line.split_whitespace()
    //             .map(|i| i.parse::<isize>().unwrap())
    //             .collect::<Vec<isize>>()
    //             .windows(2)
    //             .map(|window| window[1] - window[0])
    //             .collect::<Vec<isize>>()
    //     })
    //     .filter(|diffs| {
    //         let min = diffs.iter().min().unwrap();
    //         let max = diffs.iter().max().unwrap();
    //         (1 <= *min && *max <= 3) || (-3 <= *min && *max <= -1)
    //     })
    //     .count();

    // part 2 -- WARN: O(n^2)
    let safe = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .filter(|ints| {
            for idx in 0..ints.len() {
                let mut holed = ints.clone();
                holed.remove(idx);
                let diffs = holed
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect::<Vec<isize>>();
                let min = diffs.iter().min().unwrap();
                let max = diffs.iter().max().unwrap();

                if (1 <= *min && *max <= 3) || (-3 <= *min && *max <= -1) {
                    return true;
                }
            }
            false
        })
        .count();
    println!("{:#?}", safe);
}
