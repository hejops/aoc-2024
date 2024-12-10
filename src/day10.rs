use std::collections::HashMap;
use std::collections::HashSet;
use std::usize;

pub fn main() {
    let contents = "89010123
                    78121874
                    87430965
                    96549874
                    45678903
                    32019012
                    01329801
                    10456732";

    let contents = include_str!("../input/day-10-input.txt"); // 57*57

    let rows = contents.lines().count();
    let cols = contents.lines().next().unwrap().len();
    let flat_len = rows * cols;

    let flat_grid = contents
        .chars()
        .filter_map(|n| n.to_string().parse::<u8>().ok())
        .collect::<Vec<_>>();
    // println!("{:?}", flat_grid);

    let zeros = flat_grid
        .iter()
        .enumerate()
        .filter_map(|(i, n)| n.eq(&0).then_some(i))
        .collect::<Vec<_>>();
    // println!("{:?}", zeros);

    let get_neighbours = |pos: usize, value: u8| -> Option<HashSet<usize>> {
        let up = pos.checked_sub(cols);
        let left = (pos % cols).gt(&0).then(|| pos.checked_sub(1).unwrap());
        let right = {
            if (pos % cols) < cols - 1 && pos + 1 < flat_len {
                Some(pos + 1)
            } else {
                None
            }
        };
        let down = (pos + cols).lt(&flat_len).then_some(pos + cols);

        let neighbours = [up, left, right, down]
            .into_iter()
            .flatten()
            .filter(|dir| flat_grid[*dir] == value)
            .collect::<HashSet<usize>>();

        (!neighbours.is_empty()).then_some(neighbours)
    };

    // part 1
    let mut endpoints = HashMap::new();

    for pos in zeros.clone() {
        let start = pos;
        let mut positions = HashSet::from_iter(vec![pos]);
        let mut next_value = 1;
        while !positions.is_empty() && next_value <= 9 {
            positions = positions
                .iter()
                .filter_map(|pos| get_neighbours(*pos, next_value))
                .flatten()
                .collect::<HashSet<_>>();
            next_value += 1;
            endpoints.insert(start, positions.clone());
        }
    }

    let score = endpoints.values().map(|s| s.len()).sum::<usize>();
    println!("{:?}", score);

    // part 2
    let mut sum = 0;

    for pos in zeros {
        let route = vec![pos];
        let mut routes: Vec<Vec<usize>> = vec![route];
        let mut next_value = 1;

        while next_value <= 9 {
            let mut extended_routes: Vec<Vec<usize>> = vec![];

            for route in &routes {
                if let Some(neighbours) = get_neighbours(*route.last().unwrap(), next_value) {
                    for n in neighbours {
                        let mut extended_route: Vec<usize> = route.clone();
                        extended_route.push(n);
                        extended_routes.push(extended_route);
                    }
                };
            }

            routes = extended_routes;
            next_value += 1;
        }

        sum += routes.len();
    }
    println!("{:?}", sum);
}
