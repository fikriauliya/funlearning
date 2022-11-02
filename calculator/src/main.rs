use std::collections::HashMap;

fn main() {}

#[derive(PartialEq, Eq, Hash, Debug)]
enum CalculatorError {
    InvalidToken(String),
    TooManyOperands,
    NotEnoughOperators,
}

fn postfix_calculate(equation: &str) -> Result<f64, CalculatorError> {
    let mut stack: Vec<f64> = Vec::new();
    let tokens = equation.split_whitespace();
    for token in tokens {
        match token {
            "+" | "-" | "*" | "/" | "^" => {
                let right = stack.pop().ok_or(CalculatorError::NotEnoughOperators)?;
                let left = stack.pop().ok_or(CalculatorError::NotEnoughOperators)?;
                let result = match token {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    "^" => left.powf(right),
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            _ => {
                let num = token
                    .parse::<f64>()
                    .map_err(|_| CalculatorError::InvalidToken(token.to_string()))?;
                stack.push(num);
            }
        }
    }
    if stack.len() > 1 {
        return Err(CalculatorError::TooManyOperands);
    }
    return stack.pop().ok_or(CalculatorError::NotEnoughOperators);
}

fn infix_to_postfix(equation: &str) -> Result<String, CalculatorError> {
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
                let to_be_inserted_precedence = precedence
                    .get(token)
                    .ok_or(CalculatorError::InvalidToken(token.to_string()))?;
                while !operators.is_empty() {
                    let top_precedence = precedence
                        .get(operators.last().unwrap())
                        .ok_or(CalculatorError::InvalidToken(token.to_string()))?;
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
                while !operators.is_empty() && operators.last().unwrap() != &"(" {
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
    Ok(results.join(" "))
}

fn calculate(equation: &str) -> Result<f64, CalculatorError> {
    let postfix = infix_to_postfix(equation)?;
    return postfix_calculate(&postfix);
}

#[cfg(test)]
mod tests {
    #[test]
    fn postfix_calculate() {
        assert_eq!(super::postfix_calculate("1 2 + 3 *"), Ok(9.0));
        assert_eq!(super::postfix_calculate("1 2 + 3 * 4 +"), Ok(13.0));
        assert_eq!(
            super::postfix_calculate("2 3 4 ^ ^"),
            Ok(2417851639229258349412352.0)
        );

        assert_eq!(
            super::postfix_calculate("1 5"),
            Err(super::CalculatorError::TooManyOperands)
        );
        assert_eq!(
            super::postfix_calculate("1 +"),
            Err(super::CalculatorError::NotEnoughOperators)
        );
        assert_eq!(
            super::postfix_calculate("1 e + 3"),
            Err(super::CalculatorError::InvalidToken("e".to_string()))
        );
        assert_eq!(
            super::postfix_calculate(""),
            Err(super::CalculatorError::NotEnoughOperators)
        );
    }

    #[test]
    fn infix_to_postfix() {
        assert_eq!(
            super::infix_to_postfix("1 + 2 * 3"),
            Ok("1 2 3 * +".to_string())
        );
        assert_eq!(
            super::infix_to_postfix("1 * 2 + 3"),
            Ok("1 2 * 3 +".to_string())
        );
        assert_eq!(
            super::infix_to_postfix("1 * 2 / 3 + 4"),
            Ok("1 2 * 3 / 4 +".to_string())
        );

        assert_eq!(
            super::infix_to_postfix("2 ^ 3 ^ 4"),
            Ok("2 3 4 ^ ^".to_string())
        );
        assert_eq!(
            super::infix_to_postfix("2 ^ 3 ^ 4 ^ 5"),
            Ok("2 3 4 5 ^ ^ ^".to_string())
        );
        assert_eq!(
            super::infix_to_postfix("2 * 3 ^ 4"),
            Ok("2 3 4 ^ *".to_string())
        );
        assert_eq!(
            super::infix_to_postfix("2 ^ 3 * 4"),
            Ok("2 3 ^ 4 *".to_string())
        );

        assert_eq!(
            super::infix_to_postfix("( 1 + 2 ) * 3"),
            Ok("1 2 + 3 *".to_string())
        );
        assert_eq!(
            super::infix_to_postfix("1 * ( 2 + 3 )"),
            Ok("1 2 3 + *".to_string())
        );
        assert_eq!(
            super::infix_to_postfix("1 * ( 2 * ( 3 + 4 ) )"),
            Ok("1 2 3 4 + * *".to_string())
        );
    }

    #[test]
    fn calculate() {
        assert_eq!(super::calculate("1 + 1"), Ok(2.0));
        assert_eq!(super::calculate("1 + 2 * 3"), Ok(7.0));
        assert_eq!(super::calculate("( 1 + 2 ) * 3"), Ok(9.0));

        assert_eq!(
            super::calculate("2 ^ 3 ^ 4"),
            Ok(2417851639229258349412352.0)
        );
        assert_eq!(
            super::calculate("2 & 3"),
            Err(super::CalculatorError::InvalidToken("&".to_string()))
        );
    }
}
