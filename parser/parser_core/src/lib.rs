use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ScriptParser;

pub mod ast;
pub mod parser;

use ast::Program;
use parser::parse_pairs;


pub fn parse_script(input: &str) -> Result<(), pest::error::Error<crate::Rule>> {
    let mut program: Program = Program{
        commands: Vec::new()
    };

    let commands = input.split('&')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    for command in commands {
        if command.contains("(") {
            let pairs = ScriptParser::parse(Rule::program, command)
                .expect("unsuccessful parse")
                .next().unwrap();
        
            parse_pairs(pairs);
            // push res to program.commands
        } else {
            // it is just a command without args
            println!("Command without args: {}", command);
        }
    }
    
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
