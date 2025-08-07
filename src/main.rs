use single_variable_algebra_compiler::*;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
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
                actual_tree: parse_expression(expr),
            };
            trees.push(tree);
        } else {
            println!("Invalid function definition: {arg}");
            return;
        }
    }
    if let Some((func_name, input_val)) = parse_function_call(input.last().unwrap()) {
        if let Some(tree) = trees.iter().find(|t| t.name == func_name) {
            let x: Dec = input_val.parse().unwrap_or_else(|_| {
                println!("Invalid input value: {input_val}");
                std::process::exit(1);
            });
            let result = apply_algebra_to_tree_node(&tree.actual_tree, &x, &trees);
            println!("{}", trim2(result));
        } else {
            println!("Function {func_name} not defined");
        }
    } else {
        println!("Invalid function call: {}", input.last().unwrap());
    }
}

fn parse_function(s: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None;
    }
    let func_part = parts[0].trim();
    if !func_part.ends_with(')') || !func_part.contains('(') {
        return None;
    }
    let name = func_part.split('(').next()?;
    Some((name, parts[1].trim()))
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
