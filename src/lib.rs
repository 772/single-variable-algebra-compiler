use serde::{Deserialize, Serialize};
use std::io::{self, Read};
use std::sync::OnceLock;

mod decimal_crate;
use decimal_crate::*;

static DECIMAL_PLACES: OnceLock<usize> = OnceLock::new();
static NAN: OnceLock<String> = OnceLock::new();

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum TreeNode {
    Op(char, Box<TreeNode>, Box<TreeNode>),
    Num(String),
    Var(String),
    Fun(String, usize, Box<TreeNode>),
    Paren(Box<TreeNode>),
    Empty,
}

/// The main struct of this crate. A binary algebraic expression tree is a TreeNode.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BinaryAlgebraicExpressionTree {
    pub name: String,
    pub root_node: TreeNode,
}

/// A TestCase is useful for unit testing in `mod tests`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub description: Option<String>,
    /// A vector full of examples. The first String is the input and the second the expected output.
    pub examples: Vec<[String; 2]>,
    pub solution: Vec<BinaryAlgebraicExpressionTree>,
}

impl Default for BinaryAlgebraicExpressionTree {
    fn default() -> Self {
        BinaryAlgebraicExpressionTree {
            name: "NEW".to_string(),
            root_node: parse_expression("1"),
        }
    }
}

fn get_decimal_places() -> usize {
    *DECIMAL_PLACES.get_or_init(|| MAX_DECIMAL_PLACES)
}

fn get_nan() -> &'static String {
    NAN.get_or_init(|| format!("-0.{}1", "0".repeat(get_decimal_places())))
}

#[cfg(not(target_arch = "wasm32"))]
fn output(s: String) {
    println!("{s}");
}

#[cfg(target_arch = "wasm32")]
fn output(s: String) {
    use web_sys::wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let textarea = document
        .get_element_by_id("output")
        .unwrap()
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap();
    textarea.set_value(&s);
}

#[cfg(not(target_arch = "wasm32"))]
fn read_args() -> Vec<String> {
    std::env::args().skip(1).collect()
}

#[cfg(target_arch = "wasm32")]
fn read_args() -> Vec<String> {
    use web_sys::wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    vec![
        document
            .get_element_by_id("input")
            .unwrap()
            .dyn_into::<web_sys::HtmlTextAreaElement>()
            .unwrap()
            .value(),
    ]
}

/// For CLI.
pub fn read_input() {
    let mut use_math_tricks = false;
    let args: Vec<String> = read_args();
    if args.len() == 1
        && let Some(first_line) = args[0].trim().lines().next()
        && first_line.trim().starts_with("decimals(x) =")
    {
        use_math_tricks = true;
        if let Some(k_str) = first_line.trim().strip_prefix("decimals(x) =") {
            let k_clean = k_str.split_whitespace().next().unwrap_or(k_str);
            if let Ok(k) = k_clean.parse::<usize>() {
                let _ = DECIMAL_PLACES.set(k);
                let _ = NAN.set(format!("-0.{}1", "0".repeat(k)));
            }
        }
    }
    let input = if args.len() == 1 && args[0].contains('\n') {
        args[0]
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    } else if args.is_empty() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    } else {
        args
    };
    if input.is_empty() {
        println!(
            "Usage:\n  single-variable-algebra-compiler FUNC1(x)=expr1 FUNC2(x)=expr2 ... FUNCN(INPUT)\nOR\n  echo 'F(x)=4+4\nG(x)=F(x)*2\nG(1)' | single-variable-algebra-compiler"
        );
        return;
    }
    let mut trees = Vec::new();
    for arg in input.iter().take(input.len() - 1) {
        if let Some((name, expr)) = parse_function(arg) {
            let tree = BinaryAlgebraicExpressionTree {
                name: name.to_string(),
                root_node: parse_expression(expr),
            };
            trees.push(tree);
        } else {
            output(format!("Invalid function definition: {arg}"));
            return;
        }
    }
    if let Some((func_name, input_val)) = parse_function_call(input.last().unwrap()) {
        if let Some(tree) = trees.iter().find(|t| t.name == func_name) {
            let x: Dec = input_val.parse().unwrap_or_else(|_| {
                output(format!("Invalid input value: {input_val}"));
                std::process::exit(1);
            });
            let result = apply_algebra_to_tree_node(&tree.root_node, &x, &trees, use_math_tricks);
            output(result.map_or("Undefined".to_string(), trim2));
        } else {
            output(format!("Function {func_name} not defined"));
        }
    } else {
        output(format!("Invalid function call: {}", input.last().unwrap()));
    }
}

