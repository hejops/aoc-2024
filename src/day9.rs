pub fn main() {
    let contents = "12345";
    let contents = "2333133121414131402";
    let contents = include_str!("../input/day-9-input.txt");

    let blocks = contents
        .chars()
        .filter_map(|n| n.to_string().parse::<usize>().ok())
        .enumerate()
        .fold(vec![], |mut b: Vec<Option<usize>>, (i, num)| {
            b.extend(vec![(i & 1 == 0).then_some(i / 2); num]);
            b
        });

    // part 1

    // relatively slow
    let mut blocks1 = blocks.clone();
    let mut left = 0;
    let mut right = blocks1.iter().rposition(|n| n.is_some()).unwrap();
    while left < blocks1.len() && left < right {
        if blocks1[left].is_none() {
            blocks1.swap(left, right);
            right = blocks1.iter().rposition(|n| n.is_some()).unwrap();
        }
        left += 1;
    }

    let sum = blocks1
        .iter()
        .flatten()
        .enumerate()
        .fold(0, |sum, (i, n)| sum + i * n);
    println!("{:?}", sum);

    // part 2
    // NOTE: storing gaps in a HashMap {gap length: [positions]} is not trivial,
    // because on any move event, both the newly unfreed block and the newly
    // freed block must be accounted for in the HashMap

    /// get leftmost free block ([start, end]) with the requested `length`
    fn get_left_block(
        blocks: &[Option<usize>],
        length: usize,
    ) -> Option<(usize, usize)> {
        let mut start = 0;
        let mut block_started = false;

        for (i, block) in blocks.iter().enumerate() {
            if block.is_none() && !block_started {
                start = i;
                block_started = true;
            }
            if block.is_some() && block_started {
                if i - start >= length {
                    return Some((start, i - 1));
                }

                start = i;
                block_started = false;
            }
        }
        // println!("could not find block with len {}", length);
        None
    }

    let mut blocks2 = blocks.clone();

    // although maintaining 4 state variables looks weird (and the while condition
    // looks much uglier), this is apparently 2x faster than only maintaining
    // right_val (which would require reinstantiating right_start, right_end, left
    // on every iteration)
    let mut right_val = blocks2.iter().rfind(|n| n.is_some()).unwrap().unwrap();
    let mut right_start = blocks2.iter().position(|n| *n == Some(right_val)).unwrap();
    let mut right_end = blocks2.iter().rposition(|n| *n == Some(right_val)).unwrap();
    let mut left = get_left_block(&blocks2, right_end - right_start + 1);

    while right_val > 0 && left.is_some() && left.unwrap().0 < right_start {
        let (left_start, left_end) = left.unwrap();
        (right_start..=right_end)
            .zip(left_start..=left_end)
            .for_each(|(right, left)| blocks2.swap(right, left));

        right_val -= 1;
        right_start = blocks2.iter().position(|n| *n == Some(right_val)).unwrap();
        right_end = blocks2.iter().rposition(|n| *n == Some(right_val)).unwrap();
        left = get_left_block(&blocks2, right_end - right_start + 1);
    }

    let sum = blocks2.iter().enumerate().fold(0, |sum, (i, block)| {
        if let Some(n) = block {
            return sum + i * n;
        }
        sum
    });

    println!("{:?}", sum);
}
