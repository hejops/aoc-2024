use std::fs;

pub fn main() {
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
