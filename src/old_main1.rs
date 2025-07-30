// Naive tree-walking evaluator
use std::time::Instant;
use rand::Rng;

#[derive(Debug)]
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
}

fn eval_naive(expr: &Expr) -> i32 {
    match expr {
        Expr::Value(v) => *v,
        Expr::Add(lhs, rhs) => eval_naive(lhs) + eval_naive(rhs),
    }
}

// Register-based bytecode VM
#[derive(Debug)]
enum Instr {
    Load(i32, usize),       // Load(value, register_index)
    Add(usize, usize, usize), // Add(src1, src2, dest)
}

fn eval_register(bytecode: &[Instr], regs: &mut [i32; 10]) -> i32 {
    for instr in bytecode {
        match instr {
            Instr::Load(val, dst) => regs[*dst] = *val,
            Instr::Add(r1, r2, dst) => regs[*dst] = regs[*r1] + regs[*r2],
        }
    }
    regs[6] // final result
}

fn main() {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..=100);
    let b = rng.gen_range(1..=100);
    let c = rng.gen_range(1..=100);
    let d = rng.gen_range(1..=100);

    // Construct naive expression tree: ((a + b) + (c + d))
    let expr = Expr::Add(
        Box::new(Expr::Add(Box::new(Expr::Value(a)), Box::new(Expr::Value(b)))),
        Box::new(Expr::Add(Box::new(Expr::Value(c)), Box::new(Expr::Value(d)))),
    );

    // Construct equivalent bytecode
    let bytecode = vec![
        Instr::Load(a, 0),
        Instr::Load(b, 1),
        Instr::Add(0, 1, 4),
        Instr::Load(c, 2),
        Instr::Load(d, 3),
        Instr::Add(2, 3, 5),
        Instr::Add(4, 5, 6),
    ];

    let rounds = 100_000;

    let start_naive = Instant::now();
    for _ in 0..rounds {
        eval_naive(&expr);
    }
    let duration_naive = start_naive.elapsed();

    let mut regs = [0; 10];
    let start_register = Instant::now();
    for _ in 0..rounds {
        eval_register(&bytecode, &mut regs);
    }
    let duration_register = start_register.elapsed();

    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
    println!("Naive tree-walk time: {:?}", duration_naive);
    println!("Register-based VM time: {:?}", duration_register);
}