/// Calculate the result of a binary tree.
pub fn apply_algebra_to_tree_node(
    node: &TreeNode,
    x: &Dec,
    tablets: &Vec<BinaryAlgebraicExpressionTree>,
    use_math_tricks: bool,
) -> Option<Dec> {
    match node {
        TreeNode::Num(n) => Some(n.parse::<Dec>().unwrap()),
        TreeNode::Var(s) => {
            if s == "x" {
                Some(x.clone())
            } else {
                Some(
                    s.parse::<Dec>()
                        .unwrap_or_else(|_| panic!("Unexpected variable: {s}")),
                )
            }
        }
        TreeNode::Fun(name, iterate, arg) => {
            let mut arg_value = apply_algebra_to_tree_node(arg, x, tablets, use_math_tricks);
            /*let mut old_value = arg_value.clone();*/
            for _ in 0..*iterate {
                if name.as_str() == "abs" && use_math_tricks {
                    arg_value = Some(math_trick::abs(arg_value.unwrap()).parse().unwrap());
                } else if name.as_str() == "ge0" && use_math_tricks {
                    arg_value = Some(math_trick::ge0(arg_value.unwrap()).parse().unwrap());
                } else if name.as_str() == "is0" && use_math_tricks {
                    arg_value = Some(math_trick::is0(arg_value.unwrap()).parse().unwrap());
                } else if name.as_str() == "floor1" && use_math_tricks {
                    arg_value = Some(math_trick::floor1(arg_value.unwrap()).parse().unwrap());
                } else if name.as_str() == "left" && use_math_tricks {
                    arg_value = Some(math_trick::left(arg_value.unwrap()).parse().unwrap());
                } else {
                    let tablet = tablets
                        .iter()
                        .find(|tablet| name == &tablet.name)
                        .unwrap_or_else(|| panic!("There is no tree called {name}"));
                    arg_value = apply_algebra_to_tree_node(
                        &tablet.root_node,
                        &arg_value.unwrap(),
                        tablets,
                        use_math_tricks,
                    );
                }
                /*if arg_value == old_value {
                    break;
                }
                old_value = arg_value.clone();*/
            }
            arg_value
        }
        TreeNode::Op(op, left, right) => {
            let left_val = apply_algebra_to_tree_node(left, x, tablets, use_math_tricks);
            let right_val = apply_algebra_to_tree_node(right, x, tablets, use_math_tricks);
            let left_str = trim2(left_val.clone().unwrap());
            let right_str = trim2(right_val.clone().unwrap());
            match op {
                '+' => Some(left_val.unwrap() + right_val.unwrap()),
                '-' => Some(left_val.unwrap() - right_val.unwrap()),
                '*' => Some(left_val.unwrap() * right_val.unwrap()),
                '/' => {
                    if right_str == "0" {
                        return None;
                    }
                    Some(left_val.unwrap() / right_val.unwrap())
                }
                '^' => {
                    if (left_str == "0" && (right_val <= Some(zero())))
                        || (left_val < Some(zero()) && right_str.contains('.'))
                    {
                        return None;
                    }
                    Some(pow(left_val.unwrap(), right_val.unwrap()))
                }
                _ => panic!("Unknown operator: {op}"),
            }
        }
        TreeNode::Paren(expr) => apply_algebra_to_tree_node(expr, x, tablets, use_math_tricks),
        TreeNode::Empty => Some(zero()),
    }
}

/// Converts a String like 3*x+5 to a binary tree.
pub fn parse_expression(s: &str) -> TreeNode {
    let tokens: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
    let mut index = 0;
    parse_additive(&tokens, &mut index)
}

