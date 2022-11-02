use std::collections::HashMap;

fn main() {}

fn postfix_calculate(equation: &str) -> f64 {
    let mut stack: Vec<f64> = Vec::new();
    let tokens = equation.split_whitespace();
    for token in tokens {
        match token {
            "+" | "-" | "*" | "/" | "^" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let result = match token {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    "^" => left.powf(right),
                    _ => panic!("Invalid operator"),
                };
                stack.push(result);
            }
            _ => {
                let num = token.parse::<f64>().unwrap();
                stack.push(num);
            }
        }
    }
    return stack.pop().unwrap();
}

fn infix_to_postfix(equation: &str) -> String {
    let tokens = equation.split_ascii_whitespace();
    let mut results: Vec<&str> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();

    let precedence: HashMap<&str, i8> =
        [("(", 0), ("+", 1), ("-", 1), ("*", 2), ("/", 2), ("^", 3)]
            .iter()
            .cloned()
            .collect();
    let is_left_associative: HashMap<&str, bool> = [
        ("+", true),
        ("-", true),
        ("*", true),
        ("/", true),
        ("^", false),
    ]
    .iter()
    .cloned()
    .collect();
    for token in tokens {
        match token {
            "+" | "-" | "*" | "/" | "^" => {
                //operator
                let to_be_inserted_precedence = precedence.get(token).unwrap();
                while !operators.is_empty() {
                    let top_precedence = precedence.get(operators.last().unwrap()).unwrap();
                    if top_precedence > to_be_inserted_precedence {
                        results.push(operators.pop().unwrap());
                    } else if top_precedence == to_be_inserted_precedence {
                        if *is_left_associative.get(token).unwrap() {
                            results.push(operators.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            "(" => {
                operators.push(token);
            }
            ")" => {
                while operators.last().unwrap() != &"(" {
                    results.push(operators.pop().unwrap());
                }
                operators.pop();
            }
            _ => {
                //operand
                results.push(token)
            }
        }
    }
    while !operators.is_empty() {
        results.push(operators.pop().unwrap());
    }
    return results.join(" ");
}

fn calculate(equation: &str) -> f64 {
    let postfix = infix_to_postfix(equation);
    dbg!(postfix.clone());
    return postfix_calculate(&postfix);
}

#[cfg(test)]
mod tests {
    #[test]
    fn postfix_calculate() {
        assert_eq!(super::postfix_calculate("1 2 + 3 *"), 9.0);
        assert_eq!(super::postfix_calculate("1 2 + 3 * 4 +"), 13.0);
        assert_eq!(
            super::postfix_calculate("2 3 4 ^ ^"),
            2417851639229258349412352.0
        );
    }

    #[test]
    fn infix_to_postfix() {
        assert_eq!(super::infix_to_postfix("1 + 2 * 3"), "1 2 3 * +");
        assert_eq!(super::infix_to_postfix("1 * 2 + 3"), "1 2 * 3 +");
        assert_eq!(super::infix_to_postfix("1 * 2 / 3 + 4"), "1 2 * 3 / 4 +");

        assert_eq!(super::infix_to_postfix("2 ^ 3 ^ 4"), "2 3 4 ^ ^");
        assert_eq!(super::infix_to_postfix("2 ^ 3 ^ 4 ^ 5"), "2 3 4 5 ^ ^ ^");
        assert_eq!(super::infix_to_postfix("2 * 3 ^ 4"), "2 3 4 ^ *");
        assert_eq!(super::infix_to_postfix("2 ^ 3 * 4"), "2 3 ^ 4 *");

        assert_eq!(super::infix_to_postfix("( 1 + 2 ) * 3"), "1 2 + 3 *");
        assert_eq!(super::infix_to_postfix("1 * ( 2 + 3 )"), "1 2 3 + *");
        assert_eq!(
            super::infix_to_postfix("1 * ( 2 * ( 3 + 4 ) )"),
            "1 2 3 4 + * *"
        );
    }

    #[test]
    fn calculate() {
        assert_eq!(super::calculate("1 + 1"), 2.0);
        assert_eq!(super::calculate("1 + 2 * 3"), 7.0);
        assert_eq!(super::calculate("( 1 + 2 ) * 3"), 9.0);
        assert_eq!(super::calculate("2 ^ 3 ^ 4"), 2417851639229258349412352.0);
    }
}
