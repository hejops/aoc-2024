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
        let left = ((pos % cols) > 0).then_some(pos - 1);
        let right = ((pos % cols) + 1 < cols).then_some(pos + 1);
        let down = ((pos / rows) + 1 < rows).then_some(pos + rows);

        let neighbours = [up, left, right, down]
            .into_iter()
            .flatten()
            .filter(|neighbour| flat_grid[*neighbour] == value)
            .collect::<HashSet<usize>>();

        (!neighbours.is_empty()).then_some(neighbours)
    };

    // part 1
    // behold... the double fold
    let score = zeros.clone().into_iter().fold(0, |sum, start| {
        sum + (1..=9)
            .fold(HashSet::from_iter(vec![start]), |pos, next_value| {
                pos.iter()
                    .filter_map(|pos| get_neighbours(*pos, next_value))
                    .flatten()
                    .collect::<HashSet<_>>()
            })
            .len()
    });
    println!("{:?}", score);

    // part 2
    let score = zeros.iter().fold(0, |sum, pos| {
        let route = vec![*pos];
        let mut routes: Vec<Vec<usize>> = vec![route];
        let mut next_value = 1;

        while next_value <= 9 {
            let mut extended_routes: Vec<Vec<usize>> = vec![];

            for route in &mut routes {
                if let Some(neighbours) = get_neighbours(route[route.len() - 1], next_value) {
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

    println!("{:?}", score);
}
