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
use console_error_panic_hook;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_func() -> String {
    "Hello from WASM".to_string()
}

#[wasm_bindgen]
pub async fn evaluate_script_wasm(input: &str) -> Option<JsValue> {
    console_error_panic_hook::set_once();
    let res = parse_script(input);
    match res {
        Ok(ast) => {
            println!("Parsed successfully!\n{:#?}", ast);
            let response = evaluate_ast(&ast).await;
            println!("response {:?}", response);
            // serialize Response into JsValue
            serde_wasm_bindgen::to_value(&response).ok()
        }
        Err(e) => {
            eprintln!("Parse error:\n{}", e);
            None
        }
    }
}

pub async fn evaluate_script(input: &str) -> Option<Response> {
    let res = parse_script(input);
    match res {
        Ok(ast) => {
            println!("Parsed successfully!\n{:#?}", ast);
            let response = evaluate_ast(&ast).await;
            println!("response {:?}", response);
            Some(response)
        }
        Err(e) => {
            eprintln!("Parse error:\n{}", e);
            None
        }
    }
}

pub async fn evaluate_ast(ast: &Program) -> Response {
    println!("Evaluating AST: {:#?}", ast);
    let res = evaluate_input(&ast).await;
    println!("Final response: {:#?}", res);
    res
}