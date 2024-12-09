pub fn main() {
    let contents = "12345";
    let contents = "2333133121414131402";
    let contents = include_str!("../input/day-9-input.txt");

    let blocks = contents
        .chars()
        .filter_map(|n| n.to_string().parse::<usize>().ok())
        .enumerate()
        .fold(vec![], |mut b: Vec<Option<usize>>, (i, num)| {
            match i & 1 == 0 {
                true => b.extend(vec![Some(i / 2); num]),
                false => b.extend(vec![None; num]),
            };
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
    let mut right_val = blocks.iter().rfind(|n| n.is_some()).unwrap().unwrap();

    while right_val > 0 {
        let right_start = blocks2.iter().position(|n| *n == Some(right_val)).unwrap();
        let right_end = blocks2.iter().rposition(|n| *n == Some(right_val)).unwrap();

        let left = get_left_block(&blocks2, right_end - right_start + 1);

        if let Some((left_start, left_end)) = left {
            if left_start < right_start {
                (right_start..=right_end)
                    .zip(left_start..=left_end)
                    .for_each(|(right, left)| blocks2.swap(right, left));
            }
        }

        right_val -= 1;
    }

    let sum = blocks2.iter().enumerate().fold(0, |sum, (i, block)| {
        if let Some(n) = block {
            return sum + i * n;
        }
        sum
    });

    println!("{:?}", sum);
}
