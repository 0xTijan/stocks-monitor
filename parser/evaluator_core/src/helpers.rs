use chrono::Local;
use parser_core::ast::{FunctionArg, Expr, ArithmeticOp};

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

pub fn expr_to_id(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Ident(s) => s.clone(),
        Expr::Group(inner) => expr_to_id(inner),

        Expr::BinaryOp { left, op, right } => {
            let left_id = expr_to_id(left);
            let right_id = expr_to_id(right);
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
                    None
                }
            }).collect::<Vec<String>>();
            create_function_id(&func_call.name, &func_call.args, ids.first().unwrap_or(&"".to_string()))
        }

        Expr::Tuple(_) => {
            panic!("Tuple to_id not supported");
        }
    }
}