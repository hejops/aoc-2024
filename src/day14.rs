use std::collections::HashSet;

use regex::Regex;

pub fn main() {
    let contents = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    let cols = 11;
    let rows = 7;

    let contents = include_str!("../input/day-14-input.txt");
    let cols = 101;
    let rows = 103;

    // part 1
    let reg = Regex::new(r"(-?\d+)").unwrap();
    let robots = contents
        .lines()
        .map(|line| {
            let nums = reg
                .captures_iter(line)
                .map(|c| c.extract::<1>().1[0].parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            let r_x = nums[0];
            let r_y = nums[1];
            let d_x = nums[2];
            let d_y = nums[3];

            let mut end_x = (r_x + 100 * d_x) % cols;
            let mut end_y = (r_y + 100 * d_y) % rows;
            if end_x < 0 {
                end_x += cols
            }
            if end_y < 0 {
                end_y += rows
            }

            (end_x, end_y)
        })
        .collect::<Vec<_>>();
    // println!("{:#?}", robots);

    let mid_x = cols / 2;
    let mid_y = rows / 2;
    let product = robots
        .iter()
        .fold([0; 4], |mut quadrants, (end_x, end_y)| {
            match 1 {
                _a if *end_x == mid_x => {}
                _a if *end_y == mid_y => {}
                _a if *end_x < mid_x && *end_y < mid_y => quadrants[0] += 1,
                _a if *end_x < mid_x && *end_y > mid_y => quadrants[1] += 1,
                _a if *end_x > mid_x && *end_y < mid_y => quadrants[2] += 1,
                _a if *end_x > mid_x && *end_y > mid_y => quadrants[3] += 1,
                _ => unreachable!(),
            };
            quadrants
        })
        .iter()
        .product::<usize>();
    println!("{:#?}", product);

    struct Robot {
        x: isize,
        y: isize,
        dx: isize,
        dy: isize,
    }

    impl Robot {
        fn _move(
            &mut self,
            rows: &usize,
            cols: &usize,
        ) {
            self.x += self.dx;
            self.x %= *cols as isize;
            if self.x < 0 {
                self.x += *cols as isize
            }

            self.y += self.dy;
            self.y %= *rows as isize;
            if self.y < 0 {
                self.y += *rows as isize
            }
        }
    }

    // part 2
    // https://old.reddit.com/r/adventofcode/comments/1hdvhvu/2024_day_14_solutions/m1ztgnx/
    // checking for seconds with all unique robot positions seems cheap;
    // at some point i might implement triangle checking:
    //   X
    //  XXX
    // XXXXX

    let mut robots = contents
        .lines()
        .map(|line| {
            let nums = reg
                .captures_iter(line)
                .map(|c| c.extract::<1>().1[0].parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            let x = nums[0];
            let y = nums[1];
            let dx = nums[2];
            let dy = nums[3];

            Robot { x, y, dx, dy }
        })
        .collect::<Vec<_>>();

    let len = robots.len();
    let mut uniq: HashSet<(isize, isize)> =
        HashSet::from_iter(robots.iter().map(|r| (r.x, r.y)).collect::<Vec<_>>());
    let mut seconds = 0;

    while uniq.len() != len {
        robots
            .iter_mut()
            .for_each(|r| r._move(&(rows as usize), &(cols as usize)));
        uniq = HashSet::from_iter(robots.iter().map(|r| (r.x, r.y)).collect::<Vec<_>>());
        seconds += 1;
    }
    println!("{:?}", seconds);
}
