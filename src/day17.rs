pub fn main() {
    let contents = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    let contents = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    let contents = include_str!("../input/day-17-input.txt");
    println!("{}", contents);
    // return;

    let mut regs = contents.lines().take(3).map(|line| {
        line.split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    });
    let real_a = regs.next().unwrap();
    let real_b = regs.next().unwrap();
    let real_c = regs.next().unwrap();

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

    let run_program = |program: &[usize], mut a: usize, mut b: usize, mut c: usize| {
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

        outputs
    };

    // part 1
    let output = run_program(&program, real_a, real_b, real_c)
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", output);

    // part 2
    // 3 secs is not too shabby for a stupid brute force solution tbqh

    fn last_n_matching<T>(
        v1: &[T],
        v2: &[T],
    ) -> usize
    where
        T: PartialEq,
    {
        assert_eq!(v1.len(), v2.len());
        let mut n = 0;
        while v1[v1.len() - 1 - n] == v2[v2.len() - 1 - n] {
            n += 1;
        }
        n
    }

    let mut new_a = real_a;
    let l = &program.len();
    loop {
        let output = run_program(&program, new_a, real_b, real_c);
        match 1 {
            _a if output == program => break,
            _a if output.len() > *l => panic!(),
            _a if output.len() < *l - 1 => {
                new_a *= 10001;
                new_a /= 10000;
            }
            _a if output.len() < *l => {
                new_a *= 100001;
                new_a /= 100000;
            }
            _ => {
                // arrives at a solution fairly quickly, but it will be too high
                // i haven't found a way to reach the goal in one pass
                let last = last_n_matching(&output, &program);
                new_a += 3_usize.pow(17_u32.saturating_sub(last as u32));
            }
        }
    }
    println!("first pass (high): {:?}", new_a);

    new_a -= 3_usize.pow(17);
    loop {
        let output = run_program(&program, new_a, real_b, real_c);
        if output == program {
            break;
        }
        let last = last_n_matching(&output, &program);
        // guarantee we increment by 1 when we are close to the goal
        new_a += 2_usize.pow(14_u32.saturating_sub(last as u32));
    }
    println!("{:?}", new_a);

    // https://github.com/onlycs/advent24/blob/main/solutions/src/day17.rs#L99
    let mut candidates = vec![0];
    // if some digit in the range [X, X+8] is used as input to program P, and P(A=X)
    // evaluates to an output O whose first digit O[0] is equal to P[n], then
    // digit X is marked as a successful candidate, multiplied by 8, and
    // propagated to the next iteration, until P[0] is reached.
    //
    // each run of the program must go through three loops:
    // 1. each (octal) digit in the reversed program (seed: 0)
    // 2. each successful candidate from the previous iteration (multiplied by 8)
    // 3. octal increment
    for val in program.iter().rev() {
        let mut ok = vec![];
        for c in &candidates {
            for k in 0..8 {
                let cur = (c * 8) + k;
                if run_program(&program, cur, real_b, real_c)[0] == *val {
                    ok.push(cur);
                }
            }
        }
        // println!("{:?}", ok);
        candidates = ok;
    }
    println!("{:?}", candidates.iter().min().unwrap());
}
