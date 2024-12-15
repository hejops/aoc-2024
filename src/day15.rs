use std::collections::HashSet;
use std::fmt::Debug;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Object {
    x: usize,
    y: usize,
    // is_box: bool,
    kind: ObjectKind,
}

#[derive(Debug, Clone, PartialEq)]
enum ObjectKind {
    Obstacle,
    Box,
    WideBox,
}

impl Object {
    fn _move(
        &mut self,
        direction: (isize, isize),
    ) {
        if !matches!(self.kind, ObjectKind::Obstacle) {
            let (dx, dy) = direction;
            self.x = self.x.checked_add_signed(dx).unwrap();
            self.y = self.y.checked_add_signed(dy).unwrap();
        }
    }
}

// fn print_grid(
//     grid: &[Object],
//     robot: (usize, usize),
// ) {
//     for row in 0..10 {
//         let mut line = "".to_string();
//         for col in 0..10 {
//             if robot == (col, row) {
//                 line.push('@');
//                 continue;
//             }
//             let c = grid
//                 .iter()
//                 .find(|obj| obj.x == col && obj.y == row)
//                 .map(|obj| if obj.is_box { 'O' } else { '#' })
//                 .unwrap_or('.');
//             line.push(c);
//         }
//         println!("{}", line);
//     }
// }

fn print_wide_grid(
    grid: &[Object],
    robot: (usize, usize),
) {
    for row in 0..10 {
        let mut line = "".to_string();
        for col in 0..(10 * 2) {
            if robot == (col, row) {
                line.push('@');
                continue;
            }
            let c = grid
                .iter()
                .find(|obj| obj.x == col && obj.y == row)
                .map(|obj| match obj.kind {
                    ObjectKind::WideBox => "[",
                    ObjectKind::Obstacle => "#",
                    _ => "",
                    // ObjectKind::Box => unreachable!(),
                })
                .unwrap_or(".");
            line.push_str(c);
        }
        println!("{}", line.replace("[.", "[]"));
    }
}

