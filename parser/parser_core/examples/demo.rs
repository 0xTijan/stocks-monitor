use std::env;
use std::fs;
use parser_core::parse_script;

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
        "CHART(items=[(SBITOP / 2) , RSI(14, (ZVTG / POSR))], from=2015-01-01, to=today)".to_string()
    };

    match parse_script(&input) {
        Ok(ast) => {
            println!("Parsed successfully!\n{:#?}", ast);
        }
        Err(e) => eprintln!("Parse error:\n{}", e),
    }
}
