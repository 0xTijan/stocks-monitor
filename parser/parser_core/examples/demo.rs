use parser_core::parse_script;

fn main() {
    let input = "PLOT(items=[(TRGV / PZVS)], from=2015-01-01, to=today)";

    match parse_script(input) {
        Ok(_) => println!("Parsed successfully!"),
        Err(e) => eprintln!("Parse error:\n{}", e),
    }
}