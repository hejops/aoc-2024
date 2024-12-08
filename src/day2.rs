pub fn main() {
    let contents = include_str!("../input/day-2-input.txt");

    // part 1: happy path
    let safe = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<isize>>()
        })
        .filter(|diffs| {
            let min = diffs.iter().min().unwrap();
            let max = diffs.iter().max().unwrap();
            (1 <= *min && *max <= 3) || (-3 <= *min && *max <= -1)
        })
        .count();
    println!("{:#?}", safe);

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
                let diffs = ints
                    .iter()
                    .enumerate()
                    .filter_map(|(i, c)| (!i.eq(&idx)).then_some(c))
                    .collect::<Vec<_>>()
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
