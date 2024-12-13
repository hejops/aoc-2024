use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Sub;
use std::usize;

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
MMMISSJEEE";

    let contents = "AAAA
BBCD
BBCC
EEEC";

    // let contents = include_str!("../input/day-12-input.txt");

    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();
    // println!("{:?} {}", rows, cols); // 140*140

    let flat_grid = contents.replace('\n', "").chars().collect::<Vec<char>>();

    let get_neighbours = |pos: usize, c: &char| -> HashSet<usize> {
        let up = pos.checked_sub(cols);
        let left = ((pos % cols) > 0).then(|| pos - 1);
        let right = ((pos % cols) + 1 < cols).then(|| pos + 1);
        let down = ((pos / rows) + 1 < rows).then(|| pos + rows);

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
        edges.insert(pos, 4.sub(neighbours.len()));
        regions.insert(pos, HashSet::from_iter(vec![pos]));

        while !neighbours.is_empty() {
            let mut all_neighbours = HashSet::new();

            for n in neighbours {
                seen.insert(n);
                let new = get_neighbours(n, c);
                edges.insert(n, 4.sub(new.len()));
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
}
