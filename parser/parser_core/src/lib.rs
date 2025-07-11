use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ScriptParser;

pub mod ast;
pub mod parser;

use parser::parse_pairs;


pub fn parse_script(input: &str) -> Result<(), pest::error::Error<crate::Rule>> {
    let pairs = ScriptParser::parse(Rule::program, input)
        .expect("unsuccessful parse")
        .next().unwrap();

    parse_pairs(pairs);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
