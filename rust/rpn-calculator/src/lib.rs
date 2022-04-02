#[derive(Debug, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn get_operands(operands: &mut Vec<i32>) -> Option<(i32, i32)> {
    if operands.len() < 2 {
        None
    } else {
        Some((operands.pop().unwrap(), operands.pop().unwrap()))
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut input_vec = inputs.to_vec();
    input_vec.reverse();
    let mut output = vec![];

    println!("{:?} {:?}", input_vec, input_vec.is_empty());
    while !input_vec.is_empty() {
        match input_vec.pop().unwrap() {
            CalculatorInput::Add => {
                let (a, b) = get_operands(&mut output)?;
                output.push(a + b);
            }
            CalculatorInput::Subtract => {
                let (a, b) = get_operands(&mut output)?;
                output.push(b - a);
            }
            CalculatorInput::Multiply => {
                let (a, b) = get_operands(&mut output)?;
                output.push(a * b);
            }
            CalculatorInput::Divide => {
                let (a, b) = get_operands(&mut output)?;
                output.push(b / a);
            }
            CalculatorInput::Value(val) => output.push(val),
        }
    }

    if output.len() != 1 {
        return None;
    }

    output.pop()
}
