use std::collections::HashMap;
use std::usize;

use itertools::repeat_n;
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

    let contents = include_str!("../input/day-7-input.txt").unwrap();

    /// permutations with replacement
    fn generate_op_sequences(
        ops: u8,
        len: usize,
    ) -> impl Iterator<Item = Vec<u8>> {
        // (0..(2_usize).pow(len.try_into().unwrap())).map(move |n| {
        //     format!(
        //         "{:01$b}", // https://stackoverflow.com/a/26286238
        //         n, len,
        //     )
        //     .chars()
        //     .collect::<Vec<_>>()
        // })

        repeat_n(0..ops, len).multi_cartesian_product()
    }

    let max_seqs = contents
        .lines()
        .map(|line| line.split_whitespace().count())
        .max()
        .unwrap();

    let mut seqs: HashMap<usize, Vec<Vec<u8>>> = HashMap::new();
    for n in 0..=max_seqs {
        seqs.insert(n, generate_op_sequences(2, n).collect());
    }

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

            seqs.get(&(nums.len() - 1))
                .unwrap()
                .iter()
                .map(|seq| {
                    nums[1..]
                        .iter()
                        .zip(seq)
                        .fold(nums[0], |sum, (next, op)| match op {
                            0 => sum + next,
                            1 => sum * next,
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

    // moving sequence generation out of filter_map is about 33% faster
    // real    0m7.669s
    // user    0m7.593s
    // sys     0m0.113s

    let mut seqs: HashMap<usize, Vec<Vec<u8>>> = HashMap::new();
    for n in 0..=max_seqs {
        seqs.insert(n, generate_op_sequences(3, n).collect());
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

            seqs.get(&(nums.len() - 1))
                .unwrap()
                .iter()
                .map(|seq| {
                    nums[1..]
                        .iter()
                        .zip(seq.iter())
                        .fold(nums[0], |sum, (next, op)| match op {
                            0 => sum + next,
                            1 => sum * next,
                            2 => (sum.to_string() + &next.to_string()).parse().unwrap(),
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
