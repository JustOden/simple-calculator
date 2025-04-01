use std::{io, io::Write, str::FromStr};

const DEBUG_MODE: bool = false;

fn main() {
    println!("Enter an equation (ex. '5^(6/3)') or type 'exit' to quit: ");
    loop {
        let mut equation: String = String::new();

        print!(">> ");
        let _=io::stdout().flush();

        io::stdin().read_line(&mut equation).expect("Reading input failed.");
        equation.retain(|c: char| !c.is_whitespace());

        if equation.to_lowercase() == "exit" {break}

        let tokens: Vec<Token> = tokenize(equation);

        if DEBUG_MODE {println!("Tokens: {:?}", tokens)}

        match calculate_v2(tokens) {
            Some(n) => println!("Answer: {}", n),
            None => println!("Invalid syntax")
        }
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidOperator,
    InvalidToken
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

impl FromStr for Operator {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" | "x" => Self::Multiply,
            "/" => Self::Divide,
            "^" => Self::Exponent,
            _ => return Err(ParseError::InvalidOperator)
        })
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Operation(Operator),
    Number(f64),
    DecPoint,
    OpenParen,
    CloseParen,
    ParenExpr(Vec<Token>),
    NegParenExpr(Vec<Token>)
}

impl FromStr for Token {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(op) = Operator::from_str(s) {
            Ok(Self::Operation(op))

        } else if let Ok(num) = s.parse::<f64>() {
            Ok(Self::Number(num))

        } else {
            Ok(match s {
                "(" => Self::OpenParen,
                ")" => Self::CloseParen,
                "." => Self::DecPoint,
                _ => return Err(ParseError::InvalidToken)
            })
        }
    }
}

fn tokenize(s: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut str_num: String = String::new();

    let mut open_paren_index: usize = 0;
    let mut open_paren_count: i32 = 0;

    let mut close_paren_count: i32 = 0;
    let mut paren_opened: bool = false;

    let mut neg_paren: bool = false;

    for (i, c) in s.chars().enumerate() {
        if let Ok(token) = Token::from_str(&c.to_string()) {
            match token {
                Token::Number(_) | Token::DecPoint => {
                    if !paren_opened {
                        str_num.push(c);

                    } else {
                        continue;
                    }
                },
                Token::Operation(op) => {
                    if !paren_opened {

                        let unary_minus: bool = {
                            tokens.len() > 1 &&
                            Operator::from_str(s.get(i-1..i).unwrap()).is_ok() &&
                            op == Operator::Subtract ||
                            tokens.is_empty() &&
                            op == Operator::Subtract
                        };
                        
                        if unary_minus {
                            str_num.push(c);

                        } else {
                            tokens.push(token);
                        }

                    } else {
                        continue;
                    }
                },
                Token::OpenParen => {
                    open_paren_count += 1;
                    if !paren_opened {

                        let unary_minus: bool = {
                            tokens.len() > 1 &&
                            Operator::from_str(s.get(i-2..i-1).unwrap()).is_ok() &&
                            tokens.last().is_some_and(|t| *t == Token::Operation(Operator::Subtract)) ||
                            tokens.len() == 1 &&
                            tokens.last().is_some_and(|t| *t == Token::Operation(Operator::Subtract))
                        };

                        if unary_minus {
                            neg_paren = true;
                            tokens.remove(tokens.len()-1);

                        } else {
                            neg_paren = false;
                        }

                        open_paren_index = i;
                        paren_opened = true;

                    } else {
                        continue;
                    }
                },
                Token::CloseParen => {
                    close_paren_count += 1;
                    if open_paren_count == close_paren_count {
                        if neg_paren {
                            tokens.push(Token::NegParenExpr(tokenize(s.get(open_paren_index+1..i).unwrap().to_string())));

                        } else {
                            tokens.push(Token::ParenExpr(tokenize(s.get(open_paren_index+1..i).unwrap().to_string())));
                        }

                        paren_opened = false;
                    }
                },
                _ => ()
            }
            let next_str: Option<&str> = s.get(i..=i+1);

            if next_str.is_none() || next_str.unwrap().parse::<f64>().is_err() {
                if str_num.len() > 0 {
                    tokens.push(Token::from_str(&str_num).unwrap());
                    str_num = String::new();
                }
            }
        }
    }
    tokens
}

fn calculate(num1: f64, num2: f64, op: Operator) -> f64 {
    let ans: f64 = match op {
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
    if DEBUG_MODE {println!("{} {:?} {} = {}", num1, op, num2, ans)};
    ans
}

fn calculate_v2(tokens: Vec<Token>) -> Option<f64> {
    let mut nums: Vec<f64> = vec![];
    let mut ops: Vec<Operator> = vec![];

    for token in tokens {
        match token {
            Token::Number(num) => nums.push(num),
            Token::Operation(op) => ops.push(op),
            Token::ParenExpr(t) => nums.push(
                match calculate_v2(t) {
                    Some(n) => n,
                    None => return None,
                }
            ),
            Token::NegParenExpr(t) => nums.push(
                match calculate_v2(t) {
                    Some(n) => -n,
                    None => return None,
                }
            ),
            _ => ()
        }
    }

    if ops.len() >= nums.len() {
        return None;
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

    if nums.is_empty() {
        None
    } else {
        Some(nums[0])
    }
}