/// Converts a binary tree back to a String like 3*x+5.
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
            TreeNode::Fun(name, iterate, arg) => {
                let mut iterating = "".to_string();
                if iterate > 1 {
                    iterating = "^[".to_string() + &iterate.to_string() + "]";
                }
                format!("{}{}({})", name, iterating, build_expr(*arg, 0, false))
            }
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
            TreeNode::Fun(name, _iterate, arg) => {
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

fn trim2(dec: Dec) -> String {
    let mut x = dec_to_string(dec);
    if x.contains('.') {
        x = x.trim_end_matches('0').trim_end_matches('.').to_string();
    }
    if x.starts_with("-0") {
        x.remove(0);
    }
    x
}

fn parse_function(s: &str) -> Option<(&str, &str)> {
    let s = s.trim();
    if let Some((func_part, expr)) = s.split_once('=') {
        let func_part = func_part.trim();
        if let Some((name, _)) = func_part.split_once('(') {
            return Some((name, expr.trim()));
        }
    }
    None
}

fn parse_function_call(s: &str) -> Option<(&str, &str)> {
    if !s.ends_with(')') || !s.contains('(') {
        return None;
    }
    let open_paren = s.find('(')?;
    let name = &s[..open_paren];
    let input = &s[open_paren + 1..s.len() - 1];
    Some((name, input))
}

fn trim_zeros(s: &str) -> String {
    if s.contains('.') {
        let trimmed = s.trim_end_matches('0');
        trimmed.trim_end_matches('.').to_string()
    } else {
        s.to_string()
    }
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
            let mut num_str = String::new();
            while *index < tokens.len() && tokens[*index].is_ascii_digit() {
                num_str.push(tokens[*index]);
                *index += 1;
            }
            if *index < tokens.len() && tokens[*index] == '.' {
                num_str.push(tokens[*index]);
                *index += 1;
                while *index < tokens.len() && tokens[*index].is_ascii_digit() {
                    num_str.push(tokens[*index]);
                    *index += 1;
                }
            }
            TreeNode::Num(num_str)
        }
        'A'..='Z' | 'a'..='z' => {
            let mut name = String::new();
            let mut iterate: usize = 1;
            while *index < tokens.len()
                && (tokens[*index].is_alphanumeric() || tokens[*index] == '_')
            {
                name.push(tokens[*index]);
                *index += 1;
            }
            if *index < tokens.len() && tokens[*index] == '^' {
                let saved_index = *index;
                *index += 1;
                if *index < tokens.len() && tokens[*index] == '[' {
                    *index += 1;
                    let mut num_str = String::new();
                    while *index < tokens.len() && tokens[*index].is_ascii_digit() {
                        num_str.push(tokens[*index]);
                        *index += 1;
                    }
                    if *index < tokens.len() && tokens[*index] == ']' {
                        *index += 1;
                        if let Ok(parsed_iterate) = num_str.parse::<usize>() {
                            iterate = parsed_iterate;
                        }
                    } else {
                        *index = saved_index;
                    }
                } else {
                    *index = saved_index;
                }
            }

            if *index < tokens.len() && tokens[*index] == '(' {
                *index += 1;
                let arg = parse_additive(tokens, index);
                if *index < tokens.len() && tokens[*index] == ')' {
                    *index += 1;
                }
                TreeNode::Fun(name, iterate, Box::new(arg))
            } else if name == "x" {
                TreeNode::Var("x".to_string())
            } else {
                TreeNode::Empty
            }
        }
        _ => {
            *index += 1;
            TreeNode::Empty
        }
    }
}

fn parse_unary(tokens: &[char], index: &mut usize) -> TreeNode {
    if *index >= tokens.len() {
        return TreeNode::Empty;
    }

    // Check for unary operators: +, -
    match tokens[*index] {
        '+' => {
            *index += 1;
            parse_unary(tokens, index) // Unary plus, just skip it
        }
        '-' => {
            *index += 1;
            // Check if this is a negative number
            if *index < tokens.len() && (tokens[*index].is_ascii_digit() || tokens[*index] == '.') {
                // Parse as negative number
                let mut num_str = String::from("-");
                while *index < tokens.len() && tokens[*index].is_ascii_digit() {
                    num_str.push(tokens[*index]);
                    *index += 1;
                }
                if *index < tokens.len() && tokens[*index] == '.' {
                    num_str.push(tokens[*index]);
                    *index += 1;
                    while *index < tokens.len() && tokens[*index].is_ascii_digit() {
                        num_str.push(tokens[*index]);
                        *index += 1;
                    }
                }
                TreeNode::Num(num_str)
            } else {
                // Unary minus operator
                TreeNode::Op(
                    '-',
                    Box::new(TreeNode::Num("0".to_string())),
                    Box::new(parse_unary(tokens, index)),
                )
            }
        }
        _ => parse_atomic(tokens, index),
    }
}

