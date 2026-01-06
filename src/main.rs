use std::{cell::RefCell, io::stdin, rc::Rc};

use calculus::function::FunctionTable;

fn main() {
    let function_table = Rc::new(RefCell::new(FunctionTable::new()));
    let mut input;

    loop{
        input=String::new();
        stdin().read_line(& mut input).unwrap();
        if input.trim()=="put"{
            calculus::put(function_table.clone());
        }else if input.trim()=="caculate"{
            calculus::caculate(function_table.clone());
        }else if input.trim()=="stop"{
            break;
        }else{
            println!("unknown command");
        }
    }
}
//cargo build --target x86_64-pc-windows-gnu --release 