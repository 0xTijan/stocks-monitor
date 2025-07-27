use parser_core::ast::*;
use crate::response::{TrackedItem, Item};
use std::collections::HashMap;
use crate::types::{Stock, Index};
use crate::helpers::get_today;


#[derive(Debug)]
struct EvalContext {
    // === Raw Market Data ===
    stocks: HashMap<String, Stock>,          // e.g. "AAPL" → Stock struct
    indexes: HashMap<String, Index>,         // e.g. "S&P500"

    // === Time Series ===
    price_series: HashMap<String, Vec<f64>>, // e.g. "AAPL" → [150.0, 151.2, ...]
    index_series: HashMap<String, Vec<f64>>,

    // === Derived Series ===
    derived_series: HashMap<String, Vec<f64>>, // e.g. "my_custom_avg" → [123.4, ...]

    /*// === Function Registry ===
    functions: HashMap<String, fn(&EvalContext, Vec<String>) -> Vec<f64>>,*/

    // === Metadata / Settings ===
    date_range: (String, String),
    tracked_items: Vec<TrackedItem>,
}


pub fn evaluate_input(program: &Program) {
    let mut isFirst = true;
    let mut context = EvalContext {
        stocks: HashMap::new(),
        indexes: HashMap::new(),
        price_series: HashMap::new(),
        index_series: HashMap::new(),
        derived_series: HashMap::new(),
        // functions: HashMap::new(),
        date_range: ("2019-01-01".to_string(), get_today()),
        tracked_items: Vec::new(),
    };

    for command in program.commands.iter() {
        match command {
            Command::Filter(args) => evaluate_filter(&mut context, args, isFirst),
            Command::Sort(args) => evaluate_sort(&mut context, args, isFirst),
            Command::Backtest(args) => evaluate_backtest(&mut context, args, isFirst),
            Command::Plot(args) => evaluate_plot(&mut context, args, isFirst),
        }
        isFirst = false;
    }
}

fn evaluate_plot(ctx: &mut EvalContext, args: &Vec<NamedArg>, isFirst: bool) {
    // Implement plot evaluation logic here
    if isFirst {
        evaluate_first(ctx, args);
    }
}

fn evaluate_filter(ctx: &mut EvalContext, args: &Vec<NamedArg>, isFirst: bool) {
    // Implement filter evaluation logic here
    if isFirst {
        evaluate_first(ctx, args);
    }
} 

fn evaluate_sort(ctx: &mut EvalContext, args: &Vec<NamedArg>, isFirst: bool) {
    // Implement sort evaluation logic here
    if isFirst {
        evaluate_first(ctx, args);
    }
}

fn evaluate_backtest(ctx: &mut EvalContext, args: &Vec<NamedArg>, isFirst: bool) {
    // Implement backtest evaluation logic here
    if isFirst {
        evaluate_first(ctx, args);
    }
}

fn evaluate_first(ctx: &mut EvalContext, args: &Vec<NamedArg>) {
    for arg in args {
        println!("Evaluating argument: {:?}", arg);
        match arg.name.as_str() {
            "from" => {
                if let Value::Date(date_str) = &arg.value {
                    ctx.date_range.0 = date_str.clone();
                } else {
                    panic!("Expected a Date for 'from', got {:?}", arg.value);
                }
            }
            "to" => {
                if let Value::Date(date_str) = &arg.value {
                    ctx.date_range.1 = date_str.clone();
                } else if let Value::Keyword(keyword) = &arg.value {
                    match keyword {
                        Keyword::Today => {
                            let today = get_today();
                            ctx.date_range.1 = today;
                        }
                        _ => panic!("Expected 'today' or a Date for 'to', got {:?}", arg.value),
                    }
                } else {
                    panic!("Expected a Date for 'to', got {:?}", arg.value);
                }
            }
            "items" => {
                match &arg.value {
                    Value::List(items) => {
                        for item in items {
                            match item {
                                Value::FunctionCall(func_call) => {
                                    // Handle function calls like RSI, MACD, etc.
                                    println!("Function call: {:?}", func_call);
                                    // Here you would evaluate the function call
                                }
                                _ => println!("Unhandled item in 'items': {:?}", item),
                            }
                        }
                    }
                    _ => panic!("Expected a List for 'items', got {:?}", arg.value),
                }
            }
            &_ => {
                // Handle other arguments as needed
                println!("Unhandled argument: {:?}", arg);
            }
        }
    }
}

fn evaluate_function_call(ctx: &mut EvalContext, func_call: &FunctionCall) {
    let name = &func_call.name;
    let args = &func_call.args;

    match name.as_str() {
        "RSI" => {
            // Implement RSI calculation logic here
            println!("Calculating RSI with args: {:?}", args);
        }
        "MACD" => {
            // Implement MACD calculation logic here
            println!("Calculating MACD with args: {:?}", args);
        }
        _ => {
            eprintln!("Unknown function call: {}", name);
        }
    }
}

fn get_item_prices(ctx: &EvalContext, item_id: &str) -> Option<Vec<f64>> {
    if let Some(ctx_stock_prices) = ctx.price_series.get(item_id) {
        return Some(ctx_stock_prices.clone());
    }
    if let Some(ctx_index_prices) = ctx.index_series.get(item_id) {
        return Some(ctx_index_prices.clone());
    }
    None

    // get id from symbol
    // fetch prices and item info and store in context
    // return the prices
}

fn get_item_data(ctx: &EvalContext, item_id: &str) -> Option<Item> {
    if let Some(stock) = ctx.stocks.get(item_id) {
        return Some(Item::Stock(stock.clone()));
    }
    if let Some(index) = ctx.indexes.get(item_id) {
        return Some(Item::Index(index.clone()));
    }
    None
       
    // get id from symbol
    // fetch prices and item info and store in context
    // return the data
}