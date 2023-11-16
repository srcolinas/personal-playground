
pub fn sol0(parens: &str) -> bool {
    let mut num = 0;
    for ch in parens.chars() {
        match ch {
            '(' => num += 1,
            ')' => num -= 1,
            _ => return false,
        };
        if num < 0 {
            return false
        }
    };
    num == 0
}


pub fn sol1(parens: &str) -> bool {
    let mut parens = parens.to_string();
    while parens.matches("()").count() > 0 {
        parens = parens.replace("()", "");
    }
    parens.is_empty()
}

