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

    /// get rightmost filled block ([start, end]) that contains the requested
    /// `value`
    fn get_right_block(
        blocks: &[Option<usize>],
        value: usize,
    ) -> (usize, usize) {
        let start = blocks.iter().position(|n| *n == Some(value)).unwrap();
        let end = blocks.iter().rposition(|n| *n == Some(value)).unwrap();
        (start, end)
    }

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
        let right = get_right_block(&blocks2, right_val);
        let left = get_left_block(&blocks2, right.1 - right.0 + 1);

        if let Some(left) = left {
            if left.0 < right.0 {
                (right.0..=right.1)
                    .zip(left.0..=left.1)
                    .for_each(|(right, left)| blocks2.swap(right, left));
            }
        }

        right_val -= 1;
    }

    let sum = blocks2.iter().enumerate().fold(0, |sum, (i, block)| {
        sum + {
            if let Some(n) = block {
                i * n
            } else {
                0
            }
        }
    });

    println!("{:?}", sum);
}