// Update parse_power to use parse_unary instead of parse_atomic
fn parse_power(tokens: &[char], index: &mut usize) -> TreeNode {
    let mut left = parse_unary(tokens, index);
    while *index < tokens.len() && tokens[*index] == '^' {
        let op = tokens[*index];
        *index += 1;
        let right = parse_unary(tokens, index);
        left = TreeNode::Op(op, Box::new(left), Box::new(right));
    }
    left
}

pub mod math_trick {
    use super::*;

    pub fn abs(x: Dec) -> String {
        let mut res = x.to_string();
        if res.starts_with('-') {
            res.remove(0);
        }
        res
    }

    pub fn ge0(x: Dec) -> String {
        let nan: Dec = get_nan().parse().unwrap();
        match x {
            _ if x > nan => "1".to_string(),
            _ if x < nan => "0".to_string(),
            _ => "NaN".to_string(),
        }
    }

    pub fn is0(x: Dec) -> String {
        let nan: Dec = get_nan().parse().unwrap();
        match x {
            _ if x < nan => "0".to_string(),
            _ if x > nan && x < "1".parse::<Dec>().unwrap() + &nan => "1".to_string(),
            _ if x > "1".parse::<Dec>().unwrap() + nan => "0".to_string(),
            _ => "NaN".to_string(),
        }
    }

    pub fn floor1(x: Dec) -> String {
        let nan: Dec = get_nan().parse().unwrap();
        match x {
            _ if x < nan => "0".to_string(),
            _ if x > nan && x < "1".parse::<Dec>().unwrap() + &nan => "0".to_string(),
            _ if x > "1".parse::<Dec>().unwrap() + &nan
                && x < "2".parse::<Dec>().unwrap() + &nan =>
            {
                "1".to_string()
            }
            _ if x > "2".parse::<Dec>().unwrap() + &nan
                && x < "3".parse::<Dec>().unwrap() + &nan =>
            {
                "2".to_string()
            }
            _ if x > "3".parse::<Dec>().unwrap() + &nan
                && x < "4".parse::<Dec>().unwrap() + &nan =>
            {
                "3".to_string()
            }
            _ if x > "4".parse::<Dec>().unwrap() + &nan
                && x < "5".parse::<Dec>().unwrap() + &nan =>
            {
                "4".to_string()
            }
            _ if x > "5".parse::<Dec>().unwrap() + &nan
                && x < "6".parse::<Dec>().unwrap() + &nan =>
            {
                "5".to_string()
            }
            _ if x > "6".parse::<Dec>().unwrap() + &nan
                && x < "7".parse::<Dec>().unwrap() + &nan =>
            {
                "6".to_string()
            }
            _ if x > "7".parse::<Dec>().unwrap() + &nan
                && x < "8".parse::<Dec>().unwrap() + &nan =>
            {
                "7".to_string()
            }
            _ if x > "8".parse::<Dec>().unwrap() + &nan
                && x < "9".parse::<Dec>().unwrap() + &nan =>
            {
                "8".to_string()
            }
            _ if x > "9".parse::<Dec>().unwrap() + &nan
                && x < "10".parse::<Dec>().unwrap() + &nan =>
            {
                "9".to_string()
            }
            _ if x > "10".parse::<Dec>().unwrap() + &nan => "0".to_string(),
            _ => "NaN".to_string(),
        }
    }

