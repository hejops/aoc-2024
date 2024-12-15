use std::fmt::Debug;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Object {
    x: usize,
    y: usize,
    is_box: bool,
}

impl Object {
    fn _move(
        &mut self,
        direction: (isize, isize),
    ) {
        if self.is_box {
            let (dx, dy) = direction;
            self.x = self.x.checked_add_signed(dx).unwrap();
            self.y = self.y.checked_add_signed(dy).unwrap();
        }
    }
}

fn print_grid(
    grid: &[Object],
    robot: (usize, usize),
) {
    for row in 0..10 {
        let mut line = "".to_string();
        for col in 0..10 {
            if robot == (col, row) {
                line.push('@');
                continue;
            }
            let c = grid
                .iter()
                .find(|obj| obj.x == col && obj.y == row)
                .map(|obj| if obj.is_box { 'O' } else { '#' })
                .unwrap_or('.');
            line.push(c);
        }
        println!("{}", line);
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

    let contents = include_str!("../input/day-15-input.txt");

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
                    is_box: c == 'O',
                }),
            };
        }
    }

    for dir in contents
        .lines()
        .filter(|line| !line.contains('#'))
        .map(|l| l.to_string())
        .join("")
        .chars()
    {
        let cloned_grid = grid.clone();
        let traversable = cloned_grid
            .iter()
            .filter(|obj| match dir {
                '<' => obj.x < robot.0 && obj.y == robot.1,
                '>' => obj.x > robot.0 && obj.y == robot.1,
                '^' => obj.x == robot.0 && obj.y < robot.1,
                'v' => obj.x == robot.0 && obj.y > robot.1,
                _ => unreachable!(),
            })
            .collect::<Vec<&Object>>();

        let dist_to_obstacle = traversable
            .iter()
            .filter(|obj| !obj.is_box)
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
                    || traversable // tile is occupied
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

        let movable_boxes = traversable
            .iter()
            .filter(|obj| {
                obj.is_box
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

            // https://old.reddit.com/r/rust/comments/15b95px/_/jtp76vm/
            for idx in 0..grid.len() {
                let box_dist = grid[idx].x.abs_diff(robot.0) + grid[idx].y.abs_diff(robot.1);
                if (box_dist < dist_to_free) && movable_boxes.contains(&&&grid[idx]) {
                    grid[idx]._move(offset);
                }
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
        .filter(|obj| obj.is_box)
        .map(|_box| _box.x + _box.y * 100)
        .sum::<usize>();
    println!("{:?}", sum);
}
