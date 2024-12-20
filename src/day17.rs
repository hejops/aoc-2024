pub fn main() {
    let contents = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    let contents = include_str!("../input/day-17-input.txt");
    println!("{}", contents);

    let mut regs = contents.lines().take(3).map(|line| {
        line.split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    });
    let mut a = regs.next().unwrap();
    let mut b = regs.next().unwrap();
    let mut c = regs.next().unwrap();

    let program = contents
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // part 1
    let mut i_ptr = 0;
    let mut outputs = vec![];
    while i_ptr + 1 < program.len() {
        let inst = program[i_ptr];
        let literal_op = program[i_ptr + 1];

        let combo_op = match literal_op {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => a,
            5 => b,
            6 => c,
            7 => 999, // this value should never be used
            _ => unreachable!(),
        };

        match inst {
            0 => a /= 2_usize.pow(combo_op as u32),
            1 => b ^= literal_op,
            2 => b = combo_op % 8,
            3 => {
                if a != 0 {
                    i_ptr = literal_op;
                    continue;
                }
            }
            4 => b ^= c,
            5 => outputs.push(combo_op % 8),
            6 => b = a / 2_usize.pow(combo_op as u32),
            7 => c = a / 2_usize.pow(combo_op as u32),
            _ => unreachable!(),
        }
        i_ptr += 2;
    }

    let output = outputs
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", output);
}