    /// num should be a to_standard_notation_string().
    pub fn left(x: Dec) -> String {
        // left(x) and right(x) only consist of several floor(x*10). That means this here should ne enough to get all NaNs.
        let mut num = dec_to_string(x);
        if floor1(num.parse::<Dec>().unwrap() * "10".parse::<Dec>().unwrap()) == "NaN" {
            return "NaN".to_string();
        }
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
        let len = get_decimal_places() - chars.len();
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

    fn get_test_cases() -> &'static Vec<TestCase> {
        static INSTANCE: std::sync::OnceLock<Vec<TestCase>> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(|| {
        vec![
            TestCase {
                description: None,
                examples: vec![
                    ["2".to_string(), get_decimal_places().to_string()],
                    ["-0.2424".to_string(), get_decimal_places().to_string()],
                    ["100".to_string(), get_decimal_places().to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "decimal_places".to_string(),
                    root_node: parse_expression(&get_decimal_places().to_string())
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["-1".to_string(), "1".to_string()],
                    ["11.2".to_string(), "11.2".to_string()],
                    ["0".to_string(), "0".to_string()],
                    ["-0.0025".to_string(), "0.0025".to_string()],
                    ["1".to_string(), "1".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "abs".to_string(),
                    root_node: parse_expression("(x^2)^0.5")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    //["0".to_string(), "NaN".to_string()],
                    ["0.3".to_string(), "1".to_string()],
                    ["-0.3".to_string(), "0".to_string()],
                    ["1.0".to_string(), "1".to_string()],
                    ["400.0".to_string(), "1".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "H".to_string(),
                    root_node: parse_expression("(x+abs(x))/(2*x)")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    [
                        "55".to_string(),
                        "0.".to_string() + &"0".repeat(get_decimal_places() - 1) + "1",
                    ],
                    [
                        "-11.9".to_string(),
                        "0.".to_string() + &"0".repeat(get_decimal_places() - 1) + "1",
                    ],
                    [
                        "0.0".to_string(),
                        "0.".to_string() + &"0".repeat(get_decimal_places() - 1) + "1",
                    ],
                    [
                        "-0.95".to_string(),
                        "0.".to_string() + &"0".repeat(get_decimal_places() - 1) + "1",
                    ],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "tiny".to_string(),
                    root_node: parse_expression("10^(-decimal_places(x)))")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    [
                        "0.".to_string() + &"9".repeat(get_decimal_places()),
                        "1".to_string(),
                    ],
                    /*[
                        "-0.".to_string() + &"0".repeat(get_decimal_places()) + "1",
                        "NaN".to_string(),
                    ],*/
                    ["0.3".to_string(), "1".to_string()],
                    ["-0.3".to_string(), "0".to_string()],
                    ["1.0".to_string(), "1".to_string()],
                    ["400.0".to_string(), "1".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "ge0".to_string(),
                    root_node: parse_expression("H(x+tiny(x)/10)")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["0".to_string(), "1".to_string()],
                    ["-6.4".to_string(), "1".to_string()],
                    ["1.0".to_string(), "0".to_string()],
                    ["0.999".to_string(), "1".to_string()],
                    ["50".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "lt1".to_string(),
                    root_node: parse_expression("1-ge0(x-1)")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["0".to_string(), "1".to_string()],
                    ["0.5".to_string(), "1".to_string()],
                    ["1".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "is0".to_string(),
                    root_node: parse_expression("ge0(x)*lt1(x)")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["1".to_string(), "1".to_string()],
                    ["1.5".to_string(), "1".to_string()],
                    ["2".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "is1".to_string(),
                    root_node: parse_expression("is0(x-1)")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["2".to_string(), "1".to_string()],
                    ["2.5".to_string(), "1".to_string()],
                    ["3".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "is2".to_string(),
                    root_node: parse_expression("is0(x-2)")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["3".to_string(), "1".to_string()],
                    ["3.5".to_string(), "1".to_string()],
                    ["4".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "is3".to_string(),
                    root_node: parse_expression("is0(x-3)")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["4".to_string(), "1".to_string()],
                    ["4.5".to_string(), "1".to_string()],
                    ["5".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "is4".to_string(),
                    root_node: parse_expression("is0(x-4)")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["5".to_string(), "1".to_string()],
                    ["5.5".to_string(), "1".to_string()],
                    ["6".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "is5".to_string(),
                    root_node: parse_expression("is0(x-5)")
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["6".to_string(), "1".to_string()],
                    ["6.5".to_string(), "1".to_string()],
                    ["7".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "is6".to_string(),
                    root_node: parse_expression("is0(x-6)"),
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["7".to_string(), "1".to_string()],
                    ["7.5".to_string(), "1".to_string()],
                    ["8".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "is7".to_string(),
                    root_node: parse_expression("is0(x-7)"),
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["8".to_string(), "1".to_string()],
                    ["8.5".to_string(), "1".to_string()],
                    ["9".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "is8".to_string(),
                    root_node: parse_expression("is0(x-8)"),
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["9".to_string(), "1".to_string()],
                    ["9.5".to_string(), "1".to_string()],
                    ["10".to_string(), "0".to_string()],
                ],
                solution: vec![BinaryAlgebraicExpressionTree {
                    name: "is9".to_string(),
                    root_node: parse_expression("is0(x-9)"),
                }],
            },
            TestCase {
                description: None,
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
                    name: "floor1".to_string(),
                    root_node: parse_expression(
                        "is1(x)+2*is2(x)+3*is3(x)+4*is4(x)+5*is5(x)+6*is6(x)+7*is7(x)+8*is8(x)+9*is9(x)",
                    ),
                }],
            },
            TestCase {
                description: None,
                examples: vec![
                    ["0.06".to_string(), "0.6".to_string()],
                    [
                        "0.12345678".to_string(),
                        "0.2345678".to_string() + &"0".repeat(get_decimal_places() - 8) + "1",
                    ],
                    [
                        "0.7".to_string(),
                        "0.".to_string() + &"0".repeat(get_decimal_places() - 1) + "7",
                    ],
                ],
                solution: vec![
                    BinaryAlgebraicExpressionTree {
                        name: "right".to_string(),
                        root_node: parse_expression("x*10-floor1(x*10)+floor1(x*10)*tiny(x)")
                    },
                ],
            },
            TestCase {
                description: None,
                examples: vec![
                    [
                        "0.2345678".to_string() + &"0".repeat(get_decimal_places() - 8) + "1",
                        "0.12345678".to_string(),
                    ],
                    [
                        "0.".to_string() + &"0".repeat(get_decimal_places() - 1) + "7",
                        "0.7".to_string(),
                    ],
                ],
                solution: vec![
                    BinaryAlgebraicExpressionTree {
                        name: "left".to_string(),
                        root_node: parse_expression(
                            &("right(".repeat(get_decimal_places() - 1) + "(x)" + &")".repeat(get_decimal_places() - 1))
                        )
                    },
                ],
            },
        ]
    })
    }

    #[test]
    fn test_solutions() {
        let tasks = get_test_cases();
        let trees: Vec<BinaryAlgebraicExpressionTree> = tasks
            .iter()
            .flat_map(|task| &task.solution)
            .cloned()
            .collect();
        for i in 0..tasks.len() {
            for [input, output] in &tasks[i].examples {
                let name_function = &tasks[i].solution.last().unwrap().name;
                let result = trim2(
                    apply_algebra_to_tree_node(
                        &tasks[i].solution.last().unwrap().root_node,
                        &input.parse::<Dec>().unwrap(),
                        &trees,
                        true,
                    )
                    .unwrap(),
                );
                assert_eq!(
                    format!("{}({}) = {}", name_function, input, output),
                    format!("{}({}) = {}", name_function, input, result)
                );
            }
        }
    }

    #[test]
    fn test_math_tricks() {
        let tasks = get_test_cases();
        let mut trees: Vec<BinaryAlgebraicExpressionTree> = vec![];
        for task in tasks {
            for tree in &task.solution {
                trees.push(tree.clone());
            }
        }
        for i in 0..tasks.len() {
            for [input, output] in &tasks[i].examples {
                let name_function = &tasks[i].solution.last().unwrap().name;
                let input_dec = input.parse().unwrap();
                let result = match name_function.as_str() {
                    "abs" => math_trick::abs(input_dec),
                    "ge0" => math_trick::ge0(input_dec),
                    "is0" => math_trick::is0(input_dec),
                    "floor1" => math_trick::floor1(input_dec),
                    "left" => math_trick::left(input_dec),
                    _ => continue,
                };
                assert_eq!(
                    format!("{}({}) = {}", name_function, input, output),
                    format!("{}({}) = {}", name_function, input, result)
                );
            }
        }
    }
}
