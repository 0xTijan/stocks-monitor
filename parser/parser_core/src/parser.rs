use pest::iterators::Pair;
use crate::Rule;
use crate::ast::*;

/// Entry point for parsing
pub fn parse_pairs(pair: Pair<Rule>) -> Program {
    let mut commands = Vec::new();
    for command_pair in pair.into_inner() {
        for inner_pair in command_pair.into_inner() {
            let cmd = match inner_pair.as_rule() {
                Rule::plot_cmd => Command::Plot(parse_command(inner_pair)),
                Rule::filter_cmd => Command::Filter(parse_command(inner_pair)),
                Rule::sort_cmd => Command::Sort(parse_command(inner_pair)),
                Rule::backtest_cmd => Command::Backtest(parse_command(inner_pair)),
                _ => continue,
            };
            commands.push(cmd);
        }
    }
    Program { commands }
}

fn parse_command(pair: Pair<Rule>) -> Vec<NamedArg> {
    let args = pair.into_inner().next().unwrap(); // assumes `args` is inside
    args.into_inner()
        .map(parse_argument)
        .collect()
}

fn parse_argument(pair: Pair<Rule>) -> NamedArg {
    let mut name = "";
    let mut value_pair = None;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::ident => name = inner_pair.as_str(),
            Rule::value => value_pair = Some(inner_pair),
            _ => {}
        }
    }

    let value = value_pair.map(parse_arg_value).unwrap();
    NamedArg {
        name: name.to_string(),
        value,
    }
}

fn parse_arg_value(pair: Pair<Rule>) -> Value {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::number => Value::Number(inner.as_str().parse::<f64>().unwrap()),
        Rule::string => Value::String(inner.as_str().to_string()),
        Rule::ident => Value::Ident(inner.as_str().to_string()),
        Rule::keyword => Value::Keyword(match inner.as_str() {
            "today" => Keyword::Today,
            "stocks" => Keyword::Stocks,
            "indexes" => Keyword::Indexes,
            _ => panic!("Unknown keyword"),
        }),
        Rule::date => Value::Date(inner.as_str().to_string()),
        Rule::duration => Value::Duration(inner.as_str().to_string()),
        Rule::arithmetic_expr => Value::ArithmeticExpr(parse_arithmetic_expr(inner)),
        Rule::logical_block => Value::LogicalExpr(parse_logical_block(inner)),
        Rule::list => Value::List(parse_list(inner)),
        Rule::function_call => Value::FunctionCall(parse_function_call(inner)),
        _ => panic!("Unknown value type: {:?}", inner.as_rule()),
    }
}

//-- ARITHMETIC EXPRESSION PARSING --
fn parse_arithmetic_expr(pair: Pair<Rule>) -> Expr {
    let mut expr = None;
    let mut op = None;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::arithmetic_term => {
                let term_expr = parse_arithmetic_term(inner_pair);
                if expr.is_none() {
                    expr = Some(term_expr);
                } else {
                    expr = Some(Expr::BinaryOp {
                        left: Box::new(expr.unwrap()),
                        op: op.take().unwrap(),
                        right: Box::new(term_expr),
                    });
                }
            }
            Rule::operation => {
                op = Some(match inner_pair.as_str() {
                    "+" => ArithmeticOp::Add,
                    "/" => ArithmeticOp::Div,
                    _ => panic!("Unknown arithmetic op"),
                });
            }
            _ => {}
        }
    }

    expr.unwrap()
}

fn parse_arithmetic_term(pair: Pair<Rule>) -> Expr {
    let term = pair.into_inner().next().unwrap();
    match term.as_rule() {
        Rule::function_call => Expr::FunctionCall(parse_function_call(term)),
        Rule::ident => Expr::Ident(term.as_str().to_string()),
        Rule::number => Expr::Number(term.as_str().parse::<f64>().unwrap()),
        Rule::tuple_expr => {
            let values = term.into_inner().map(parse_arg_value).collect();
            Expr::Tuple(values)
        }
        Rule::arithmetic_expr => Expr::Group(Box::new(parse_arithmetic_expr(term))),
        _ => panic!("Unknown arithmetic term: {:?}", term.as_rule()),
    }
}

