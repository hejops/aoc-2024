use std::collections::HashMap;
use std::ops::AddAssign;

pub fn main() {
    let contents = include_str!("../input/day-11-input.txt");

    // i wasted about an hour on part 2 because i forgot to disable this...
    // let contents = "125 17";

    fn blink(num: &usize) -> Vec<usize> {
        match num {
            0 => vec![1],
            a if (a.to_string().len() & 1 == 0) => {
                let digits = ((*num as f32).log10().floor() as u32) + 1;
                let divisor = 10_usize.pow(digits / 2);
                let x = num / divisor;
                let y = num % divisor;
                vec![x, y]
            }
            _ => vec![num * 2024],
        }
    }

    // part 1
    // brute force vec

    let len = (0..25)
        .fold(
            contents
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
            |v, _| v.iter().flat_map(blink).collect(),
        )
        .len();
    println!("{:?}", len);

    // part 2
    // https://github.com/David-Rushton/AdventOfCode-Solutions/blob/main/2024/cmd/day11/main.go

    let mut nums = HashMap::new();
    for num in contents
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
    {
        nums.entry(num)
            .and_modify(|c: &mut usize| c.add_assign(1))
            .or_insert(1);
    }

    let mut seen: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..75 {
        let mut new_nums = HashMap::new();

        for (num, count) in nums {
            let result = match seen.get(&num) {
                Some(result) => result.clone(),
                None => {
                    let result = blink(&num);
                    seen.entry(num).or_insert(result.clone());
                    result
                }
            };

            for r in result {
                new_nums
                    .entry(r)
                    .and_modify(|c: &mut usize| c.add_assign(count))
                    .or_insert(count);
            }
        }

        nums = new_nums;
    }

    println!("{:?}", nums.values().sum::<usize>());
}
