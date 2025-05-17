use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ScriptParser;

pub fn parse_script(input: &str) -> Result<(), pest::error::Error<crate::Rule>> {
    let pairs = ScriptParser::parse(Rule::program, input)?;
    for pair in pairs {
        println!("{:#?}", pair);
    }
    Ok(())
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
