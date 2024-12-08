use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn debug_print(
    mut flat_grid: Vec<char>,
    position: usize,
) {
    flat_grid[position] = 'O';
    let x = flat_grid
        .chunks(10)
        .map(|v| v.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
        .replace('.', " ");
    println!("{}", x);
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

pub fn main() {
    let contents = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    let contents = include_str!("../input/day-6-input.txt");

    let rows = contents.lines().count(); // 130*130
    let last = contents.lines().count().pow(2);

    let right = 1;
    let down: isize = rows.try_into().unwrap();
    let up = -down;
    let left = -right;

    let flat_grid = contents
        .chars()
        .filter(|c| *c != '\n')
        .collect::<Vec<char>>();
    let obstacles = flat_grid
        .iter()
        .enumerate()
        .filter_map(|(i, c)| c.eq(&'#').then_some(i))
        .collect::<HashSet<_>>();
    // println!("{:#?}", obstacles);

    let mut direction = up;
    let mut position = flat_grid.iter().position(|c| *c == '^').unwrap();
    let mut next = position.checked_add_signed(direction);

    // part 1
    let mut visited = HashSet::new();
    visited.insert(position);

    // `move ||` leads to an infinite loop apparently
    let not_oob = |direction, position, next: Option<usize>| {
        match direction {
            // `up => ...` declares a new variable `up` -inside- the match arm. it does -not- match
            // against `up`, like a Go switch/case would
            // https://stackoverflow.com/a/28226030
            a if a == up => next.is_some(),
            a if a == down => next.unwrap() < last,
            a if a == right => next.unwrap() / rows == position / rows,
            a if a == left => next.unwrap() / rows == position / rows,
            _ => unreachable!(),
        }
    };

    while not_oob(direction, position, next) {
        // println!("now at {}", position);
        // println!(
        //     "next is {} (obstacle: {})",
        //     next.unwrap(),
        //     obstacles.get(next.unwrap()).is_some()
        // );
        while obstacles.contains(&next.unwrap()) {
            // debug_print(flat_grid.clone(), position);
            direction = match direction {
                a if a == up => right,
                a if a == right => down,
                a if a == down => left,
                a if a == left => up,
                _ => unreachable!(),
            };

            // println!("obstacle at {}!", next.unwrap());
            // println!("direction is now {}", direction);
            next = position.checked_add_signed(direction);
            // println!("next is {}", next.unwrap());
        }
        position = next.unwrap();
        next = position.checked_add_signed(direction);
        visited.insert(position);
        // println!("{} {}", visited.len(), position);
    }
    // println!("left map at position {}", position);
    println!("{:#?}", visited.len());

    // return;

    // part 2
    // this makes use of part 1's answer to reduce the search space to 1/3, because
    // only tiles that would have been walked on in the 'default' route are
    // relevant
    let mut infinite = 0;
    for new_obstacle in visited {
        // let mut new_grid = flat_grid.clone(); // only for debug
        // new_grid[x] = '!';

        let mut new_obstacles = obstacles.clone();
        new_obstacles.insert(new_obstacle);

        // Vec
        // real    1m39.745s
        // user    1m39.612s
        // sys     0m0.072s

        // apparently, accessing a HashSet is about 20x faster than Vec :)
        // real    0m5.186s
        // user    0m5.173s
        // sys     0m0.007s

        let mut stopped_positions: HashMap<usize, HashSet<isize>> = HashMap::new();

        let mut direction = up;
        let mut position = flat_grid.iter().position(|c| *c == '^').unwrap();
        let mut next = position.checked_add_signed(direction);

        'outer: while not_oob(direction, position, next) {
            while new_obstacles.contains(&next.unwrap()) {
                // debug_print(new_grid.clone(), position);

                // https://users.rust-lang.org/t/what-is-the-difference-between-map-and-and-then/29108/3
                // https://davirain.xlog.app/and_then-he-map-zai-shi-yong-shang-you-shen-me-qu-bie?locale=en
                // map:      F: FnOnce(T) -> U
                // and_then: F: FnOnce(T) -> Option<U> (preserves shape)

                // usually, drilling down the 'dumb' way will make the compiler tell you to do
                // the following:
                // map+filter(is_some)+map(unwrap) -> map+flatten -> and_then

                // we have stopped at the same position coming from the same direction
                if stopped_positions
                    .get(&position)
                    .and_then(|s| s.get(&direction))
                    .is_some()
                {
                    infinite += 1;
                    // println!("{} = infinite! ({})", x, infinite);
                    break 'outer;
                }
                stopped_positions
                    .entry(position)
                    .and_modify(|s| {
                        s.insert(direction);
                    })
                    .or_insert(HashSet::from_iter(vec![direction].into_iter()));

                direction = match direction {
                    a if a == up => right,
                    a if a == right => down,
                    a if a == down => left,
                    a if a == left => up,
                    _ => unreachable!(),
                };

                next = position.checked_add_signed(direction);
            }

            position = next.unwrap();
            next = position.checked_add_signed(direction);
        }
    }
    println!("{:#?}", infinite);
}
