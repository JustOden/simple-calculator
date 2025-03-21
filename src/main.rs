use std::{io, str::FromStr};

#[derive(Debug)]
struct InvalidOperator;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent
}

impl FromStr for Operation {
    type Err = InvalidOperator;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        Ok(match op {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            "^" => Self::Exponent,
            _ => return Err(InvalidOperator)
        })
    }
}

fn calculate(num1: f32, num2: f32, op: &Operation) -> f32 {
    match op {
        Operation::Add => num1 + num2,
        Operation::Subtract => num1 - num2,
        Operation::Multiply => num1 * num2,
        Operation::Divide => num1 / num2,
        Operation::Exponent => {
            if num1 < 0. {
                -(num1.powf(num2))
            } else {
                num1.powf(num2)
            }
        }
    }
}

fn debug(num1: f32, num2: f32, op: &Operation) {
    println!("{} {:?} {} = {}", num1, op, num2, calculate(num1, num2, op))
}

fn calculate_v2(mut nums: Vec<f32>, mut ops: Vec<Operation>) -> f32 {
    while ops.contains(&Operation::Exponent) {
        for i in 0..ops.len() {
            match ops[i] {
                Operation::Exponent => {
                    debug(nums[i], nums[i+1], &ops[i]);
                    nums[i] = calculate(nums[i], nums[i+1], &ops[i]);
                    ops.remove(i);
                    nums.remove(i+1);
                    break;
                },
                _ => ()
            }
        }
    }

    while ops.contains(&Operation::Multiply) | ops.contains(&Operation::Divide) {
        for i in 0..ops.len() {
            match ops[i] {
                Operation::Multiply | Operation::Divide => {
                    debug(nums[i], nums[i+1], &ops[i]);
                    nums[i] = calculate(nums[i], nums[i+1], &ops[i]);
                    ops.remove(i);
                    nums.remove(i+1);
                    break;
                },
                _ => ()
            }
        }
    }

    while ops.contains(&Operation::Add) | ops.contains(&Operation::Subtract) {
        for i in 0..ops.len() {
            match ops[i] {
                Operation::Add | Operation::Subtract => {
                    debug(nums[i], nums[i+1], &ops[i]);
                    nums[i] = calculate(nums[i], nums[i+1], &ops[i]);
                    ops.remove(i);
                    nums.remove(i+1);
                    break;
                },
                _ => ()
            }
        }
    }
    nums[0]
}

#[allow(dead_code)]
fn math1() {
    let mut num1: String = String::new();
    let mut num2: String = String::new();
    let mut op: String = String::new();

    println!("Enter num1");
    io::stdin().read_line(&mut num1).expect("Reading input failed.");
    println!("Enter op");
    io::stdin().read_line(&mut op).expect("Reading input failed.");
    println!("Enter num2");
    io::stdin().read_line(&mut num2).expect("Reading input failed.");

    let num1: f32 = num1.trim().parse().expect("expected number");
    let num2: f32 = num2.trim().parse().expect("expected number");
    let op: Result<Operation, InvalidOperator> = Operation::from_str(op.trim());
    
    match op {
        Ok(op) => println!("Answer is {}", calculate(num1, num2, &op)),
        Err(e) => println!("{:?}", e)
    }
}

#[allow(dead_code)]
fn math2() {
    let mut equation: String = String::new();

    println!("Enter equation...");
    io::stdin().read_line(&mut equation).expect("Reading input failed.");

    let mut nums = vec![];
    let mut ops = vec![];

    for s in equation.split(" ") {
        let op: Result<Operation, InvalidOperator> = Operation::from_str(s.trim());
        
        if let Ok(op) = op {
            ops.push(op);

        } else {
            let number: f32 = s.trim().parse().expect("Expected number");
            nums.push(number);
        }
    }
    println!("Answer is {}", calculate_v2(nums, ops))
}

fn main() {
    math1();
    math2();
}
