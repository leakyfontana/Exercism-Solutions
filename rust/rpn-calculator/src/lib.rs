#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;

    if inputs.len() == 0 {
        return None
    }

    let mut stack: Vec<i32> = Vec::with_capacity(inputs.len());
    let mut next: i32;
    let mut first: i32;

    for item in inputs.iter() {
        match item {
            Value(num) => stack.push(*num),
            Add => {
                first = stack.pop()?;
                next = stack.pop()?;
                stack.push(next + first);
            },
            Subtract => {
                first = stack.pop()?;
                next = stack.pop()?;
                stack.push(next - first);
            },
            Multiply => {
                first = stack.pop()?;
                next = stack.pop()?;
                stack.push(next * first);
            },
            Divide => {
                first = stack.pop()?;
                next = stack.pop()?;
                stack.push(next / first);
            },
        }
    }

    if stack.len() > 1 {
        return None
    }
    else {      
        Some(stack[0])
    }
}
