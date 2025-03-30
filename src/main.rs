use std::{io, str::FromStr};

#[derive(Debug)]
struct InvalidOperator;

#[derive(Debug)]
struct InvalidToken;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

impl FromStr for Operator {
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

#[derive(Debug, PartialEq)]
enum Token {
    Operation(Operator),
    Number(f32),
    DecPoint,
    OpenParen,
    CloseParen,
    ParenExpr(String)
}

impl FromStr for Token {
    type Err = InvalidToken;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        if let Ok(op) = Operator::from_str(token) {
            Ok(Self::Operation(op))

        } else if let Ok(num) = token.parse::<f32>() {
            Ok(Self::Number(num))

        } else {
            Ok(match token {
                "(" => Self::OpenParen,
                ")" => Self::CloseParen,
                "." => Self::DecPoint,
                _ => return Err(InvalidToken)
            })
        }
    }
}

fn tokenize(s: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut num_index: usize = 0;
    let mut num_count: usize = 0;

    let mut open_paren_index: usize = 0;
    let mut open_paren_count: i32 = 0;

    let mut close_paren_count: i32 = 0;
    let mut opened: bool = false;

    for (i, c) in s.chars().enumerate() {
        if let Ok(token) = Token::from_str(&c.to_string()) {
            match token {
                Token::Number(_) | Token::DecPoint => {
                    if !opened {
                        num_index = i;
                        num_count += 1;
                    }
                },
                Token::Operation(op) => {
                    if !opened {
                        let no_prev: bool = i == 0;
                        if no_prev || Operator::from_str(s.get(i-1..i).unwrap()).is_ok() {
                            if op == Operator::Subtract {
                                num_index = i;
                                num_count += 1;
                            }
                        } else {
                            tokens.push(token)
                        }
                    }
                },
                Token::OpenParen => {
                    open_paren_count += 1;
                    if !opened {
                        open_paren_index = i;
                        opened = true;
                    }
                },
                Token::CloseParen => {
                    close_paren_count += 1;
                    if open_paren_count == close_paren_count {
                        tokens.push(Token::ParenExpr(s.get(open_paren_index+1..i).unwrap().to_string()));
                        opened = false;
                    }
                },
                _ => ()
            }

            let next_str: Option<&str> = s.get(i..=i+1);

            if next_str.is_none() || next_str.is_some() && next_str.unwrap().parse::<f32>().is_err() {
                if num_count > 0 {
                    tokens.push(Token::from_str(s.get(num_index-(num_count-1)..=num_index).unwrap()).unwrap());
                    num_count = 0;
                }
            }
        }
    }
    tokens
}

fn calculate(num1: f32, num2: f32, op: Operator) -> f32 {
    let ans: f32 = match op {
        Operator::Add => num1 + num2,
        Operator::Subtract => num1 - num2,
        Operator::Multiply => num1 * num2,
        Operator::Divide => num1 / num2,
        Operator::Exponent => {
            if num1 < 0. {
                -(num1.powf(num2))
            } else {
                num1.powf(num2)
            }
        }
    };
    debug(num1, num2, op, ans);
    ans
}

fn calculate_v2(tokens: Vec<Token>) -> f32 {
    let mut nums: Vec<f32> = vec![];
    let mut ops: Vec<Operator> = vec![];

    for token in tokens {
        match token {
            Token::Number(num) => nums.push(num),
            Token::Operation(op) => ops.push(op),
            Token::ParenExpr(s) => {
                if ops[ops.len()-1] == Operator::Subtract && ops.len()+1 >= nums.len() {
                    nums.push(-calculate_v2(tokenize(s)));
                    ops.remove(ops.len()-1);
                } else {
                    nums.push(calculate_v2(tokenize(s)));
                }
            },
            _ => ()
        }
    }

    while ops.contains(&Operator::Exponent) {
        for i in 0..ops.len() {
            match ops[i] {
                Operator::Exponent => {
                    nums[i] = calculate(nums[i], nums[i+1], ops[i]);
                    ops.remove(i);
                    nums.remove(i+1);
                    break;
                },
                _ => ()
            }
        }
    }

    while ops.contains(&Operator::Multiply) | ops.contains(&Operator::Divide) {
        for i in 0..ops.len() {
            match ops[i] {
                Operator::Multiply | Operator::Divide => {
                    nums[i] = calculate(nums[i], nums[i+1], ops[i]);
                    ops.remove(i);
                    nums.remove(i+1);
                    break;
                },
                _ => ()
            }
        }
    }

    while ops.contains(&Operator::Add) | ops.contains(&Operator::Subtract) {
        for i in 0..ops.len() {
            match ops[i] {
                Operator::Add | Operator::Subtract => {
                    nums[i] = calculate(nums[i], nums[i+1], ops[i]);
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

fn debug(num1: f32, num2: f32, op: Operator, ans: f32) {
    println!("{} {:?} {} = {}", num1, op, num2, ans);
}

fn main() {
    let mut equation: String = String::new();

    println!("Enter equation...");
    io::stdin().read_line(&mut equation).expect("Reading input failed.");
    equation.retain(|c: char| !c.is_whitespace());

    let tokens: Vec<Token> = tokenize(equation);
    println!("Answer = {}", calculate_v2(tokens));
}
