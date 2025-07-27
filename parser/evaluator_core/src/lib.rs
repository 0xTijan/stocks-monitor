use parser_core::ast::Program;
use parser_core::parse_script;
use env_logger;

pub mod evaluator;
pub mod response;
pub mod types;
pub mod helpers;
pub mod context;
pub mod apis;

use evaluator::evaluate_input;

#[tokio::main]
pub async fn evaluate_script(input: &str) {
    let res = parse_script(input);
    match res {
        Ok(ast) => {
            println!("Parsed successfully!\n{:#?}", ast);
            evaluate_ast(&ast).await;
        }
        Err(e) => eprintln!("Parse error:\n{}", e),
    }
}

pub async fn evaluate_ast(ast: &Program) {
    println!("Evaluating AST: {:#?}", ast);
    evaluate_input(&ast).await;
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
