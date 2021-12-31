#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use CalculatorInput::{Add, Divide, Multiply, Subtract, Value};

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();

    for input in inputs.iter() {
        match input {
            Value(v) => stack.push(*v),
            op @ _ => match stack.pop() {
                Some(x) => match stack.pop() {
                    Some(y) => match op {
                        Add => stack.push(x + y),
                        Subtract => stack.push(y - x),
                        Multiply => stack.push(x * y),
                        Divide => stack.push(y / x),
                        Value(_) => (),
                    },
                    None => return None,
                },
                None => return None,
            },
        }
    }

    if stack.len() != 1 {
        return None;
    }

    stack.pop()
}
