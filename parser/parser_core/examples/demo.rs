use std::env;
use parser_core::parse_script;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let input = if let Some(arg) = args.first() {
        arg.as_str()
    } else {
        "PLOT(items=[(TRGV / PZVS)], from=2015-01-01, to=today)"
    };

    match parse_script(input) {
        Ok(_) => println!("Parsed successfully!"),
        Err(e) => eprintln!("Parse error:\n{}", e),
    }
}
