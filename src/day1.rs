pub fn main() {
    let contents = include_str!("../input/day-1-input.txt");

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

    // part 1
    lefts.sort();
    rights.sort();
    let sum = lefts
        .iter()
        .zip(rights.clone())
        .map(|(l, r)| l.abs_diff(r))
        .sum::<usize>();
    println!("{:#?}", sum);

    // part 2
    let sum = lefts
        .iter()
        .map(|l| rights.iter().filter(move |r| *r == l).count() * *l)
        .sum::<usize>();
    println!("{:#?}", sum);
}
