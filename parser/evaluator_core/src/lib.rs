use parser_core::ast::Program;
use parser_core::parse_script;

pub mod evaluator;
pub mod response;

use evaluator::evaluate_input;

pub fn evaluate_script(input: &str) {
    let res = parse_script(input);
    match res {
        Ok(ast) => {
            println!("Parsed successfully!\n{:#?}", ast);
            evaluate_ast(&ast);
        }
        Err(e) => eprintln!("Parse error:\n{}", e),
    }
}

pub fn evaluate_ast(ast: &Program) {
    println!("Evaluating AST: {:#?}", ast);
    evaluate_input(&ast);
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
