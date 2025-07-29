use parser_core::ast::{NamedArg, Value};
use crate::context::EvalContext;

pub fn plot_eval(ctx: &EvalContext, args: &Vec<NamedArg>) {
    // rebase - cut off to the shortest and rebase
    let mut rebase: Option<f64> = None;

    for arg in args {
        match arg.name.as_str() {
            "rebase" => {
                match &arg.value {
                    Value::Number(num) => rebase = Some(*num),
                    _ => panic!("Expected number for rebase.")
                }
            }
            _ => {}
        }
    }

    println!("rebase: {:?}", rebase);
}