use dec::Decimal;
use serde::{Deserialize, Serialize};

const DECIMAL_PLACES_DEFAULT: usize = 27; // TODO: Use OnceLock if using CLI. The test cases can use 27.
const NAN: &str = "-0.0000000000000000000000000001"; // TODO: Use DECIMAL_PLACES_DEFAULT via OnceLock.

pub type Dec = Decimal<DECIMAL_PLACES_DEFAULT>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TreeNode {
    Op(char, Box<TreeNode>, Box<TreeNode>),
    Num(i32),
    Var(String),
    Fun(String, Box<TreeNode>),
    Paren(Box<TreeNode>),
    Empty,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BinaryAlgebraicExpressionTree {
    pub name: String,
    pub actual_tree: TreeNode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub text: String,
    pub examples: Vec<[String; 2]>,
    pub solution: Vec<BinaryAlgebraicExpressionTree>,
}

impl Default for BinaryAlgebraicExpressionTree {
    fn default() -> Self {
        BinaryAlgebraicExpressionTree {
            name: "NEW".to_string(),
            actual_tree: parse_expression("1"),
        }
    }
}

pub fn apply_algebra_to_tree_node(
    node: &TreeNode,
    x: &Dec,
    tablets: &Vec<BinaryAlgebraicExpressionTree>,
) -> Dec {
    match node {
        TreeNode::Num(n) => Dec::from(*n),
        TreeNode::Var(s) => {
            if s == "x" {
                *x
            } else {
                s.parse::<Dec>()
                    .unwrap_or_else(|_| panic!("Unexpected variable: {s}"))
            }
        }
        TreeNode::Fun(name, arg) => {
            let arg_value = apply_algebra_to_tree_node(arg, x, tablets);
            match name.as_str() {
                "GE0" => math_trick::ge0(arg_value).parse().unwrap(),
                "IS0" => math_trick::is0(arg_value).parse().unwrap(),
                "FLOOR1" => math_trick::floor1(arg_value).parse().unwrap(),
                "RIGHT" => math_trick::right(arg_value.to_standard_notation_string())
                    .parse()
                    .unwrap(),
                "LEFT" => math_trick::left(arg_value.to_standard_notation_string())
                    .parse()
                    .unwrap(),
                _ => {
                    let tablet = tablets
                        .iter()
                        .find(|tablet| name == &tablet.name)
                        .unwrap_or_else(|| panic!("There is no tree called {name}"));
                    apply_algebra_to_tree_node(&tablet.actual_tree, &arg_value, tablets)
                }
            }
        }
        TreeNode::Op(op, left, right) => {
            let left_val = apply_algebra_to_tree_node(left, x, tablets);
            let right_val = apply_algebra_to_tree_node(right, x, tablets);
            match op {
                '+' => left_val + right_val,
                '-' => left_val - right_val,
                '*' => left_val * right_val,
                '/' => left_val / right_val,
                '^' => {
                    let mut ctx = dec::Context::<Dec>::default();
                    ctx.set_min_exponent(-100).unwrap();
                    ctx.set_max_exponent(100).unwrap();
                    let mut result = left_val;
                    ctx.pow(&mut result, &right_val);
                    result
                }
                _ => panic!("Unknown operator: {op}"),
            }
        }
        TreeNode::Paren(expr) => apply_algebra_to_tree_node(expr, x, tablets),
        TreeNode::Empty => Dec::zero(),
    }
}

pub fn trim2(mut dec: Dec) -> String {
    dec.trim();
    let result = dec.to_standard_notation_string();
    if result == "-0" {
        "0".to_string()
    } else {
        result
    }
}

fn trim_zeros(s: &str) -> String {
    if s.contains('.') {
        let trimmed = s.trim_end_matches('0');
        trimmed.trim_end_matches('.').to_string()
    } else {
        s.to_string()
    }
}

pub fn parse_expression(s: &str) -> TreeNode {
    let tokens: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
    let mut index = 0;
    parse_additive(&tokens, &mut index)
}

fn parse_additive(tokens: &[char], index: &mut usize) -> TreeNode {
    let mut left = parse_multiplicative(tokens, index);
    while *index < tokens.len() {
        match tokens[*index] {
            '+' | '-' => {
                let op = tokens[*index];
                *index += 1;
                let right = parse_multiplicative(tokens, index);
                left = TreeNode::Op(op, Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    left
}

fn parse_multiplicative(tokens: &[char], index: &mut usize) -> TreeNode {
    let mut left = parse_power(tokens, index);
    while *index < tokens.len() {
        match tokens[*index] {
            '*' | '/' => {
                let op = tokens[*index];
                *index += 1;
                let right = parse_power(tokens, index);
                left = TreeNode::Op(op, Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    left
}

fn parse_power(tokens: &[char], index: &mut usize) -> TreeNode {
    let mut left = parse_atomic(tokens, index);
    while *index < tokens.len() && tokens[*index] == '^' {
        let op = tokens[*index];
        *index += 1;
        let right = parse_atomic(tokens, index);
        left = TreeNode::Op(op, Box::new(left), Box::new(right));
    }
    left
}

fn parse_atomic(tokens: &[char], index: &mut usize) -> TreeNode {
    if *index >= tokens.len() {
        return TreeNode::Empty;
    }
    let c = tokens[*index];
    match c {
        '(' => {
            *index += 1;
            let node = parse_additive(tokens, index);
            if *index < tokens.len() && tokens[*index] == ')' {
                *index += 1;
            }
            TreeNode::Paren(Box::new(node))
        }
        '0'..='9' => {
            let mut num = 0;
            while *index < tokens.len() && tokens[*index].is_ascii_digit() {
                num = num * 10 + tokens[*index].to_digit(10).unwrap() as i32;
                *index += 1;
            }
            TreeNode::Num(num)
        }
        'x' => {
            *index += 1;
            TreeNode::Var("x".to_string())
        }
        'A'..='Z' => {
            let mut name = String::new();
            while *index < tokens.len()
                && (tokens[*index].is_alphanumeric() || tokens[*index] == '_')
            {
                name.push(tokens[*index]);
                *index += 1;
            }
            if *index < tokens.len() && tokens[*index] == '(' {
                *index += 1;
                let arg = parse_additive(tokens, index);
                if *index < tokens.len() && tokens[*index] == ')' {
                    *index += 1;
                }
                TreeNode::Fun(name, Box::new(arg))
            } else {
                TreeNode::Var(name)
            }
        }
        _ => TreeNode::Empty,
    }
}

pub fn level_order_to_array(root: TreeNode) -> [String; 15] {
    let mut result = std::array::from_fn(|_| String::new());
    let mut queue = std::collections::VecDeque::with_capacity(15);
    queue.push_back((0, root));
    while let Some((i, node)) = queue.pop_front() {
        if i >= 15 {
            continue;
        }
        match node {
            TreeNode::Op(op, left, right) => {
                result[i] = op.to_string();
                queue.push_back((2 * i + 1, *left));
                queue.push_back((2 * i + 2, *right));
            }
            TreeNode::Num(n) => result[i] = n.to_string(),
            TreeNode::Var(v) => result[i] = v,
            TreeNode::Fun(name, arg) => {
                result[i] = name;
                queue.push_back((2 * i + 2, *arg));
            }
            TreeNode::Paren(expr) => {
                result[i] = "()".to_string();
                queue.push_back((2 * i + 1, *expr));
            }
            TreeNode::Empty => {}
        }
    }
    result
}

pub fn create_expression(node: TreeNode) -> String {
    fn build_expr(node: TreeNode, parent_prec: u8, is_root: bool) -> String {
        match node {
            TreeNode::Op(op, left, right) => {
                let (prec, is_left_assoc) = match op {
                    '^' => (4, false),
                    '*' | '/' => (3, true),
                    '+' | '-' => (2, true),
                    _ => (0, true),
                };
                let left_str = build_expr(*left, prec, false);
                let right_str = build_expr(*right, prec + !is_left_assoc as u8, false);
                let expr = format!("{left_str}{op}{right_str}");
                if prec < parent_prec && !is_root {
                    format!("({expr})")
                } else {
                    expr
                }
            }
            TreeNode::Num(n) => n.to_string(),
            TreeNode::Var(v) => v,
            TreeNode::Fun(name, arg) => format!("{}({})", name, build_expr(*arg, 0, false)),
            TreeNode::Paren(expr) => {
                let inner = build_expr(*expr, 0, true);
                if parent_prec > 0 {
                    format!("({inner})")
                } else {
                    inner
                }
            }
            TreeNode::Empty => String::new(),
        }
    }
    build_expr(node, 0, true)
}

pub fn get_tasks() -> &'static Vec<TestCase> {
    static INSTANCE: std::sync::OnceLock<Vec<TestCase>> = std::sync::OnceLock::new();
    INSTANCE.get_or_init(|| {
        vec![
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["2".to_string(), "27".to_string()],
                    ["-0.2424".to_string(), "27".to_string()],
                    ["100".to_string(), "27".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "DECIMAL_PLACES".to_string(),
                    actual_tree: parse_expression("27")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["-1".to_string(), "1".to_string()],
                    ["11.9".to_string(), "11.9".to_string()],
                    ["0".to_string(), "0".to_string()],
                    ["-0.0024".to_string(), "0.0024".to_string()],
                    ["1".to_string(), "1".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "ABS".to_string(),
                    actual_tree: parse_expression("(x^2)^(1/2)")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["0".to_string(), "NaN".to_string()],
                    ["0.3".to_string(), "1".to_string()],
                    ["-0.3".to_string(), "0".to_string()],
                    ["1.0".to_string(), "1".to_string()],
                    ["400.0".to_string(), "1".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "H".to_string(),
                    actual_tree: parse_expression("(x+ABS(x))/(2*x)")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    [
                        "55".to_string(),
                        "0.".to_string() + &"0".repeat(DECIMAL_PLACES_DEFAULT - 1) + "1",
                    ],
                    [
                        "-11.9".to_string(),
                        "0.".to_string() + &"0".repeat(DECIMAL_PLACES_DEFAULT - 1) + "1",
                    ],
                    [
                        "0.0".to_string(),
                        "0.".to_string() + &"0".repeat(DECIMAL_PLACES_DEFAULT - 1) + "1",
                    ],
                    [
                        "-0.95".to_string(),
                        "0.".to_string() + &"0".repeat(DECIMAL_PLACES_DEFAULT - 1) + "1",
                    ],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "TINY".to_string(),
                    actual_tree: parse_expression("10^(-DECIMAL_PLACES(x)))")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    [
                        "0.".to_string() + &"9".repeat(DECIMAL_PLACES_DEFAULT),
                        "1".to_string(),
                    ],
                    [
                        "-0.".to_string() + &"0".repeat(DECIMAL_PLACES_DEFAULT) + "1",
                        "NaN".to_string(),
                    ],
                    ["0.3".to_string(), "1".to_string()],
                    ["-0.3".to_string(), "0".to_string()],
                    ["1.0".to_string(), "1".to_string()],
                    ["400.0".to_string(), "1".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "GE0".to_string(),
                    actual_tree: parse_expression("H(x+TINY(x)/10)")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["0".to_string(), "1".to_string()],
                    ["-6.4".to_string(), "1".to_string()],
                    ["1.0".to_string(), "0".to_string()],
                    ["0.999".to_string(), "1".to_string()],
                    ["50".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "LT1".to_string(),
                    actual_tree: parse_expression("1-GE0(x-1)")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["0".to_string(), "1".to_string()],
                    ["0.5".to_string(), "1".to_string()],
                    ["1".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "IS0".to_string(),
                    actual_tree: parse_expression("GE0(x)*LT1(x)")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["1".to_string(), "1".to_string()],
                    ["1.5".to_string(), "1".to_string()],
                    ["2".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "IS1".to_string(),
                    actual_tree: parse_expression("IS0(x-1)")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["2".to_string(), "1".to_string()],
                    ["2.5".to_string(), "1".to_string()],
                    ["3".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "IS2".to_string(),
                    actual_tree: parse_expression("IS0(x-2)")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["3".to_string(), "1".to_string()],
                    ["3.5".to_string(), "1".to_string()],
                    ["4".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "IS3".to_string(),
                    actual_tree: parse_expression("IS0(x-3)")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["4".to_string(), "1".to_string()],
                    ["4.5".to_string(), "1".to_string()],
                    ["5".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "IS4".to_string(),
                    actual_tree: parse_expression("IS0(x-4)")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["5".to_string(), "1".to_string()],
                    ["5.5".to_string(), "1".to_string()],
                    ["6".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "IS5".to_string(),
                    actual_tree: parse_expression("IS0(x-5)")
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["6".to_string(), "1".to_string()],
                    ["6.5".to_string(), "1".to_string()],
                    ["7".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "IS6".to_string(),
                    actual_tree: parse_expression("IS0(x-6)"),
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["7".to_string(), "1".to_string()],
                    ["7.5".to_string(), "1".to_string()],
                    ["8".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "IS7".to_string(),
                    actual_tree: parse_expression("IS0(x-7)"),
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["8".to_string(), "1".to_string()],
                    ["8.5".to_string(), "1".to_string()],
                    ["9".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "IS8".to_string(),
                    actual_tree: parse_expression("IS0(x-8)"),
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["9".to_string(), "1".to_string()],
                    ["9.5".to_string(), "1".to_string()],
                    ["10".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "IS9".to_string(),
                    actual_tree: parse_expression("IS0(x-9)"),
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["0".to_string(), "0".to_string()],
                    ["0.2".to_string(), "0".to_string()],
                    ["1".to_string(), "1".to_string()],
                    ["1.2".to_string(), "1".to_string()],
                    ["2".to_string(), "2".to_string()],
                    ["2.2".to_string(), "2".to_string()],
                    ["3".to_string(), "3".to_string()],
                    ["3.2".to_string(), "3".to_string()],
                    ["4".to_string(), "4".to_string()],
                    ["4.2".to_string(), "4".to_string()],
                    ["5".to_string(), "5".to_string()],
                    ["5.2".to_string(), "5".to_string()],
                    ["6".to_string(), "6".to_string()],
                    ["6.2".to_string(), "6".to_string()],
                    ["7".to_string(), "7".to_string()],
                    ["7.2".to_string(), "7".to_string()],
                    ["8".to_string(), "8".to_string()],
                    ["8.2".to_string(), "8".to_string()],
                    ["9".to_string(), "9".to_string()],
                    ["9.2".to_string(), "9".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "FLOOR1".to_string(),
                    actual_tree: parse_expression(
                        "IS1(x)+2*IS2(x)+3*IS3(x)+4*IS4(x)+5*IS5(x)+6*IS6(x)+7*IS7(x)+8*IS8(x)+9*IS9(x)",
                    ),
                }],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["0.06".to_string(), "0.6".to_string()],
                    [
                        "0.12345678".to_string(),
                        "0.2345678".to_string() + &"0".repeat(DECIMAL_PLACES_DEFAULT - 8) + "1",
                    ],
                    [
                        "0.7".to_string(),
                        "0.".to_string() + &"0".repeat(DECIMAL_PLACES_DEFAULT - 1) + "7",
                    ],
                ],
                solution: vec![
                    BinaryAlgebraicExpressionTree {
                        name: "RIGHT".to_string(),
                    actual_tree: parse_expression("x*10-FLOOR1(x*10)+FLOOR1(x*10)*TINY(x)")
                    },
                ],
            },
            TestCase {
                text: "".to_string(),
                examples: vec![
                    ["0.6".to_string(), "0.06".to_string()],
                    [
                        "0.2345678".to_string() + &"0".repeat(DECIMAL_PLACES_DEFAULT - 8) + "1",
                        "0.12345678".to_string(),
                    ],
                    [
                        "0.".to_string() + &"0".repeat(DECIMAL_PLACES_DEFAULT - 1) + "7",
                        "0.7".to_string(),
                    ],
                ],
                solution: vec![
                    BinaryAlgebraicExpressionTree {
                        name: "LEFT".to_string(),
                    actual_tree: parse_expression(
                        "RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(RIGHT(x))))))))))))))))))))))))))",
                    )
                    },
                ],
            },
        ]
    })
}

pub mod math_trick {
    use super::*;

    pub fn ge0(x: Dec) -> String {
        let nan: Dec = NAN.parse().unwrap();
        match x {
            _ if x > nan => "1".to_string(),
            _ if x < nan => "0".to_string(),
            _ => "NaN".to_string(),
        }
    }

    pub fn is0(x: Dec) -> String {
        let nan: Dec = NAN.parse().unwrap();
        match x {
            _ if x < nan => "0".to_string(),
            _ if x > nan && x < "1".parse::<Dec>().unwrap() + nan => "1".to_string(),
            _ if x > "1".parse::<Dec>().unwrap() + nan => "0".to_string(),
            _ => "NaN".to_string(),
        }
    }

    pub fn floor1(x: Dec) -> String {
        let nan: Dec = NAN.parse().unwrap();
        match x {
            _ if x < nan => "0".to_string(),
            _ if x > nan && x < "1".parse::<Dec>().unwrap() + nan => "0".to_string(),
            _ if x > "1".parse::<Dec>().unwrap() + nan && x < "2".parse::<Dec>().unwrap() + nan => {
                "1".to_string()
            }
            _ if x > "2".parse::<Dec>().unwrap() + nan && x < "3".parse::<Dec>().unwrap() + nan => {
                "2".to_string()
            }
            _ if x > "3".parse::<Dec>().unwrap() + nan && x < "4".parse::<Dec>().unwrap() + nan => {
                "3".to_string()
            }
            _ if x > "4".parse::<Dec>().unwrap() + nan && x < "5".parse::<Dec>().unwrap() + nan => {
                "4".to_string()
            }
            _ if x > "5".parse::<Dec>().unwrap() + nan && x < "6".parse::<Dec>().unwrap() + nan => {
                "5".to_string()
            }
            _ if x > "6".parse::<Dec>().unwrap() + nan && x < "7".parse::<Dec>().unwrap() + nan => {
                "6".to_string()
            }
            _ if x > "7".parse::<Dec>().unwrap() + nan && x < "8".parse::<Dec>().unwrap() + nan => {
                "7".to_string()
            }
            _ if x > "8".parse::<Dec>().unwrap() + nan && x < "9".parse::<Dec>().unwrap() + nan => {
                "8".to_string()
            }
            _ if x > "9".parse::<Dec>().unwrap() + nan
                && x < "10".parse::<Dec>().unwrap() + nan =>
            {
                "9".to_string()
            }
            _ if x > "10".parse::<Dec>().unwrap() + nan => "0".to_string(),
            _ => "NaN".to_string(),
        }
    }

    /// num should be a to_standard_notation_string().
    pub fn right(mut num: String) -> String {
        // TODO: NaN.
        if !num.contains('.') {
            num += ".0";
        }
        if num.ends_with('0') {
            num = num.trim_end_matches('0').to_string()
        }
        if num.ends_with('.') {
            num += "0";
        }
        let decimal_pos = num.find('.').unwrap();
        let (integer_part, fractional_part) = num.split_at(decimal_pos + 1);
        let mut chars: Vec<_> = fractional_part.chars().collect();
        let len = DECIMAL_PLACES_DEFAULT - chars.len();
        if len > 0 {
            chars.extend(vec!['0'; len]);
        }
        chars.rotate_left(1);
        let rotated_fractional_part: String = chars.into_iter().collect();
        let result = format!("{integer_part}{rotated_fractional_part}");
        trim_zeros(&result)
    }

    /// num should be a to_standard_notation_string().
    pub fn left(mut num: String) -> String {
        // TODO: NaN.
        if !num.contains('.') {
            num += ".0";
        }
        if num.ends_with('0') {
            num = num.trim_end_matches('0').to_string()
        }
        if num.ends_with('.') {
            num += "0";
        }
        let decimal_pos = num.find('.').unwrap();
        let (integer_part, fractional_part) = num.split_at(decimal_pos + 1);
        let mut chars: Vec<_> = fractional_part.chars().collect();
        let len = DECIMAL_PLACES_DEFAULT - chars.len();
        if len > 0 {
            chars.extend(vec!['0'; len]);
        }
        chars.rotate_right(1);
        let rotated_fractional_part: String = chars.into_iter().collect();
        let result = format!("{integer_part}{rotated_fractional_part}");
        trim_zeros(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_solutions() {
        let tasks = get_tasks();
        let mut trees: Vec<BinaryAlgebraicExpressionTree> = vec![];
        for task in tasks {
            for tree in &task.solution {
                trees.push(tree.clone());
            }
        }
        for i in 0..tasks.len() {
            for [input, output] in &tasks[i].examples {
                let name_function = &tasks[i].solution.last().unwrap().name;
                let name = format!("{}({}) = ", name_function, input);
                let result = trim2(apply_algebra_to_tree_node(
                    &tasks[i].solution.last().unwrap().actual_tree,
                    &input.parse::<Dec>().unwrap(),
                    &trees,
                ));
                assert_eq!(name.clone() + output.as_str(), name + &result);
            }
        }
    }

    #[test]
    fn test_math_tricks() {
        let tasks = get_tasks();
        let mut trees: Vec<BinaryAlgebraicExpressionTree> = vec![];
        for task in tasks {
            for tree in &task.solution {
                trees.push(tree.clone());
            }
        }
        for i in 0..tasks.len() {
            for [input, output] in &tasks[i].examples {
                let name_function = &tasks[i].solution.last().unwrap().name;
                let name = format!("{}({}) = ", name_function, input);
                let input_dec = input.parse().unwrap();
                if name_function == "GE0" {
                    assert_eq!(
                        name.clone() + output.as_str(),
                        name + &math_trick::ge0(input_dec)
                    );
                } else if name_function == "IS0" {
                    assert_eq!(
                        name.clone() + output.as_str(),
                        name + &math_trick::is0(input_dec)
                    );
                } else if name_function == "FLOOR1" {
                    assert_eq!(
                        name.clone() + output.as_str(),
                        name + &math_trick::floor1(input_dec)
                    );
                } else if name_function == "RIGHT" {
                    assert_eq!(
                        name.clone() + output.as_str(),
                        name + &math_trick::right(input_dec.to_standard_notation_string())
                    );
                } else if name_function == "LEFT" {
                    assert_eq!(
                        name.clone() + output.as_str(),
                        name + &math_trick::left(input_dec.to_standard_notation_string())
                    );
                }
            }
        }
    }
}