pub fn main() {
    let contents = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    // let contents = include_str!("../input/day-15-input.txt");

    let mut robot = (0, 0); // x, y
    let mut grid = vec![];
    for (y, row) in contents.lines().filter(|l| l.contains('#')).enumerate() {
        for (x, c) in row.to_string().chars().enumerate() {
            match c {
                '.' => {}
                '@' => robot = (x, y),
                _ => grid.push(Object {
                    x,
                    y,
                    kind: match c {
                        'O' => ObjectKind::Box,
                        '#' => ObjectKind::Obstacle,
                        _ => unreachable!(),
                    },
                }),
            };
        }
    }

    for dir in contents
        .lines()
        .filter(|line| !line.contains('#'))
        .join("")
        .chars()
    {
        // continue;
        let cloned_grid = grid.clone();
        let path = cloned_grid
            .iter()
            .filter(|obj| match dir {
                '<' => obj.x < robot.0 && obj.y == robot.1,
                '>' => obj.x > robot.0 && obj.y == robot.1,
                '^' => obj.x == robot.0 && obj.y < robot.1,
                'v' => obj.x == robot.0 && obj.y > robot.1,
                _ => unreachable!(),
            })
            .collect::<Vec<&Object>>();

        let dist_to_obstacle = path
            .iter()
            .filter(|obj| matches!(obj.kind, ObjectKind::Obstacle))
            .map(|obj| match dir {
                '<' | '>' => obj.x.abs_diff(robot.0),
                '^' | 'v' => obj.y.abs_diff(robot.1),
                _ => unreachable!(),
            })
            .min()
            .unwrap();

        let dist_to_free = {
            let mut free = robot;
            while free.0 > 0 // not at edge
                && free.1 > 0
                && (free == robot
                    || path // tile is occupied
                        .iter()
                        .any(|obj| obj.x == free.0 && obj.y == free.1))
            {
                match dir {
                    '<' => free.0 -= 1,
                    '>' => free.0 += 1,
                    '^' => free.1 -= 1,
                    'v' => free.1 += 1,
                    _ => unreachable!(),
                };
            }

            match dir {
                '<' | '>' => free.0.abs_diff(robot.0),
                '^' | 'v' => free.1.abs_diff(robot.1),
                _ => unreachable!(),
            }
        };

        // println!("{:?}", i);
        // // println!("{:?} traversable {:?} boxes {:?}", dir, traversable, boxes);
        // println!("{:?}", dir);
        // println!("dist to first free space {:?}", dist_to_free);
        // println!("dist to first obst {:?}", dist_to_obstacle);
        // println!();

        let offset = match dir {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => unreachable!(),
        };

        let movable_boxes = path
            .iter()
            .filter(|obj| {
                matches!(obj.kind, ObjectKind::Box)
                    && match dir {
                        '<' | '>' => obj.x.abs_diff(robot.0),
                        '^' | 'v' => obj.y.abs_diff(robot.1),
                        _ => unreachable!(),
                    } < dist_to_free
            })
            .collect::<Vec<_>>();

        if dist_to_free >= dist_to_obstacle {
            // println!("cannot move");
        } else if dist_to_free == 1 {
            // println!("moving without pushing boxes");
            robot.0 = robot.0.checked_add_signed(offset.0).unwrap();
            robot.1 = robot.1.checked_add_signed(offset.1).unwrap();
        } else if movable_boxes.len() < dist_to_obstacle {
            // println!("moving and pushing boxes");

            for box_to_move in movable_boxes {
                grid.iter_mut()
                    .find(|b| b == box_to_move)
                    .unwrap()
                    ._move(offset);
            }

            robot.0 = robot.0.checked_add_signed(offset.0).unwrap();
            robot.1 = robot.1.checked_add_signed(offset.1).unwrap();
        } else {
            unreachable!()
        }
    }

    // print_grid(&grid, robot);

    let sum = grid
        .iter()
        .filter(|obj| matches!(obj.kind, ObjectKind::Box))
        .map(|_box| _box.x + _box.y * 100)
        .sum::<usize>();
    println!("{:?}", sum);

    let wide_contents = contents
        .replace('#', "##")
        .replace('O', "[]")
        .replace('.', "..")
        .replace('@', "@.");
    println!("{}", wide_contents);

    let mut robot = (0, 0); // x, y
    let mut wide_grid = vec![];
    for (y, row) in wide_contents
        .lines()
        .filter(|l| l.contains('#'))
        .enumerate()
    {
        for (x, c) in row.to_string().chars().enumerate() {
            match c {
                '.' | ']' => {}
                '@' => robot = (x, y),
                '[' => wide_grid.push(Object {
                    x,
                    y,
                    kind: ObjectKind::WideBox,
                }),
                '#' => wide_grid.push(Object {
                    x,
                    y,
                    kind: ObjectKind::Obstacle,
                }),
                _ => unreachable!(),
            };
        }
    }
    // println!("{:?}", wide_grid);

    let obstacle_positions = wide_grid
        .iter()
        .filter(|obj| matches!(obj.kind, ObjectKind::Obstacle))
        .map(|obj| (obj.x, obj.y))
        .collect::<HashSet<_>>();

    let mut i = 0;
    for dir in contents
        .lines()
        .filter(|line| !line.contains('#'))
        .join("")
        .chars()
    {
        i += 1;
        let cloned_grid = wide_grid.clone();

        let boxes = wide_grid
            .iter()
            .filter(|obj| matches!(obj.kind, ObjectKind::WideBox))
            .collect::<Vec<_>>();

        let real_box_positions = wide_grid
            .iter()
            .filter(|obj| matches!(obj.kind, ObjectKind::WideBox))
            .flat_map(|_box| [(_box.x, _box.y), (_box.x + 1, _box.y)])
            .collect::<Vec<_>>();

        let path = cloned_grid // now includes wide boxes
            .iter()
            .filter(|obj| match dir {
                '<' => obj.x < robot.0 && obj.y == robot.1,
                '>' => obj.x > robot.0 && obj.y == robot.1,
                // also include wide boxes
                '^' => (obj.x / 2).abs_diff((robot.0) / 2) == 0 && obj.y < robot.1,
                'v' => (obj.x / 2).abs_diff((robot.0) / 2) == 0 && obj.y > robot.1,
                _ => unreachable!(),
            })
            .collect::<Vec<&Object>>();

        let dist_to_obstacle = path // unchanged
            .iter()
            .filter(|obj| matches!(obj.kind, ObjectKind::Obstacle))
            .map(|obj| match dir {
                '<' | '>' => obj.x.abs_diff(robot.0),
                '^' | 'v' => obj.y.abs_diff(robot.1),
                _ => unreachable!(),
            })
            .min()
            .unwrap();

        let dist_to_free = {
            let mut free = robot;
            while (free.0 > 1 && free.1 > 0) // not at edge
                && (free == robot || real_box_positions.contains(&free))
            {
                match dir {
                    '<' => free.0 -= 1,
                    '>' => free.0 += 1,
                    '^' => free.1 -= 1,
                    'v' => free.1 += 1,
                    _ => unreachable!(),
                };
            }

            match dir {
                '<' | '>' => free.0.abs_diff(robot.0),
                '^' | 'v' => free.1.abs_diff(robot.1),
                _ => unreachable!(),
            }
        };

        if i >= 310 {
            println!("{:?}", i);
            println!("{:?}", robot);
            print_wide_grid(&wide_grid, robot);
            println!("next {} {:?}", dir, path);
            println!("dist to free {:?}", dist_to_free);
            println!("dist to obst {:?}", dist_to_obstacle);
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
        }

        let offset = match dir {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => unreachable!(),
        };

        // TODO: at 310, a box is erroneously excluded from this vec; use get_nei or
        // something
        let movable_boxes = path
            .iter()
            .filter(|obj| {
                matches!(obj.kind, ObjectKind::WideBox)
                    && match dir {
                        '<' | '>' => obj.x.abs_diff(robot.0),
                        '^' | 'v' => obj.y.abs_diff(robot.1),
                        _ => unreachable!(),
                    } < dist_to_free
            })
            .collect::<Vec<_>>();

        let get_nei = |pos: (usize, usize), is_robot: bool| -> Vec<&Object> {
            boxes
                .clone()
                .into_iter()
                .filter(|b| {
                    let x_ok = match is_robot {
                        //  @  @
                        // []  []
                        true => pos.0 == b.x || pos.0 - 1 == b.x,
                        //  [] [] []
                        // []  []  []
                        false => pos.0.abs_diff(b.x) <= 1,
                    };
                    match dir {
                        '^' => (x_ok) && pos.1 - 1 == b.y,
                        'v' => (x_ok) && pos.1 + 1 == b.y,
                        _ => unreachable!(),
                    }
                })
                .collect()
        };

        // check if vertical contiguous @B# path possible (which means cannot move)
        // horizontal paths require no special treatment
        let mut is_blocked: bool = false;
        if ['^', 'v'].contains(&dir) {
            let mut neighbour_boxes = get_nei(robot, true);
            println!("neighbours {:?}", neighbour_boxes);
            while !neighbour_boxes.is_empty() {
                neighbour_boxes = neighbour_boxes
                    .clone()
                    .into_iter()
                    .flat_map(|b| get_nei((b.x, b.y), false))
                    .collect();

                // obstacles that would block the current set of boxes
                let obstacles = neighbour_boxes
                    .iter()
                    .flat_map(|b| {
                        let ny = match dir {
                            '^' => b.y - 1,
                            'v' => b.y + 1,
                            _ => unreachable!(),
                        };
                        [(b.x, ny), (b.x + 1, ny)]
                    })
                    .collect();
                println!("neighbours {:?}", neighbour_boxes);
                // println!("{:?}", obstacle_positions);
                if obstacle_positions.intersection(&obstacles).count() > 0 {
                    is_blocked = true;
                    println!("blocked!");
                    // print_wide_grid(&wide_grid, robot);
                    // panic!()
                }
            }
            // println!("{:?}", is_blocked);
        }

        if dist_to_free >= dist_to_obstacle || is_blocked {
            // println!("cannot move");
        } else if dist_to_free == 1 {
            // println!("moving without pushing boxes");
            robot.0 = robot.0.checked_add_signed(offset.0).unwrap();
            robot.1 = robot.1.checked_add_signed(offset.1).unwrap();
        } else if movable_boxes.len() < dist_to_obstacle {
            // println!("moving and pushing boxes");

            for box_to_move in movable_boxes {
                wide_grid
                    .iter_mut()
                    .find(|b| b == box_to_move)
                    .unwrap()
                    ._move(offset);
            }

            robot.0 = robot.0.checked_add_signed(offset.0).unwrap();
            robot.1 = robot.1.checked_add_signed(offset.1).unwrap();
        } else {
            unreachable!()
        }
    }
}
