use std::pin::Pin;
use parser_core::ast::{Comparator, LogicalExpr, LogicalOp, NamedArg, Operand, Value};
use crate::{context::EvalContext, evaluator::evaluate_function_call, response_types::{Item, TrackedItem}};


pub async fn filter_eval(ctx: &mut EvalContext, args: &Vec<NamedArg>) {
    let mut conditions: Option<LogicalExpr> = None;

    for arg in args {
        match arg.name.as_str() {
            "conditions" => {
                match &arg.value {
                    Value::LogicalExpr(expr) => conditions = Some(expr.clone()),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    if let Some(expr) = conditions {
        // evaluate logical expression
        // filter tracked items based on the expression
        let tracked_items = ctx.tracked_items.clone();
        for tracked_item in tracked_items {
            let eval = evaluate_condition(ctx, &expr, &tracked_item).await;
            if !eval {
                // remove item from tracked if it does not match the condition
                ctx.tracked_items.retain(|item| item.id != tracked_item.id);
            }
        }
    } else {
        panic!("No filter conditions provided");
    }
}

pub fn evaluate_condition<'a>(
    ctx: &'a mut EvalContext,
    condition: &'a LogicalExpr,
    item: &'a TrackedItem,
) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
    Box::pin(async move {
        match condition {
            LogicalExpr::BinaryOp { left, op, right } => {
                let left_val = evaluate_condition(ctx, left, item).await;
                let right_val = evaluate_condition(ctx, right, item).await;
                match op {
                    LogicalOp::And => left_val && right_val,
                    LogicalOp::Or => left_val || right_val,
                }
            }
            LogicalExpr::Comparison { left, op, right } => {
                let l = evaluate_operand(ctx, left, item).await;
                let r = evaluate_operand(ctx, right, item).await;
                match op {
                    Comparator::Eq => (l - r).abs() < f64::EPSILON,
                    Comparator::Neq => (l - r).abs() >= f64::EPSILON,
                    Comparator::Gt => l > r,
                    Comparator::Lt => l < r,
                    Comparator::Gte => l >= r,
                    Comparator::Lte => l <= r,
                }
            }
            LogicalExpr::Group(inner) => evaluate_condition(ctx, inner, item).await,
        }
    })
}

async fn evaluate_operand(ctx: &mut EvalContext, operand: &Operand, item: &TrackedItem) -> f64 {
    match operand {
        Operand::Number(num) => *num,
        
        Operand::Ident(ident) => {
            let item_data = ctx.get_item_data(&item.id);
            match item_data {
                Some(data) => match ident.as_str() {
                    "price" => {
                        match data {
                            Item::Stock(stock) => stock.last_price.unwrap_or(0.0),
                            Item::Index(index) => index.last_value.unwrap_or(0.0),
                        }
                    },
                    "change" => {
                        match data {
                            Item::Stock(stock) => stock.change_prev_close_percentage.unwrap_or(0.0),
                            Item::Index(index) => index.change_prev_close_percentage.unwrap_or(0.0),
                        }
                    },
                    "country" => {
                        match data {
                            Item::Stock(stock) => {
                                if stock.mic == "XZAG" { 0.0 } else { 1.0 }
                            }
                            Item::Index(index) => {
                                if index.mic == "XZAG" { 0.0 } else { 1.0 }
                            }
                        }
                    },
                    "market_cap" => {
                        match data {
                            Item::Stock(stock) => stock.quantity.unwrap_or(0) as f64 * stock.last_price.unwrap_or(0.0),
                            Item::Index(_) => 0.0,
                        }
                    }
                    // other side identifiers
                    "si" => 1.0,
                    "hr" => 0.0,
                    _ => {
                        if let Ok(num) = ident.parse::<f64>() {
                            num
                        } else {
                            panic!("Unknown identifier in filter condition: {}", ident);
                        }
                    },
                },
                None => panic!("No data found for item {}", item.id),
            }
        },
        
        Operand::FunctionCall(func_call) => {
            let res_id = evaluate_function_call(ctx, func_call).await;
            let id = res_id + &item.id;
            if let Some(series) = ctx.derived_series.get(&id) {
                if series.is_empty() {
                    0.0
                } else {
                    series[series.len() - 1]
                }
            } else {
                panic!("No series found for function call {}", id);
            }
        },

        Operand::LogicalExpr(expr) => {
            let res = evaluate_condition(ctx, expr, item).await;
            if res {
                1.0
            } else {
                0.0
            }
        },
    }
}