//-- LOGICAL BLOCK PARSING --
fn parse_logical_block(pair: Pair<Rule>) -> LogicalExpr {
    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::logical_expr {
            return parse_logical_expr(inner_pair);
        }
    }
    panic!("Expected logical_expr in block");
}

fn parse_logical_expr(pair: Pair<Rule>) -> LogicalExpr {
    let mut expr = None;
    let mut op = None;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::logical_expr_inner => {
                let inner = parse_logical_expr_inner(inner_pair);
                if expr.is_none() {
                    expr = Some(inner);
                } else {
                    expr = Some(LogicalExpr::BinaryOp {
                        left: Box::new(expr.unwrap()),
                        op: op.take().unwrap(),
                        right: Box::new(inner),
                    });
                }
            }
            Rule::LOGICAL_OP => {
                op = Some(match inner_pair.as_str() {
                    "AND" => LogicalOp::And,
                    "OR" => LogicalOp::Or,
                    _ => panic!("Unknown logical op"),
                });
            }
            _ => {}
        }
    }

    expr.unwrap()
}

fn parse_logical_expr_inner(pair: Pair<Rule>) -> LogicalExpr {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::comparison => parse_comparison(inner),
        Rule::logical_expr => LogicalExpr::Group(Box::new(parse_logical_expr(inner))),
        _ => panic!("Unknown logical_expr_inner: {:?}", inner.as_rule()),
    }
}

fn parse_comparison(pair: Pair<Rule>) -> LogicalExpr {
    let mut parts = pair.into_inner();
    let lhs = parse_operand(parts.next().unwrap());
    let cmp = match parts.next().unwrap().as_str() {
        ">" => Comparator::Gt,
        ">=" => Comparator::Gte,
        "<" => Comparator::Lt,
        "<=" => Comparator::Lte,
        "==" => Comparator::Eq,
        _ => panic!("Unknown comparator"),
    };
    let rhs = parse_operand(parts.next().unwrap());

    LogicalExpr::Comparison {
        left: lhs,
        op: cmp,
        right: rhs,
    }
}

fn parse_operand(pair: Pair<Rule>) -> Operand {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::function_call => Operand::FunctionCall(parse_function_call(inner)),
        Rule::ident => Operand::Ident(inner.as_str().to_string()),
        Rule::number => Operand::Number(inner.as_str().parse::<f64>().unwrap()),
        Rule::logical_expr => Operand::LogicalExpr(Box::new(parse_logical_expr(inner))),
        _ => panic!("Unknown operand inner: {:?}", inner.as_rule()),
    }
}

//-- LIST PARSING --
fn parse_list(pair: Pair<Rule>) -> Vec<Value> {
    let mut items = Vec::new();
    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::list_items {
            for item in inner_pair.into_inner() {
                if item.as_rule() == Rule::value {
                    items.push(parse_arg_value(item));
                }
            }
        }
    }
    items
}

//-- FUNCTION CALL PARSING --
fn parse_function_call(pair: Pair<Rule>) -> FunctionCall {
    let mut name = "";
    let mut args = Vec::new();

    let mut inner_pairs = pair.into_inner();
    if let Some(name_pair) = inner_pairs.next() {
        name = name_pair.as_str();
    }

    if let Some(args_pair) = inner_pairs.next() {
        if args_pair.as_rule() == Rule::arguments {
            for arg in args_pair.into_inner() {
                for actual in arg.into_inner() {
                    match actual.as_rule() {
                        Rule::ident => args.push(FunctionArg::Ident(actual.as_str().to_string())),
                        Rule::number => args.push(FunctionArg::Number(actual.as_str().parse::<f64>().unwrap())),
                        Rule::string => args.push(FunctionArg::String(actual.as_str().to_string())),
                        _ => panic!("Unknown function arg"),
                    }
                }
            }
        }
    }

    FunctionCall {
        name: name.to_string(),
        args,
    }
}
