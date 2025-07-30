// Extended VM with basic control flow and arithmetic
use std::time::Instant;
use rand::Rng;

#[derive(Debug)]
enum Instr {
    Load(i32, usize),                 // Load(value, dst)
    Add(usize, usize, usize),         // Add(r1, r2, dst)
    Sub(usize, usize, usize),         // Sub(r1, r2, dst)
    Mul(usize, usize, usize),         // Mul(r1, r2, dst)
    Gt(usize, usize, usize),          // Gt(r1, r2, dst) → dst = 1 if r1 > r2 else 0
    JmpIfFalse(usize, usize),         // If regs[cond] == 0 → pc = target
    Jmp(usize),                       // Unconditional jump
}

fn eval_program(bytecode: &[Instr], regs: &mut [i32; 10]) -> i32 {
    let mut pc = 0;
    while pc < bytecode.len() {
        // println!("{:?}",regs);
        match &bytecode[pc] {
            Instr::Load(val, dst) => regs[*dst] = *val,
            Instr::Add(r1, r2, dst) => regs[*dst] = regs[*r1] + regs[*r2],
            Instr::Sub(r1, r2, dst) => regs[*dst] = regs[*r1] - regs[*r2],
            Instr::Mul(r1, r2, dst) => regs[*dst] = regs[*r1] * regs[*r2],
            Instr::Gt(r1, r2, dst) => regs[*dst] = if regs[*r1] > regs[*r2] { 1 } else { 0 },
            Instr::JmpIfFalse(cond, target) => {
                if regs[*cond] == 0 {
                    pc = *target;
                    continue;
                }
            },
            Instr::Jmp(target) => {
                pc = *target;
                continue;
            }
        }
        pc += 1;
    }
    regs[9] // final result stored in r9
}

#[derive(Debug)]
enum Expr {
    Value(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

fn eval_naive_expr(expr: &Expr) -> i32 {
    match expr {
        Expr::Value(v) => *v,
        Expr::Add(lhs, rhs) => eval_naive_expr(lhs) + eval_naive_expr(rhs),
        Expr::Sub(lhs, rhs) => eval_naive_expr(lhs) - eval_naive_expr(rhs),
        Expr::Mul(lhs, rhs) => eval_naive_expr(lhs) * eval_naive_expr(rhs),
        Expr::Gt(lhs, rhs) => {
            if eval_naive_expr(lhs) > eval_naive_expr(rhs) { 1 } else { 0 }
        }
        Expr::If(cond, then_expr, else_expr) => {
            if eval_naive_expr(cond) != 0 {
                eval_naive_expr(then_expr)
            } else {
                eval_naive_expr(else_expr)
            }
        }
    }
}

fn eval_direct(a: i32, b: i32) -> i32 {
    let c = a + b;
    if c > 10 {
        c * 2
    } else {
        c - 1
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..=10);
    let b = rng.gen_range(1..=10);

    let bytecode = vec![
        Instr::Load(a, 0),               // r0 = a
        Instr::Load(b, 1),               // r1 = b
        Instr::Add(0, 1, 2),             // r2 = a + b (c)
        Instr::Load(10, 3),              // r3 = 10
        Instr::Gt(2, 3, 4),              // r4 = c > 10 ? 1 : 0
        Instr::JmpIfFalse(4, 9),        // if not r4, jump to subtraction
        Instr::Load(2, 7),               // r7 = 2
        Instr::Mul(2, 7, 9),             // r9 = c * 2 (result)
        Instr::Jmp(12),                  // jump to end
        Instr::Load(1, 6),               // r6 = 1
        Instr::Sub(2, 6, 9),             // r9 = c - 1 (result)
    ];

    let expr = Expr::If(
        Box::new(Expr::Gt(
            Box::new(Expr::Add(Box::new(Expr::Value(a)), Box::new(Expr::Value(b)))),
            Box::new(Expr::Value(10)),
        )),
        Box::new(Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::Value(a)), Box::new(Expr::Value(b)))),
            Box::new(Expr::Value(2)),
        )),
        Box::new(Expr::Sub(
            Box::new(Expr::Add(Box::new(Expr::Value(a)), Box::new(Expr::Value(b)))),
            Box::new(Expr::Value(1)),
        )),
    );

    let rounds = 100_000;
    // let rounds = 1;
    let mut regs = [0; 10];

    let start_vm = Instant::now();
    for _ in 0..rounds {
        eval_program(&bytecode, &mut regs);
    }
    let duration_vm = start_vm.elapsed();

    let start_naive = Instant::now();
    let mut naive_result = 0;
    for _ in 0..rounds {
        naive_result = eval_naive_expr(&expr);
    }
    let duration_naive = start_naive.elapsed();

    let start_direct = Instant::now();
    let mut direct_result = 0;
    for _ in 0..rounds {
        direct_result = eval_direct(a, b);
    }
    let duration_direct = start_direct.elapsed();

    println!("a = {}, b = {}", a, b);
    println!("Result (VM)     = {}", regs[9]);
    println!("Result (Naive)  = {}", naive_result);
    println!("Result (Direct) = {}", direct_result);
    println!("VM time:     {:?}", duration_vm);
    println!("Naive time:  {:?}", duration_naive);
    println!("Direct time: {:?}", duration_direct);
}
