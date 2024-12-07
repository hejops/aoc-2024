use std::fs;
use std::usize;

use itertools::Itertools;

fn debug(
    nums: Vec<usize>,
    ops: Vec<char>,
) {
    // i tried using `intersperse_with`, but that was an abject failure
    let mut eqn = vec![];
    for i in 0..ops.len() {
        eqn.push(nums[i].to_string());
        eqn.push(
            {
                match ops[i] {
                    '0' => '+',
                    '1' => 'x',
                    _ => unreachable!(),
                }
            }
            .to_string(),
        );
    }
    eqn.push(nums[nums.len() - 1].to_string());
    println!("{:?}", eqn.join(" "));
}

pub fn main() {
    let contents = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    let contents = fs::read_to_string("day-7-input.txt").unwrap();

    fn generate_op_sequences(len: usize) -> impl Iterator<Item = Vec<char>> {
        (0..(2_usize).pow(len.try_into().unwrap())).map(move |n| {
            format!(
                "{:01$b}", // https://stackoverflow.com/a/26286238
                n, len,
            )
            .chars()
            .collect::<Vec<_>>()
        })
    }

    for i in 1..=10 {
        let combs = generate_op_sequences(i);
        assert_eq!(combs.count(), 2_usize.pow(i.try_into().unwrap()),);
    }
    // return;

    // part 1
    // WARN: 2^k
    let sum = contents
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter_map(|equation| {
            let target = equation[0].trim_end_matches(':').parse::<usize>().unwrap();
            let nums = equation[1..]
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            generate_op_sequences(nums.len() - 1)
                .map(|seq| {
                    nums[1..]
                        .iter()
                        .zip(seq)
                        .fold(nums[0], |sum, (next, op)| match op {
                            '0' => sum + next,
                            '1' => sum * next,
                            _ => unreachable!(),
                        })
                        .eq(&target)
                        .then_some(target)
                })
                .find_map(|x| x)
                .eq(&Some(target))
                .then_some(target)
        })
        .sum::<usize>();
    println!("{:?}", sum);
    // return;

    // part 2
    // WARN: 3^k

    // non-FP-style
    // real    0m19.528s
    // user    0m19.310s
    // sys     0m0.193s

    // FP-style is about 2x faster
    // real    0m11.536s
    // user    0m11.375s
    // sys     0m0.198s

    fn generate_op_sequences_3(len: usize) -> Vec<String> {
        let characters = ["+", "*", "|"];
        if len == 1 {
            return characters.iter().map(|c| c.to_string()).collect();
        }
        // https://stackoverflow.com/a/67746758
        (2..len).fold(
            characters
                .into_iter()
                .cartesian_product(characters.iter())
                .map(|(a, b)| a.to_owned() + b)
                .collect(),
            |acc: Vec<String>, _| {
                acc.into_iter()
                    .cartesian_product(characters.iter())
                    .map(|(a, b)| a.to_owned() + b)
                    .collect()
            },
        )
    }
    for i in 1..=10 {
        let combs = generate_op_sequences_3(i);
        // println!("{:?}", combs);
        assert_eq!(
            combs.len(),
            3_usize.pow(i.try_into().unwrap()),
            "{:?}",
            combs
        );
    }

    let sum = contents
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter_map(|equation| {
            let target = equation[0].trim_end_matches(':').parse::<usize>().unwrap();
            let nums = equation[1..]
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            generate_op_sequences_3(nums.len() - 1)
                .iter()
                .map(|seq| {
                    nums[1..]
                        .iter()
                        .zip(seq.chars())
                        .fold(nums[0], |sum, (next, op)| match op {
                            '+' => sum + next,
                            '*' => sum * next,
                            '|' => (sum.to_string() + &next.to_string()).parse().unwrap(),
                            _ => unreachable!(),
                        })
                        .eq(&target)
                        .then_some(target)
                })
                .find_map(|x| x)
                .eq(&Some(target))
                .then_some(target)
        })
        .sum::<usize>();
    println!("{:?}", sum);
}
