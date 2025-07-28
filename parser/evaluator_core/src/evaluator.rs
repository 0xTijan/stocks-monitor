use parser_core::ast::*;
use crate::response::{TrackedItem, Item, ItemType};
use std::collections::HashMap;
use crate::types::{Stock, Index};
use crate::helpers::{get_today, all_stocks_symbols, all_indexes_symbols, expr_to_id};
use chrono::NaiveDate;
use crate::context::*;
use std::pin::Pin;
use std::future::Future;
use crate::functions::handle_calculate_function;


/// Entry point: make this async to support async calls inside
pub async fn evaluate_input(program: &Program) {
    let mut is_first = true;
    let mut context = EvalContext {
        stocks: HashMap::new(),
        indexes: HashMap::new(),
        price_series: HashMap::new(),
        index_series: HashMap::new(),
        derived_series: HashMap::new(),
        date_range: ("2019-01-01".to_string(), get_today()),
        tracked_items: Vec::new(),
    };

    for command in &program.commands {
        match command {
            Command::Filter(args) => evaluate_filter(&mut context, args, is_first).await,
            Command::Sort(args) => evaluate_sort(&mut context, args, is_first).await,
            Command::Backtest(args) => evaluate_backtest(&mut context, args, is_first).await,
            Command::Plot(args) => evaluate_plot(&mut context, args, is_first).await,
        }
        is_first = false;
    }

    println!("Final tracked items: {:#?}", context);
}

// All these need to be async so they can await `evaluate_first`
async fn evaluate_plot(ctx: &mut EvalContext, args: &Vec<NamedArg>, is_first: bool) {
    if is_first {
        evaluate_first(ctx, args).await;
    }
}

async fn evaluate_filter(ctx: &mut EvalContext, args: &Vec<NamedArg>, is_first: bool) {
    if is_first {
        evaluate_first(ctx, args).await;
    }
}

async fn evaluate_sort(ctx: &mut EvalContext, args: &Vec<NamedArg>, is_first: bool) {
    if is_first {
        evaluate_first(ctx, args).await;
    }
}

async fn evaluate_backtest(ctx: &mut EvalContext, args: &Vec<NamedArg>, is_first: bool) {
    if is_first {
        evaluate_first(ctx, args).await;
    }
}

async fn evaluate_date_range(ctx: &mut EvalContext, args: &Vec<NamedArg>) {
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
                            ctx.date_range.1 = get_today();
                        }
                        _ => panic!("Expected 'today' or a Date for 'to', got {:?}", arg.value),
                    }
                } else {
                    panic!("Expected a Date for 'to', got {:?}", arg.value);
                }
            }
            _ => {}
        }
    }
}

async fn evaluate_first(ctx: &mut EvalContext, args: &Vec<NamedArg>) {
    evaluate_date_range(ctx, args).await;
    for arg in args {
        println!("Evaluating argument: {:?}", arg);
        match arg.name.as_str() {
            "items" => {
                match &arg.value {
                    Value::List(items) => {
                        for item in items {
                            match item {
                                Value::FunctionCall(func_call) => {
                                    println!("Function call: {:?}", func_call);
                                    // async evaluate function call if needed:
                                    evaluate_function_call(ctx, func_call).await;
                                }
                                Value::ArithmeticExpr(expr) => {
                                    // Await async computation of expression series
                                    let series = compute_expr_series(ctx, expr).await;
                                    let id = expr_to_id(expr);
                                    ctx.tracked_items.push(TrackedItem {
                                        id: id.clone(),
                                        item_type: ItemType::Derived,
                                    });
                                    ctx.derived_series.insert(id, series);
                                }
                                Value::Ident(symbol) => {
                                    println!("Ident found: {}", symbol);
                                    match symbol.as_str() {
                                        "stocks" => {
                                            let stocks = all_stocks_symbols();
                                            for s in stocks {
                                                ctx.tracked_items.push(TrackedItem {
                                                    id: s.to_string(),
                                                    item_type: ItemType::Stock,
                                                });
                                            }
                                        },
                                        "indexes" => {
                                            let indexes = all_indexes_symbols();
                                            for s in indexes {
                                                ctx.tracked_items.push(TrackedItem {
                                                    id: s.to_string(),
                                                    item_type: ItemType::Index,
                                                });
                                            }
                                        },
                                        _ => {
                                            ctx.tracked_items.push(TrackedItem {
                                                id: symbol.to_string(),
                                                item_type: ItemType::Derived,
                                            });
                                        }
                                    }
                                }
                                _ => {},
                            }
                        }
                    }
                    _ => panic!("Expected a List for 'items', got {:?}", arg.value),
                }
            }
            _ => {
                println!("Unhandled argument: {:?}", arg);
            }
        }
    }
}

// Make evaluate_function_call async so it can await inside if needed
async fn evaluate_function_call(ctx: &mut EvalContext, func_call: &FunctionCall) {
    let name = &func_call.name;
    let args = &func_call.args;
    
    handle_calculate_function(ctx, args, &name).await;
}

fn compute_expr_series<'a>(
    ctx: &'a mut EvalContext,
    expr: &'a Expr,
) -> Pin<Box<dyn Future<Output = Vec<f64>> + 'a>> {
    Box::pin(async move {
        match expr {
            Expr::Number(val) => {
                vec![*val; ctx.date_range_len()]
            }
            Expr::Ident(symbol) => {
                if let Some(series) = ctx.get_item_prices(symbol).await {
                    series.clone()
                } else {
                    panic!("No series found for symbol {}", symbol);
                }
            }
            Expr::FunctionCall(func_call) => {
                evaluate_function_call(ctx, func_call).await;
                vec![]
            }
            Expr::BinaryOp { left, op, right } => {
                let left_series = compute_expr_series(ctx, left).await;
                let right_series = compute_expr_series(ctx, right).await;
                apply_arithmetic_op(&left_series, &right_series, op)
            }
            Expr::Group(inner) => compute_expr_series(ctx, inner).await,
            Expr::Tuple(_) => {
                panic!("Tuples are not directly evaluable as numeric series");
            }
        }
    })
}

fn apply_arithmetic_op(left: &Vec<f64>, right: &Vec<f64>, op: &ArithmeticOp) -> Vec<f64> {
    let left_len = left.len();
    let right_len = right.len();
    let len = left_len.min(right_len);

    let mut result = Vec::with_capacity(len);

    for i in 0..len {
        // Calculate indices aligned to the newest data (end of arrays)
        let left_idx = left_len - len + i;
        let right_idx = right_len - len + i;

        let v = match op {
            ArithmeticOp::Add => left[left_idx] + right[right_idx],
            ArithmeticOp::Sub => left[left_idx] - right[right_idx],
            ArithmeticOp::Div => {
                if right[right_idx] != 0.0 {
                    left[left_idx] / right[right_idx]
                } else {
                    f64::NAN
                }
            }
        };
        result.push(v);
    }

    result
}
