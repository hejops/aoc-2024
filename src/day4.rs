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

    let contents = fs::read_to_string("day-4-input.txt").unwrap();

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
        .map(|x| x.into_iter().collect::<String>())
        .map(|line| line.match_indices("XMAS").count() + line.match_indices("SAMX").count())
        .sum::<usize>()
        + diagonal_pos_pos(&grid)
            .into_iter()
            .map(|x| x.into_iter().collect::<String>())
            .map(|line| line.match_indices("XMAS").count() + line.match_indices("SAMX").count())
            .sum::<usize>();

    println!("{:#?}", horizontals + verticals + diagonals);

    let flat_grid = contents.chars().filter(|c| *c != '\n').collect::<Vec<_>>();
    assert_eq!(flat_grid.len(), cols * rows);

    let mut brute = 0;
    for (i, _) in flat_grid.iter().enumerate().filter(|(_, c)| *c == &'X') {
        // note:
        // .get(-n) -> panic
        // .get(out-of-bounds) -> None (safe)

        let no_left_wrap = (i % rows) >= 3; // 3 spaces free
        let no_up_wrap = (i / rows) >= 3;
        let no_right_wrap = ((i % rows) + 1 + 3) <= rows;
        // let no_down_wrap = ((i / rows) + 1 + 3) <= rows;

        // NW
        if no_left_wrap
            && no_up_wrap
            && flat_grid.get(i - (rows + 1)) == Some(&'M')
            && flat_grid.get(i - (rows + 1) * 2) == Some(&'A')
            && flat_grid.get(i - (rows + 1) * 3) == Some(&'S')
        {
            brute += 1
        };

        // N
        if no_up_wrap
            && flat_grid.get(i - rows) == Some(&'M')
            && flat_grid.get(i - (rows * 2)) == Some(&'A')
            && flat_grid.get(i - (rows * 3)) == Some(&'S')
        {
            brute += 1
        };

        // NE
        if no_right_wrap
            && no_up_wrap
            && flat_grid.get(i - (rows - 1)) == Some(&'M')
            && flat_grid.get(i - (rows - 1) * 2) == Some(&'A')
            && flat_grid.get(i - (rows - 1) * 3) == Some(&'S')
        {
            brute += 1
        };

        // E
        if no_right_wrap
            && flat_grid.get(i + 1) == Some(&'M')
            && flat_grid.get(i + 2) == Some(&'A')
            && flat_grid.get(i + 3) == Some(&'S')
        {
            brute += 1
        };

        // SE
        if no_right_wrap
            && flat_grid.get(i + (rows + 1)) == Some(&'M')
            && flat_grid.get(i + (rows + 1) * 2) == Some(&'A')
            && flat_grid.get(i + (rows + 1) * 3) == Some(&'S')
        {
            brute += 1
        };

        // S
        if flat_grid.get(i + rows) == Some(&'M')
            && flat_grid.get(i + (rows * 2)) == Some(&'A')
            && flat_grid.get(i + (rows * 3)) == Some(&'S')
        {
            brute += 1
        };

        // SW
        if no_left_wrap
            && flat_grid.get(i + (rows - 1)) == Some(&'M')
            && flat_grid.get(i + ((rows - 1) * 2)) == Some(&'A')
            && flat_grid.get(i + ((rows - 1) * 3)) == Some(&'S')
        {
            brute += 1
        };

        // W
        if no_left_wrap
            && flat_grid.get(i - 1) == Some(&'M')
            && flat_grid.get(i - 2) == Some(&'A')
            && flat_grid.get(i - 3) == Some(&'S')
        {
            brute += 1
        };
    }
    println!("{:#?}", brute);

    // part 2
    let brute = flat_grid
        .iter()
        .enumerate()
        .filter(|(i, c)| {
            // 1 space free on all sides
            *c == &'A'
                && (1..rows).contains(&(i % rows))
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
    println!("{:#?}", brute);
}
