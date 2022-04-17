/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if let Some(is_valid) = luhn(code) {
        is_valid
    } else {
        false
    }
}

fn luhn(code: &str) -> Option<bool> {
    let code_cleaned = remove_whitespace(code);

    if code_cleaned.len() < 2 {
        return None;
    }

    let mut sum = 0;
    println!("calculating luhn for: {:?}", code);
    for (i, n) in code_cleaned.chars().rev().enumerate() {
        let mut number = n.to_digit(10)?;

        if i % 2 != 0 {
            number *= 2;
            if number > 9 {
                number -= 9;
            }
        }
        sum += number;
        println!("number: {:?}, {:?}", code, number);
    }
    Some(sum % 10 == 0)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
