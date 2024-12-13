use itertools::Itertools;
use regex::Regex;

pub fn main() {
    let contents = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    let contents = include_str!("../input/day-13-input.txt");

    let reg = Regex::new(r"(\d+)").unwrap();

    let prizes = contents
        .lines()
        .filter(|l| l.len() > 1)
        .map(|l| l.replace(',', ""))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| c.join(" "))
        .map(|prize| {
            reg.captures_iter(&prize)
                .map(|s| s.extract::<1>().1[0].parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // part 1
    // brute force 101^2 permutations

    let tokens = prizes.iter().fold(0, |sum, prize| {
        let a_x = prize[0];
        let a_y = prize[1];
        let b_x = prize[2];
        let b_y = prize[3];
        let p_x = prize[4];
        let p_y = prize[5];

        let win = (0..=100)
            .permutations(2)
            .map(|ab| {
                let a = ab[0];
                let b = ab[1];
                let result = (a * a_x + b * b_x == p_x) && (a * a_y + b * b_y == p_y);
                (result).then_some(3 * a + b)
            })
            .find_map(|x| x); // there can -only- be one valid combination of a and b

        if let Some(win) = win {
            return sum + win;
        }
        sum
    });
    println!("{:?}", tokens);

    // part 2
    // Cramer's rule -- https://i.ytimg.com/vi/-GfPV2eexJo/maxresdefault.jpg
    // https://old.reddit.com/r/adventofcode/comments/1hd4wda/2024_day_13_solutions/m1wbl25/

    let tokens = prizes.iter().fold(0, |sum, prize| {
        let a_x = prize[0] as isize;
        let a_y = prize[1] as isize;
        let b_x = prize[2] as isize;
        let b_y = prize[3] as isize;
        let p_x = prize[4] as isize + 10_000_000_000_000;
        let p_y = prize[5] as isize + 10_000_000_000_000;

        let denom = a_y * b_x - b_y * a_x;
        let a_det = b_x * p_y - b_y * p_x;
        let b_det = a_y * p_x - a_x * p_y;

        let a = a_det as f64 / denom as f64; // f32 won't have enough space for decimals!
        let b = b_det as f64 / denom as f64;

        if a.fract() == 0.0 && b.fract() == 0.0 && a > 0.0 && b > 0.0 {
            return sum + (3 * (a as usize)) + (b as usize);
        }

        sum
    });
    println!("{:?}", tokens);
}
