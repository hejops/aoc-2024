use std::fs;

use diagonal::diagonal_pos_neg;
use diagonal::diagonal_pos_pos;

pub fn main() {
    let contents = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let contents = include_str!("../input/day-4-input.txt").unwrap();

    // part 1

    let cols = contents.lines().next().unwrap().len();
    let rows = contents.lines().count();
    assert_eq!(rows, cols);
    // println!("{} {}", rows, cols);

    let horizontals = contents
        .lines()
        .map(|line| line.match_indices("XMAS").count() + line.match_indices("SAMX").count())
        .sum::<usize>();
    // println!("{:#?}", horizontals);

    let mut transposed = vec![vec!['\0'; rows]; cols];
    // with_capacity inits a vec with len 0!
    // unlike Go, Rust refuses to fill with 'default' char values
    // let mut transposed: Vec<Vec<char>> =
    // vec![Vec::<char>::with_capacity(rows); cols];
    contents.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, ch)| {
            transposed[x][y] = ch;
        })
    });
    let verticals = transposed
        .iter()
        .map(|x| x.iter().collect::<String>())
        .map(|line| line.match_indices("XMAS").count() + line.match_indices("SAMX").count())
        .sum::<usize>();
    // println!("{:?}", verticals);

    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let diagonals = diagonal_pos_neg(&grid)
        .into_iter()
        .chain(diagonal_pos_pos(&grid))
        .map(|x| x.into_iter().collect::<String>())
        .map(|line| line.match_indices("XMAS").count() + line.match_indices("SAMX").count())
        .sum::<usize>();

    println!("{:#?}", horizontals + verticals + diagonals);

    let flat_grid = contents.chars().filter(|c| *c != '\n').collect::<Vec<_>>();
    assert_eq!(flat_grid.len(), cols * rows);

    let w = -1;
    let e = 1;

    let se = <usize as TryInto<isize>>::try_into(rows + 1).unwrap();
    let s = <usize as TryInto<isize>>::try_into(rows).unwrap();
    let sw = <usize as TryInto<isize>>::try_into(rows - 1).unwrap();

    let nw = -se;
    let ne = -sw;
    let n = -s;

    let check_mas = |pos: usize, modifier: isize| {
        let left_ok = (pos % rows) >= 3; // 3 spaces free
        let up_ok = (pos / rows) >= 3;
        let right_ok = ((pos % rows) + 1 + 3) <= rows;

        if [w, nw, sw].contains(&modifier) && !left_ok {
            return false;
        }
        if [e, ne, se].contains(&modifier) && !right_ok {
            return false;
        }
        if [n, nw, ne].contains(&modifier) && !up_ok {
            return false;
        }

        let ipos: isize = pos.try_into().unwrap();

        // note:
        // .get(-n) -> panic
        // .get(out-of-bounds) -> None (safe)
        flat_grid.get((ipos + modifier).unsigned_abs()) == Some(&'M')
            && flat_grid.get((ipos + modifier * 2).unsigned_abs()) == Some(&'A')
            && flat_grid.get((ipos + modifier * 3).unsigned_abs()) == Some(&'S')
    };

    let xmas: usize = flat_grid
        .iter()
        // // filters more than necessary, possibly because the indices get changed?
        // .filter(|c| *c == &'X')
        .enumerate()
        .filter(|(_, c)| *c == &'X')
        .map(|(i, _)| {
            [nw, n, ne, e, se, s, sw, w]
                .iter()
                .filter(|dir| check_mas(i, **dir))
                .count()
        })
        .sum();
    println!("{:#?}", xmas);

    // part 2
    let x_mas = flat_grid
        .iter()
        // .filter(|c| *c == &'A')
        .enumerate()
        .filter(|(_, c)| *c == &'A')
        .filter(|(i, _)| {
            // 1 space free on all sides
            (1..rows).contains(&(i % rows))
                && (1..rows).contains(&(i / rows))
                && [
                    [Some(&'M'), Some(&'M'), Some(&'S'), Some(&'S')],
                    [Some(&'S'), Some(&'S'), Some(&'M'), Some(&'M')],
                    [Some(&'M'), Some(&'S'), Some(&'M'), Some(&'S')],
                    [Some(&'S'), Some(&'M'), Some(&'S'), Some(&'M')],
                ]
                .contains(&[
                    flat_grid.get(i - (rows + 1)),
                    flat_grid.get(i - (rows - 1)),
                    flat_grid.get(i + (rows - 1)),
                    flat_grid.get(i + (rows + 1)),
                ])
        })
        .count();
    println!("{:#?}", x_mas);
}
