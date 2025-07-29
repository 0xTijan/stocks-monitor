use std::collections::HashMap;
use parser_core::ast::{NamedArg, Value};
use crate::{context::EvalContext, evaluator::{compute_expr_series, evaluate_function_call}, helpers::expr_to_id, response::Item, types::{Direction}};

pub async  fn sort_eval(ctx: &mut EvalContext, args: &Vec<NamedArg>) {
    let mut direction: Direction = Direction::Asc;
    let mut limit: Option<u64> = None;
    let mut field: Option<String> = None;
    let mut expr_id: Option<Vec<String>> = None;
    let mut func_id: Option<String> = None;

    // id to derived_id (KRKG -> RSI_14_KRKG+RSI_10_KRKG)
    let mut id_hash_map: HashMap<String, String> = HashMap::new(); 

    for arg in args {
        match arg.name.as_str() {
            "item" => {
                match &arg.value {
                    Value::Ident(f) => {
                        match f.as_str() {
                            "market_cap" => field = Some("market_cap".to_string()),
                            "price" => field = Some("price".to_string()),
                            "quantity" => field = Some("quantity".to_string()),
                            "sector_id" => field = Some("sector_id".to_string()),
                            "change_prev_close_percentage" => field = Some("change_prev_close_percentage".to_string()),
                            "mic" => field = Some("mic".to_string()),
                            "country" => field = Some("country".to_string()),
                            "symbol" => field = Some("symbol".to_string()),
                            "name" => field = Some("name".to_string()),
                            "isin" => field = Some("isin".to_string()),
                            _ => panic!("Unknown field for sorting: {}", f),
                        }
                    }
                    Value::FunctionCall(func_call) => {
                        let id = evaluate_function_call(ctx, func_call).await;
                        func_id = Some(id);
                    }
                    Value::ArithmeticExpr(expr) => {
                        // for each tracked item do this
                        let existing_items: Vec<_> = ctx.tracked_items.clone(); 
                        for tracked_item in existing_items {
                            let id = expr_to_id(expr, Some(&tracked_item));
                            let series = compute_expr_series(ctx, expr, Some(&tracked_item)).await;
                            ctx.derived_series.insert(id.clone(), series);
                            expr_id.get_or_insert_with(Vec::new).push(id.clone());
                            id_hash_map.insert(tracked_item.id.clone(), id);
                        }
                    }
                    _ => panic!("Expected an Ident for 'item', got {:?}", arg.value),
                }
            },
            "dir" => {
                if let Value::Ident(dir) = &arg.value {
                    match dir.as_str() {
                        "asc" => direction = Direction::Asc,
                        "desc" => direction = Direction::Desc,
                        _ => panic!("Invalid sort direction: {}", dir),
                    }
                } else {
                    panic!("Expected an Ident for 'dir', got {:?}", arg.value);
                }
            },
            "limit" => {
                if let Value::Number(num) = &arg.value {
                    limit = Some(*num as u64);
                } else {
                    panic!("Expected a Number for 'limit', got {:?}", arg.value);
                }
            }
            _ => println!("Unhandled argument in plot: {:?}", arg),
        }
    }

    // sort tracked items based on the sort criteria
    // if field is Some check stocks/indexes
    // if exprId is Some check derived_series (for each id in tracked items)
    // if funcId is Some check derived_series (for each id in tracked items - append to funcId)
    // rearrange tracked_items based on sorting criteria (and cut if limit)
    let mut sorted_items = ctx.tracked_items.clone();
    println!("Before sorting: {:#?}", sorted_items);
    if field.is_none() && expr_id.is_none() && func_id.is_some() {
        // FUNCTION
        // loop through tracked items, get derived series for each id (func_id + id (RSI_14_ + id)) and compare
        sorted_items.sort_by(|a, b| {
            let a_id = format!("{}{}", func_id.as_deref().unwrap_or(""), a.id);
            let b_id = format!("{}{}", func_id.as_deref().unwrap_or(""), b.id);

            let a_val = ctx.derived_series.get(&a_id)
                .and_then(|series| series.last())
                .cloned()
                .unwrap_or(f64::MIN);

            let b_val = ctx.derived_series.get(&b_id)
                .and_then(|series| series.last())
                .cloned()
                .unwrap_or(f64::MIN);

            let ordering = a_val
                .partial_cmp(&b_val)
                .unwrap_or(std::cmp::Ordering::Equal);

            match direction {
                Direction::Asc => ordering,
                Direction::Desc => ordering.reverse(),
            }
        });
    } else if field.is_none() && expr_id.is_some() && func_id.is_none() {
        // EXPRESSION
        // loop through tracked items, get derived id from each id and get derived series for each derived id and compare
        sorted_items.sort_by(|a, b| {
            let a_val = id_hash_map
                .get(&a.id)
                .and_then(|d_id| ctx.derived_series.get(d_id))
                .and_then(|series| series.last())
                .cloned()
                .unwrap_or(f64::MIN);

            let b_val = id_hash_map
                .get(&b.id)
                .and_then(|d_id| ctx.derived_series.get(d_id))
                .and_then(|series| series.last())
                .cloned()
                .unwrap_or(f64::MIN);

            let ordering = a_val
                .partial_cmp(&b_val)
                .unwrap_or(std::cmp::Ordering::Equal);

            match direction {
                Direction::Asc => ordering,
                Direction::Desc => ordering.reverse(),
            }
        });
    } else if field.is_some() && expr_id.is_none() && func_id.is_none() {
        // FIELD
        sorted_items.sort_by(|a, b| {
            let a_info = ctx.get_item_data(a.id.as_str());
            let b_info = ctx.get_item_data(b.id.as_str());
            
            if a_info.is_none() || b_info.is_none() {
                return std::cmp::Ordering::Equal;
            }
            
            let a_item = a_info.unwrap();
            let b_item = b_info.unwrap();

            let ordering = match field.as_deref() {
                Some("market_cap") => {
                    let a_val = match a_item {
                        Item::Stock(stock) => stock.quantity.unwrap_or(0) as f64 * stock.last_price.unwrap_or(0.0),
                        Item::Index(index) => index.last_value.unwrap_or(0.0),
                    };
                    let b_val = match b_item {
                        Item::Stock(stock) => stock.quantity.unwrap_or(0) as f64 * stock.last_price.unwrap_or(0.0),
                        Item::Index(index) => index.last_value.unwrap_or(0.0),
                    };
                    a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
                }
                Some("price") => {
                    let a_val = match a_item {
                        Item::Stock(stock) => stock.last_price.unwrap_or(0.0),
                        Item::Index(index) => index.last_value.unwrap_or(0.0),
                    };
                    let b_val = match b_item {
                        Item::Stock(stock) => stock.last_price.unwrap_or(0.0),
                        Item::Index(index) => index.last_value.unwrap_or(0.0),
                    };
                    a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
                }
                Some("quantity") => {
                    let a_val = match a_item {
                        Item::Stock(stock) => stock.quantity.unwrap_or(0) as f64,
                        Item::Index(_) => 0.0,
                    };
                    let b_val = match b_item {
                        Item::Stock(stock) => stock.quantity.unwrap_or(0) as f64,
                        Item::Index(_) => 0.0,
                    };
                    a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
                }
                Some("change_prev_close_percentage") => {
                    let a_val = match a_item {
                        Item::Stock(stock) => stock.change_prev_close_percentage.unwrap_or(0.0),
                        Item::Index(index) => index.change_prev_close_percentage.unwrap_or(0.0),
                    };
                    let b_val = match b_item {
                        Item::Stock(stock) => stock.change_prev_close_percentage.unwrap_or(0.0),
                        Item::Index(index) => index.change_prev_close_percentage.unwrap_or(0.0),
                    };
                    a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
                }
                Some("sector_id") => {
                    let a_str = match a_item {
                        Item::Stock(stock) => stock.sector_id.clone().unwrap_or_default(),
                        Item::Index(_) => String::new(),
                    };
                    let b_str = match b_item {
                        Item::Stock(stock) => stock.sector_id.clone().unwrap_or_default(),
                        Item::Index(_) => String::new(),
                    };
                    a_str.cmp(&b_str)
                }
                Some("country") => {
                    let a_str = match a_item {
                        Item::Stock(stock) => stock.isin.chars().nth(0).clone().unwrap_or_default(),
                        Item::Index(index) => index.isin.chars().nth(0).clone().unwrap_or_default(),
                    };
                    let b_str = match b_item {
                        Item::Stock(stock) => stock.isin.chars().nth(0).clone().unwrap_or_default(),
                        Item::Index(index) => index.isin.chars().nth(0).clone().unwrap_or_default(),
                    };
                    a_str.cmp(&b_str)
                }
                Some("mic") => {
                    let a_str = match a_item {
                        Item::Stock(stock) => stock.mic.clone(),
                        Item::Index(index) => index.mic.clone(),
                    };
                    let b_str = match b_item {
                        Item::Stock(stock) => stock.mic.clone(),
                        Item::Index(index) => index.mic.clone(),
                    };
                    a_str.cmp(&b_str)
                }
                Some("symbol") => {
                    let a_str = match a_item {
                        Item::Stock(stock) => stock.symbol.clone(),
                        Item::Index(index) => index.symbol.clone(),
                    };
                    let b_str = match b_item {
                        Item::Stock(stock) => stock.symbol.clone(),
                        Item::Index(index) => index.symbol.clone(),
                    };
                    a_str.cmp(&b_str)
                }
                Some("name") => {
                    let a_str = match a_item {
                        Item::Stock(stock) => stock.name.clone().unwrap_or_default(),
                        Item::Index(index) => index.name.clone().unwrap_or_default(),
                    };
                    let b_str = match b_item {
                        Item::Stock(stock) => stock.name.clone().unwrap_or_default(),
                        Item::Index(index) => index.name.clone().unwrap_or_default(),
                    };
                    a_str.cmp(&b_str)
                }
                Some("isin") => {
                    let a_str = match a_item {
                        Item::Stock(stock) => stock.isin.clone(),
                        Item::Index(index) => index.isin.clone(),
                    };
                    let b_str = match b_item {
                        Item::Stock(stock) => stock.isin.clone(),
                        Item::Index(index) => index.isin.clone(),
                    };
                    a_str.cmp(&b_str)
                }
                _ => std::cmp::Ordering::Equal,
            };

            match direction {
                Direction::Asc => ordering,
                Direction::Desc => ordering.reverse(),
            }
        });
    } else {
        panic!("Invalid combination of sort arguments");
    }

    // Apply limit
    if let Some(limit) = limit {
        sorted_items.truncate(limit as usize);
    }
    println!("After sorting: {:#?}", sorted_items);
    ctx.tracked_items = sorted_items;
}