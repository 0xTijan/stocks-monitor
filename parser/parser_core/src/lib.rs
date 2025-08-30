use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ScriptParser;

pub mod ast;
pub mod parser;

use ast::{Program, Command};
use parser::parse_pairs;


pub fn parse_script(input: &str) -> Result<Program, &'static str> {
    println!("Parsing script: {}", input);
    
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
                .next();

            match pairs {
                Some(pairs) => {
                    if pairs.as_rule() != Rule::program {
                        return Err("Expected a program rule");
                    }
                    let res = parse_pairs(pairs);
                    program.commands.push(res.commands[0].clone());
                },
                None => return Err("No pairs found"),
            }
        } else {
            // it is just a command without args
            match command {
                "FILTER" => program.commands.push(Command::Filter(Vec::new())),
                "SORT" => program.commands.push(Command::Sort(Vec::new())),
                "CHART" => program.commands.push(Command::Plot(Vec::new())),
                "BACKTEST" => program.commands.push(Command::Backtest(Vec::new())),
                _ => panic!("Unknown command: {}", command),
            }
        }
    }
    
    Ok(program)
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
