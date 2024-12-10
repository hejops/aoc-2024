use std::collections::HashMap;
use std::collections::HashSet;

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
        let left = (pos % cols).gt(&0).then_some(pos - 1);
        let right = ((pos % cols) + 1 < cols && pos + 1 < flat_len).then_some(pos + 1);
        let down = (pos + cols).lt(&flat_len).then_some(pos + cols);

        let neighbours = [up, left, right, down]
            .into_iter()
            .flatten()
            .filter(|neighbour| flat_grid[*neighbour] == value)
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

            endpoints.insert(start, positions.clone());
            next_value += 1;
        }
    }

    let score = endpoints.values().map(|s| s.len()).sum::<usize>();
    println!("{:?}", score);

    // part 2
    let sum = zeros.iter().fold(0, |sum, pos| {
        let route = vec![*pos];
        let mut routes: Vec<Vec<usize>> = vec![route];
        let mut next_value = 1;

        while next_value <= 9 {
            let mut extended_routes: Vec<Vec<usize>> = vec![];

            for route in &mut routes {
                if let Some(neighbours) = get_neighbours(*route.last().unwrap(), next_value) {
                    extended_routes = neighbours.iter().fold(extended_routes, |mut routes, n| {
                        route.push(*n);
                        routes.push(route.to_vec());
                        routes
                    });
                };
            }

            routes = extended_routes;
            next_value += 1;
        }

        sum + routes.len()
    });

    println!("{:?}", sum);
}
