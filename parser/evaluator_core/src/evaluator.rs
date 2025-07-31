use parser_core::ast::*;
use crate::eval_filter::filter_eval;
use crate::eval_plot::plot_eval;
use crate::response_types::{ItemType, Response, TrackedItem};
use crate::helpers::{create_function_id, expr_to_id, get_today, number_series_with_dates};
use crate::context::*;
use std::collections::HashMap;
use std::pin::Pin;
use std::future::Future;
use crate::functions::handle_calculate_function;
use crate::eval_sort::sort_eval;


pub async fn evaluate_input(program: &Program) -> Response {
    let mut is_first = true;
    let mut context = EvalContext::init();
    let mut has_plot = false;
    let mut has_backtest = false;

    // set date range first
    for command in &program.commands {
        match command {
            Command::Plot(args) => {
                evaluate_date_range(&mut context, args);
                has_plot = true;
            },
            Command::Backtest(_) => has_backtest = true,
            _ => {}
        }
    }

    for command in &program.commands {
        match command {
            Command::Filter(args) => evaluate_filter(&mut context, args, is_first).await,
            Command::Sort(args) => evaluate_sort(&mut context, args, is_first).await,
            Command::Backtest(args) => evaluate_backtest(&mut context, args, is_first).await,
            Command::Plot(args) => evaluate_plot(&mut context, args, is_first).await,
            Command::Group(args) => evaluate_group(&mut context, args, is_first).await,
        }
        is_first = false;
    }

    //println!("Final tracked items: {:#?}", context);
    println!("has plot {:?}", has_plot);
    context.create_response(has_plot, has_backtest)
}

async fn evaluate_group(ctx: &mut EvalContext, args: &Vec<NamedArg>, is_first: bool) {
    if is_first {
        evaluate_first(ctx, args).await;
    }
}

async fn evaluate_plot(ctx: &mut EvalContext, args: &Vec<NamedArg>, is_first: bool) {
    if is_first {
        evaluate_first(ctx, args).await;
    }
    plot_eval(ctx, args);
}

async fn evaluate_filter(ctx: &mut EvalContext, args: &Vec<NamedArg>, is_first: bool) {
    if is_first {
        evaluate_first(ctx, args).await;
    }
    filter_eval(ctx, args).await;
}

async fn evaluate_sort(ctx: &mut EvalContext, args: &Vec<NamedArg>, is_first: bool) {
    if is_first {
        evaluate_first(ctx, args).await;
    }
    sort_eval(ctx, args).await;
}

async fn evaluate_backtest(ctx: &mut EvalContext, args: &Vec<NamedArg>, is_first: bool) {
    if is_first {
        evaluate_first(ctx, args).await;
    }
}

fn evaluate_date_range(ctx: &mut EvalContext, args: &Vec<NamedArg>) {
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
    evaluate_date_range(ctx, args);
    for arg in args {
        println!("Evaluating argument: {:?}", arg);
        match arg.name.as_str() {
            "items" => {
                match &arg.value {
                    Value::List(items) => {
                        for item in items {
                            match item {
                                Value::FunctionCall(func_call) => {
                                    evaluate_function_call(ctx, func_call).await;
                                }
                                Value::ArithmeticExpr(expr) => {
                                    // Await async computation of expression series
                                    let series = compute_expr_series(ctx, expr, None).await;
                                    let id = expr_to_id(expr, None);
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
                                            ctx.add_all_stocks_to_tracked().await;
                                        },
                                        "indexes" => {
                                            ctx.add_all_indexes_to_tracked().await;
                                        },
                                        "all" => {
                                            ctx.add_all_stocks_to_tracked().await;
                                            ctx.add_all_indexes_to_tracked().await;
                                        },
                                        _ => {
                                            ctx.get_item_prices(symbol, true).await;
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

// by default it checks if function has an argument for item, if not it will calculate the function for all tracked items and return id (RSI_14_)
pub async fn evaluate_function_call(ctx: &mut EvalContext, func_call: &FunctionCall) -> String {
    let name = &func_call.name;
    let args = &func_call.args;
    
    handle_calculate_function(ctx, args, &name).await;

    let args_item: Vec<String> = args
        .iter()
        .filter_map(|arg| {
            if let FunctionArg::Ident(item_id) = arg {
                Some(item_id.to_string())
            } else {
                None
            }
        })
        .collect();

    create_function_id(name, args, &args_item.first().unwrap_or(&"".to_string()))
}

pub fn compute_expr_series<'a>(
    ctx: &'a mut EvalContext,
    expr: &'a Expr,
    tracked_item: Option<&'a TrackedItem>,
) -> Pin<Box<dyn Future<Output = Vec<(String, f64)>> + 'a>> {
    Box::pin(async move {
        match expr {
            Expr::Number(val) => {
                let from = &ctx.date_range.0;
                let to = &ctx.date_range.1;
                number_series_with_dates(from, to, *val)
            }
            Expr::Ident(symbol) => {
                if let Some(series) = ctx.get_item_prices(symbol, false).await {
                    series.clone()
                } else {
                    panic!("No series found for symbol {}", symbol);
                }
            }
            Expr::FunctionCall(func_call) => {
                let id_res = evaluate_function_call(ctx, func_call).await;
                let id = if let Some(tracked_item) = tracked_item {
                    id_res + &tracked_item.id
                } else {
                    id_res
                };
                if let Some(series) = ctx.derived_series.get(&id) {
                    series.clone()
                } else {
                    panic!("No series found for function call {}", id);
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let left_series = compute_expr_series(ctx, left, tracked_item).await;
                let right_series = compute_expr_series(ctx, right, tracked_item).await;
                apply_arithmetic_op(&left_series, &right_series, op)
            }
            Expr::Group(inner) => compute_expr_series(ctx, inner, tracked_item).await,
            Expr::Tuple(_) => {
                panic!("Tuples are not directly evaluable as numeric series");
            }
        }
    })
}

pub fn apply_arithmetic_op(
    left: &Vec<(String, f64)>,
    right: &Vec<(String, f64)>,
    op: &ArithmeticOp,
) -> Vec<(String, f64)> {
    let map_left: HashMap<&String, f64> = left.iter().map(|(d, v)| (d, *v)).collect();
    let map_right: HashMap<&String, f64> = right.iter().map(|(d, v)| (d, *v)).collect();

    let mut result = Vec::new();

    for date in map_left.keys() {
        if let Some(val_right) = map_right.get(date) {
            let val_left = map_left.get(date).unwrap();
            let combined = match op {
                ArithmeticOp::Add => val_left + val_right,
                ArithmeticOp::Sub => val_left - val_right,
                ArithmeticOp::Div => {
                    if *val_right != 0.0 {
                        val_left / val_right
                    } else {
                        f64::NAN
                    }
                }
            };
            result.push(((*date).clone(), combined));
        }
    }

    result.sort_by(|a, b| a.0.cmp(&b.0)); // Optional: sort by date
    result
}
