use std::cmp::Ordering;
use std::collections::HashMap;

pub fn main() {
    let contents = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    // an update contains up to 23 pages. 23! = 2.5E22

    let contents = include_str!("../input/day-5-input.txt").unwrap();

    // println!("{}", contents);

    let orderings = contents
        .lines()
        .filter(|line| line.contains('|'))
        .collect::<Vec<_>>();

    let updates = contents.lines().filter(|line| line.contains(','));

    // println!("{:#?}", updates);

    // part 1
    let correct = updates
        .clone()
        .map(|pages| {
            pages
                .split(',')
                .collect::<Vec<&str>>()
                .windows(2)
                .map(|w| w.join("|"))
                .collect::<Vec<String>>()
        })
        .enumerate()
        .filter(|(_, page_pairs)| {
            page_pairs
                .iter()
                .all(|pair| orderings.contains(&pair.as_str()))
        })
        .collect::<Vec<(usize, _)>>(); // store indices for easier filtering in part 2
    let sum = correct
        .iter()
        // for a vec of 2n items, get the nth item, split it at |, take the 1st
        .map(|(_, pairs)| {
            pairs[pairs.len() / 2]
                .split('|')
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>();
    println!("{:?}", sum);

    // part 2
    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();
    orderings.iter().for_each(|pair| {
        let mut ints = pair.split('|');
        let i1 = ints.next().unwrap();
        let i2 = ints.next().unwrap();
        adjacency
            .entry(i1)
            .and_modify(|v| v.push(i2))
            // .or_default() // this wasted an hour of my time...
            .or_insert(vec![i2]);
    });
    // println!("{:?}", adjacency);

    // https://github.com/copperfield42/Advent-of-Code-2024/blob/main/day%2005/part2.py

    let incorrect = updates
        .enumerate()
        .filter(|(i, _)| !correct.iter().map(|x| x.0).any(|x| &x == i))
        .map(|(_, nums)| {
            let mut nums = nums.split(',').collect::<Vec<_>>();
            nums.sort_by(|a, b| match adjacency.get(a) {
                Some(next) => {
                    if next.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                None => Ordering::Equal,
            });
            nums[nums.len() / 2].parse::<usize>().unwrap()
        })
        .sum::<usize>();

    println!("{:?}", incorrect);
}

// why did i even think of DFS?

// pub fn find_solution<'a>(
//     state: &mut Vec<&'a str>,
//     remaining: Vec<&'a str>,
//     target_len: usize,
//     connections: &HashMap<&'a str, Vec<&'a str>>,
// ) -> Vec<&'a str> {
//     // {
//     // "29": [],
//     // "47": ["13", "61", "29"],
//     // "53": ["13"],
//     // "61": ["53", "29"],
//     // "75": ["53", "47", "61", "13"],
//     // "97": ["61", "47", "29", "53", "75"]
//     // }
//
//     // 75,97,47,61,53 becomes 97,75,47,61,53.
//
//     println!(/* "NEW ITERATION" */);
//     println!("current state {:?}", state);
//     println!("remaining {:?}", remaining);
//
//     if remaining.is_empty() {
//         if state.len() == target_len {
//             return state.to_vec();
//         } else {
//             println!("need to back out!");
//             panic!()
//         }
//     }
//
//     for x in &remaining {
//         state.push(x);
//         match connections.get(state.last().unwrap()) {
//             Some(conns) => {
//                 // remove current element
//                 // let remaining = remaining
//                 //     .clone()
//                 //     .into_iter()
//                 //     .filter(|n| n != x)
//                 //     .collect::<Vec<_>>();
//                 let next = conns
//                     // .clone()
//                     // .into_iter()
//                     // .filter(|conn| remaining.contains(conn))
//                     // .collect::<Vec<_>>()
//                     .to_vec();
//                 println!("{} is connected to {:?}", x, next);
//                 println!("starting new search {:?} {:?}\n", state, next);
//                 return find_solution(state, next, target_len, connections);
//             }
//             None => {
//                 println!("{} is dead end", x);
//                 panic!()
//             }
//         };
//     }
//     panic!()
// }
