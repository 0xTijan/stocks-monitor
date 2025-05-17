use parser_core::parse_script;

fn main() {
    let input = "FILTER(items=[stocks], conditions=[(MA(36) > MA(58) AND market_cap <= 50000000)])
            & SORT(item=RSI(14), dir=asc, limit=10)
            & PLOT(from=2019-01-01, to=today)";

    match parse_script(input) {
        Ok(_) => println!("Parsed successfully!"),
        Err(e) => eprintln!("Parse error:\n{}", e),
    }
}