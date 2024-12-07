use std::fs;

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

    fn generate_bit_combinations(len: usize) -> Vec<Vec<char>> {
        (0..(2_usize).pow(len.try_into().unwrap()))
            // https://stackoverflow.com/a/26286238
            .map(|n| format!("{:01$b}", n, len).chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    for i in 1..=10 {
        let combs = generate_bit_combinations(i);
        assert_eq!(combs.len(), 2_usize.pow(i.try_into().unwrap()),);
    }
    // return;

    // part 1
    // WARN: 2^k
    let sum = contents
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter_map(|equation| {
            let target = equation[0].trim_end_matches(':').parse::<usize>().unwrap();
            let nums = equation
                .iter()
                .skip(1)
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let seqs = generate_bit_combinations(nums.len() - 1);

            for ops_seq in seqs {
                let mut curr = nums[0];
                for (i, num) in nums[1..].iter().enumerate() {
                    let op = ops_seq[i];
                    match op {
                        '0' => curr += num,
                        '1' => curr *= num,
                        _ => unreachable!(),
                    }
                }
                if curr == target {
                    // println!("ok: {:?} {:?} {:?}", target, nums, ops_seq);
                    // debug(nums, ops_seq);
                    return Some(target);
                }
            }
            None
        })
        .sum::<usize>();
    println!("{:?}", sum);

    // part 2
    // WARN: 3^k
    fn generate_3bit_combinations(len: usize) -> Vec<String> {
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
        let combs = generate_3bit_combinations(i);
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
            let nums = equation
                .iter()
                .skip(1)
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let seqs = generate_3bit_combinations(nums.len() - 1);

            for ops_seq in seqs {
                let mut curr = nums[0];
                for (i, num) in nums[1..].iter().enumerate() {
                    let op = ops_seq.chars().nth(i).unwrap();
                    match op {
                        '+' => curr += num,
                        '*' => curr *= num,
                        '|' => curr = (curr.to_string() + &num.to_string()).parse().unwrap(),
                        _ => unreachable!(),
                    }
                }
                if curr == target {
                    return Some(target);
                }
            }
            None
        })
        .sum::<usize>();
    println!("{:?}", sum);
}
