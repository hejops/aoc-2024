use regex::Regex;

pub fn main() {
    let contents = include_str!("../input/day-3-input.txt");

    // part 1
    let patt = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = contents
        .lines()
        .map(|line| {
            patt.captures_iter(line)
                .map(|c| c.extract::<2>())
                .map(|(_, matches)| {
                    matches[0].parse::<usize>().unwrap() * matches[1].parse::<usize>().unwrap()
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{:?}", sum);

    // part 2
    let outer_patt = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
    let inner_patt = Regex::new(r"(\d+),(\d+)").unwrap();
    let s = &contents.lines().collect::<Vec<&str>>().join("");
    let mut enabled = true;
    let mut sum = 0;
    outer_patt.captures_iter(s).for_each(|outer_match| {
        let m = outer_match.extract::<1>().0;
        match m {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if !enabled {
                    return;
                }
                sum += inner_patt
                    .captures_iter(m)
                    .map(|c| c.extract::<2>())
                    .map(|(_, matches)| {
                        matches[0].parse::<usize>().unwrap() * matches[1].parse::<usize>().unwrap()
                    })
                    .sum::<usize>()
            }
        }
    });
    println!("{:?}", sum);
}
