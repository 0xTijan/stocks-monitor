use parser_core::ast::Program;
use parser_core::parse_script;

pub mod evaluator;
pub mod response_types;
pub mod types;
pub mod helpers;
pub mod context;
pub mod apis;
pub mod functions;
pub mod eval_sort;
pub mod eval_filter;
pub mod eval_plot;
use crate::response_types::Response;
use evaluator::evaluate_input;


pub async fn evaluate_script(input: &str) -> Option<Response> {
    let res = parse_script(input);
    match res {
        Ok(ast) => {
            println!("Parsed successfully!\n{:#?}", ast);
            let r = evaluate_ast(&ast).await;
            return Some(r);
        }
        Err(e) => {
            eprintln!("Parse error:\n{}", e);
            return None;
        }
    }
}

pub async fn evaluate_ast(ast: &Program) -> Response {
    println!("Evaluating AST: {:#?}", ast);
    let res = evaluate_input(&ast).await;
    println!("Final response: {:#?}", res);
    res
}