use parser_core::ast::Program;
use parser_core::parse_script;

pub mod evaluator;
pub mod response;
pub mod types;
pub mod helpers;
pub mod context;
pub mod apis;
pub mod functions;
pub mod eval_sort;
pub mod eval_filter;

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