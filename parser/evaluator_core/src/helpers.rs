use chrono::{Duration, Local, NaiveDate};
use parser_core::ast::{FunctionArg, Expr, ArithmeticOp};
use crate::response_types::{ChartData, TrackedItem};


pub fn get_today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn all_stocks_symbols() -> Vec<String> {
    vec![
        "CICG".to_string(),
        "EQNX".to_string(),
        "KRKG".to_string(),
        "LKPG".to_string(),
        "NLBR".to_string(),
        "PETG".to_string(),
        "POSR".to_string(),
        "TLSG".to_string(),
        "UKIG".to_string(),
        "ZVTG".to_string(),
        "ADPL".to_string(),
        "ADRS2".to_string(),
        "ARNT".to_string(),
        "ATGR".to_string(),
        "AUHR".to_string(),
        "BSQR".to_string(),
        "DDJH".to_string(),
        "DLKV".to_string(),
        "ERNT".to_string(),
        "GRNL".to_string(),
        "HPB".to_string(),
        "HT".to_string(),
        "IG".to_string(),
        "IGH".to_string(),
        "IKBA".to_string(),
        "INGR".to_string(),
        "JDGT".to_string(),
        "JDPL".to_string(),
        "KODT".to_string(),
        "KOEI".to_string(),
        "KRAS".to_string(),
        "KTJV".to_string(),
        "LKPC".to_string(),
        "LKRI".to_string(),
        "MDKA".to_string(),
        "MONP".to_string(),
        "PLAG".to_string(),
        "PODR".to_string(),
        "RIVP".to_string(),
        "SPAN".to_string(),
        "ULPL".to_string(),
        "VLEN".to_string(),
        "ZABA".to_string(),
        "ZB".to_string(),
    ]
}

pub fn all_indexes_symbols() -> Vec<String> {
    vec![
        "SBITOP".to_string(),
        "SBITR".to_string(),
        "ADRPR".to_string(),
        "C10TR".to_string(),
        "CBX".to_string(),
        "CBX10".to_string(),
        "CBXPR".to_string(),
        "CBXTR".to_string(),
    ]
}

pub fn create_function_id(name: &String, args: &Vec<FunctionArg>, item: &String) -> String {
    let args_str: Vec<String> = args
        .iter()
        .filter_map(|arg| {
            if let FunctionArg::Number(num) = arg {
                Some(num.to_string())
            } else {
                None
            }
        })
        .collect();
    
    format!("{}_{}_{}", name, args_str.join(","), item)
}

pub fn expr_to_id(expr: &Expr, tracked_item: Option<&TrackedItem>) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Ident(s) => s.clone(),
        Expr::Group(inner) => expr_to_id(inner, tracked_item),

        Expr::BinaryOp { left, op, right } => {
            let left_id = expr_to_id(left, tracked_item);
            let right_id = expr_to_id(right, tracked_item);
            let op_str = match op {
                ArithmeticOp::Add => "+",
                ArithmeticOp::Sub => "-",
                ArithmeticOp::Div => "/",
            };
            format!("{}{}{}", left_id, op_str, right_id)
        }

        Expr::FunctionCall(func_call) => {
            let ids = func_call.args.iter().filter_map(|arg| {
                if let FunctionArg::Ident(ident) = arg {
                    Some(ident.clone())
                } else {
                    if let Some(tracked_item) = tracked_item {
                        Some(tracked_item.id.clone())
                    } else {
                        None
                    }
                }
            }).collect::<Vec<String>>();
            create_function_id(&func_call.name, &func_call.args, ids.first().unwrap_or(&"".to_string()))
        }

        Expr::Tuple(_) => {
            panic!("Tuple to_id not supported");
        }
    }
}

pub fn number_series_with_dates(from: &str, to: &str, value: f64) -> Vec<(String, (f64, f64, f64, f64))> {
    let start_date = NaiveDate::parse_from_str(from, "%Y-%m-%d").expect("Invalid start date");
    let end_date = NaiveDate::parse_from_str(to, "%Y-%m-%d").expect("Invalid end date");

    let mut result = Vec::new();
    let mut current = start_date;

    while current <= end_date {
        result.push((current.to_string(), (value, value, value, value)));
        current += Duration::days(1);
    }

    result
}

pub fn enum_to_chart_data(series: Vec<(String, (f64, f64, f64, f64))>) -> Vec<ChartData> {
    series.into_iter()
        .map(|(date, value)| ChartData { date, value })
        .collect()
}

pub fn vol_to_chart_data(series: Vec<(String, (f64, f64))>) -> Vec<ChartData> {
    series.into_iter()
        .map(|(date, value)| ChartData { date, value: (value.0, value.1, 0.0, 0.0) })
        .collect()
}

pub fn rebase_data(data: &Vec<ChartData>, rebase: f64) -> Vec<ChartData> {
        if data.is_empty() {
        return vec![];
    }

    let base_close = data[0].value.0;

    data.iter()
        .map(|entry| {
            let (close, open, high, low) = entry.value;
            let rebased_close = (close / base_close) * rebase;
            ChartData {
                date: entry.date.clone(),
                value: (rebased_close, open, high, low),
            }
        })
        .collect()
}