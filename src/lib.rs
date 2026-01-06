use std::cell::RefCell;
use std::io::stdin;
use std::rc::Rc;

use crate::{
    expr::Expr,
    function::{Function, FunctionTable},
    tokenlizer::Tokenlizer,
};

pub mod expr;
pub mod function;
pub mod tokenlizer;

#[test]
fn test() {
    let function_table = Rc::new(RefCell::new(FunctionTable::new()));
    let text = "f(x)=log(2,x)+3^(2*x+1)".to_string();

    let mut tokenlizer = Tokenlizer::new(&text);

    let tokens = match tokenlizer.tokenlize() {
        Ok(res) => res,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    #[cfg(debug_assertions)]
    {
        for i in 0..tokens.len() {
            println!("{}", tokens[i])
        }
    }

    match Function::new(&tokens, function_table.clone()) {
        Ok(res) => res,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let input = "f(1)".to_string();
    let mut vec: Vec<&str> = input.trim().split(|c| c == '(' || c == ')').collect();
    let name = vec[0];

    vec = vec[1].split(|c| c == ',' || c == ' ').collect();

    let binding = function_table.borrow();
    let func = binding.find(name, vec.len());
    if func == None {
        println!("no such function: {}", name);
        return;
    } else {
        if let Some(x) = func {
            let mut args = Vec::<f64>::new();
            for i in 0..vec.len() {
                args.push(vec[i].parse::<f64>().unwrap());
            }

            let ans = match x.caculate(&args, function_table.clone()) {
                Ok(v) => v,
                Err(e) => {
                    println!("error: {}", e);
                    return;
                }
            };
            println!("{}={}", input.trim(), ans);
            println!(
                "{}",
                x.derivative(&"x".to_string(), function_table.clone())
                    .unwrap()
            );
        }
    }
}

pub fn put(function_table: Rc<RefCell<FunctionTable>>) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let text = input.trim().to_string();

    let mut tokenlizer = Tokenlizer::new(&text);

    let tokens = match tokenlizer.tokenlize() {
        Ok(res) => res,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    #[cfg(debug_assertions)]
    {
        for i in 0..tokens.len() {
            println!("{}", tokens[i])
        }
    }

    match Function::new(&tokens, function_table.clone()) {
        Ok(res) => res,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}

pub fn caculate(function_table: Rc<RefCell<FunctionTable>>) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut vec: Vec<&str> = input.trim().split(|c| c == '(' || c == ')').collect();
    let name = vec[0];

    vec = vec[1].split(|c| c == ',' || c == ' ').collect();

    let binding = function_table.borrow();
    let func = binding.find(name, vec.len());
    if func == None {
        println!("no such function: {}", name);
        return;
    } else {
        if let Some(x) = func {
            let mut args = Vec::<f64>::new();
            for i in 0..vec.len() {
                args.push(vec[i].parse::<f64>().unwrap());
            }

            let ans = match x.caculate(&args, function_table.clone()) {
                Ok(v) => v,
                Err(e) => {
                    println!("error: {}", e);
                    return;
                }
            };
            println!("{}={}", input.trim(), ans);
        }
    }
}

pub fn derivative(function_table: Rc<RefCell<FunctionTable>>) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut vec: Vec<&str> = input.trim().split(|c| c == '(' || c == ')').collect();
    let name = vec[0];

    vec = vec[1].split(|c| c == ',' || c == ' ').collect();

    let mut binding = function_table.borrow_mut();
    let func = binding.find(name, vec.len());
    if func == None {
        println!("no such function: {}", name);
        return;
    } else {
        if let Some(x) = func {
            input = String::new();
            stdin().read_line(&mut input).unwrap();

            let dx = input;

            let _f = match x.derivative(&dx, function_table.clone()) {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };
            if let Expr::Func(ref name, _) = _f.symble {
                binding.insert(name.to_string(), _f.clone());
            }
            println!("{}", _f);
        }
    }
}
