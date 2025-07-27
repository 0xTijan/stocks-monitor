use std::env;
use std::fs;
use evaluator_core::evaluate_script;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let input = if let Some(arg) = args.first() {
        if arg.ends_with(".txt") {
            match fs::read_to_string("examples/".to_owned() + arg) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Failed to read file '{}': {}", arg, e);
                    return;
                }
            }
        } else {
            arg.clone()
        }
    } else {
        // Default command if no args
        "PLOT(items=[(SBITOP / CBX) , RSI(14, SPY)], from=2025-07-01, to=today)".to_string()
    };

    evaluate_script(&input);
}
