use std::collections::HashMap;
use std::collections::HashSet;

pub fn main() {
    let contents = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"; // 10*10

    let contents = include_str!("../input/day-12-input.txt");

    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();
    // println!("{:?} {}", rows, cols); // 140*140

    let flat_grid = contents.replace('\n', "").chars().collect::<Vec<char>>();

    let get_neighbours = |pos: usize, c: &char| -> HashSet<usize> {
        let up = pos.checked_sub(cols);
        // cannot use then_some for subtraction, because the arg is eagerly evaluated
        let left = ((pos % cols) > 0).then(|| pos - 1);
        let right = ((pos % cols) + 1 < cols).then_some(pos + 1);
        let down = ((pos / rows) + 1 < rows).then_some(pos + rows);

        [up, left, right, down]
            .into_iter()
            .flatten()
            .filter(|neighbour| flat_grid[*neighbour] == *c)
            .collect::<HashSet<usize>>()
    };

    // part 1
    // flood fill
    let mut seen: HashSet<usize> = HashSet::new();
    // { pos: {perimeter, perimeter, ...}, pos: ... }
    let mut edges: HashMap<usize, usize> = HashMap::new();
    let mut regions: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (pos, c) in flat_grid.iter().enumerate() {
        if seen.contains(&pos) {
            continue;
        }

        let mut neighbours = get_neighbours(pos, c);
        seen.insert(pos);
        edges.insert(pos, 4 - neighbours.len());
        regions.insert(pos, HashSet::from_iter(vec![pos]));

        while !neighbours.is_empty() {
            let mut all_neighbours = HashSet::new();

            for n in neighbours {
                seen.insert(n);
                let new = get_neighbours(n, c);
                edges.insert(n, 4 - new.len());
                regions.entry(pos).and_modify(|v| {
                    v.insert(n);
                });
                all_neighbours.extend(new.clone());
            }

            neighbours = all_neighbours
                .into_iter()
                .filter(|n| !seen.contains(n))
                .collect();
        }
    }

    let score = regions
        .values()
        .map(|r| r.len() * r.iter().map(|pos| edges.get(pos).unwrap()).sum::<usize>())
        .sum::<usize>();
    println!("{:?}", score);

    // part 2

    // this is a monstrosity, but i don't care
    let get_corners = |pos: usize, c: &char| -> usize {
        let left_ok = (pos % cols) > 0;
        let right_ok = (pos % cols) + 1 < cols;
        let down_ok = (pos / rows) + 1 < rows;

        let up = pos
            .checked_sub(cols)
            .map(|_| pos - cols)
            .and_then(|pos| flat_grid[pos].eq(c).then_some(pos));
        let left = left_ok
            .then(|| pos - 1)
            .and_then(|pos| flat_grid[pos].eq(c).then_some(pos));
        let right = right_ok
            .then_some(pos + 1)
            .and_then(|pos| flat_grid[pos].eq(c).then_some(pos));
        let down = down_ok
            .then_some(pos + cols)
            .and_then(|pos| flat_grid[pos].eq(c).then_some(pos));

        let nw = pos
            .checked_sub(cols + 1)
            .and_then(|pos| flat_grid[pos].eq(c).then_some(pos));
        let ne = pos
            .checked_sub(cols - 1)
            .and_then(|pos| flat_grid[pos].eq(c).then_some(pos));
        let sw = (down_ok && left_ok)
            .then_some(pos + cols - 1)
            .and_then(|pos| flat_grid[pos].eq(c).then_some(pos));
        let se = (down_ok && right_ok)
            .then_some(pos + cols + 1)
            .and_then(|pos| flat_grid[pos].eq(c).then_some(pos));

        match [up, left, right, down]
            .into_iter()
            .flatten()
            .filter(|neighbour| flat_grid[*neighbour] == *c)
            .count()
        {
            0 => 4,
            1 => 2,
            2 => match 1 {
                _a if up.is_some() && down.is_some() => 0,
                _a if left.is_some() && right.is_some() => 0,
                _a if up.is_some() && left.is_some() => 2 - nw.is_some() as usize,
                _a if up.is_some() && right.is_some() => 2 - ne.is_some() as usize,
                _a if down.is_some() && right.is_some() => 2 - se.is_some() as usize,
                _a if down.is_some() && left.is_some() => 2 - sw.is_some() as usize,
                _ => unreachable!(),
            },
            3 => {
                2 - match 1 {
                    _a if up.is_none() => [sw, se],
                    _a if down.is_none() => [nw, ne],
                    _a if left.is_none() => [ne, se],
                    _a if right.is_none() => [nw, sw],
                    _ => unreachable!(),
                }
                .iter()
                .flatten()
                .count()
            }
            4 => 4 - [nw, ne, sw, se].iter().filter(|d| d.is_some()).count(),
            _ => unreachable!(),
        }
    };

    let mut corners: HashMap<usize, usize> = HashMap::new();
    for (pos, c) in flat_grid.iter().enumerate() {
        corners.insert(pos, get_corners(pos, c));
    }

    let score = regions
        .values()
        .map(|r| r.len() * r.iter().map(|pos| corners.get(pos).unwrap()).sum::<usize>())
        .sum::<usize>();
    println!("{:?}", score);
}
