//use crate::ast::;
use pest::iterators::Pair;
use crate::Rule;

pub fn parse_pairs(pair: Pair<Rule>) {
    for command_pair in pair.into_inner() {
        println!("Command: {:?}", command_pair.as_str());
        for inner_pair in command_pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::plot_cmd => {
                    parse_command(inner_pair);
                }
                Rule::filter_cmd => {
                    parse_command(inner_pair);
                }
                Rule::sort_cmd => {
                    parse_command(inner_pair);
                }
                Rule::backtest_cmd => {
                    parse_command(inner_pair);
                }
                _ => {
                    println!("Other command: {:?}", inner_pair.as_str());
                }
            }
        }
    }
}

fn parse_command(pair: Pair<Rule>) {
    println!("Command Type: {:?}", pair.as_rule());
    let args = pair.into_inner().next().unwrap();
    for inner_pair in args.into_inner() {
        parse_argument(inner_pair);
    }
}

fn parse_argument(pair: Pair<Rule>) {
    let mut name = "";
    let mut value = "";
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::ident => {
                name = inner_pair.as_str();
            }
            Rule::value => {
                value = inner_pair.as_str();
                parse_arg_value(inner_pair);
            }
            _ => {
                println!("Unexpected argument part: {:?}", inner_pair.as_str());
            }
        }
    }
    println!("Arg. Name: {}, Value: {}", name, value);
}

fn parse_arg_value(pair: Pair<Rule>) {
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::number => {
                println!("Number: {}", inner_pair.as_str());
            }
            Rule::string => {
                println!("String: {}", inner_pair.as_str());
            }
            Rule::ident => {
                println!("Identifier: {}", inner_pair.as_str());
            }
            Rule::keyword => {
                println!("Keyword: {}", inner_pair.as_str());
            }
            Rule::date => {
                println!("Date: {}", inner_pair.as_str());
            }
            Rule::duration => {
                println!("Duration: {}", inner_pair.as_str());
            }
            Rule::arithmetic_expr => {
                parse_arithmetic_expr(inner_pair);
            }
            Rule::logical_block => {
                parse_logical_block(inner_pair);
            }
            Rule::list => {
                parse_list(inner_pair);
            }
            Rule::function_call => {
                parse_function_call(inner_pair);
            }
            _ => {
                println!("Unknown value type: {:?}", inner_pair.as_rule());
            }
        }
    }
}

//-- ARITHMETIC EXPRESSION PARSING --
fn parse_arithmetic_expr(pair: Pair<Rule>) {
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::arithmetic_term => {
                for term in inner_pair.into_inner() {
                    match term.as_rule() {
                        Rule::function_call => parse_function_call(term),
                        Rule::ident => println!("    Identifier: {}", term.as_str()),
                        Rule::number => println!("    Number: {}", term.as_str()),
                        Rule::tuple_expr => println!("    Tuple Expression: {}", term.as_str()), // or recursively parse if needed
                        Rule::arithmetic_expr => parse_arithmetic_expr(term), // handle nested
                        _ => println!("    Unknown arithmetic term: {:?}", term.as_rule()),
                    }
                }
            }
            Rule::operation => {
                println!("  Operation: {}", inner_pair.as_str());
            }
            _ => {
                println!("  Operator or Unexpected: {}", inner_pair.as_str());
            }
        }
    }
}

//-- LOGICAL BLOCK PARSING --
fn parse_logical_block(pair: Pair<Rule>) {
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::logical_expr => parse_logical_expr(inner_pair),
            _ => println!("  Unexpected in logical block: {:?}", inner_pair.as_rule()),
        }
    }
}

fn parse_logical_expr(pair: Pair<Rule>) {
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::logical_expr_inner => {
                parse_logical_expr_inner(inner_pair);
            }
            Rule::LOGICAL_OP => {
                println!("    Logical Operator: {}", inner_pair.as_str());
            }
            _ => {
                println!("    Unknown logical_expr component: {:?}", inner_pair.as_rule());
            }
        }
    }
}

fn parse_logical_expr_inner(pair: Pair<Rule>) {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::comparison => {
                parse_comparison(inner);
            }
            Rule::logical_expr => {
                parse_logical_expr(inner); // handle nested expressions
            }
            _ => {
                println!("      Unknown logical_expr_inner: {:?}", inner.as_rule());
            }
        }
    }
}

fn parse_comparison(pair: Pair<Rule>) {
    let mut parts = pair.into_inner();

    let lhs = parts.next().unwrap();
    let cmp = parts.next().unwrap();
    let rhs = parts.next().unwrap();

    println!("    Comparison:");
    println!("      LHS:");
    parse_operand(lhs);
    println!("      Comparator: {}", cmp.as_str());
    println!("      RHS:");
    parse_operand(rhs);
}

fn parse_operand(pair: Pair<Rule>) {
    println!("      Operand: {:?}", pair.as_rule());
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::function_call => {
                parse_function_call(inner);
            }
            Rule::ident => {
                println!("        Identifier: {}", inner.as_str());
            }
            Rule::number => {
                println!("        Number: {}", inner.as_str());
            }
            Rule::logical_expr => {
                parse_logical_expr(inner);
            }
            _ => {
                println!("        Unknown operand inner: {:?}", inner.as_rule());
            }
        }
    }
}

//-- LIST PARSING --
fn parse_list(pair: Pair<Rule>) {
    let mut items = Vec::new();
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::list_items => {
                for item in inner_pair.into_inner() {
                    match item.as_rule() {
                        Rule::value => {
                            println!("  List item: {}", item.as_str());
                            // PARSER FURTHER
                            items.push(item.as_str().to_string());
                            parse_arg_value(item);
                        }
                        _ => {
                            println!("  Unexpected in list: {:?}", item.as_rule());
                        }
                    }
                }
            }
            _ => {
                println!("Unexpected list part: {:?}", inner_pair.as_rule());
            }
        }
    }
    println!("List Items: {:?}", items);
}

//-- FUNCTION CALL PARSING --
fn parse_function_call(pair: Pair<Rule>) {
    let mut name = "";
    let mut args = Vec::new();

    let mut inner_pairs = pair.into_inner();
    if let Some(name_pair) = inner_pairs.next() {
        name = name_pair.as_str();
    }

    if let Some(args_pair) = inner_pairs.next() {
        if args_pair.as_rule() == Rule::arguments {
            for arg in args_pair.into_inner() {
                println!("  Argument: {:?}", arg.as_str());
                // Dig into the actual value inside the `argument` wrapper
                for actual in arg.into_inner() {
                    args.push(actual.as_str().to_string());

                    match actual.as_rule() {
                        Rule::ident => println!("    Arg Ident: {}", actual.as_str()),
                        Rule::number => println!("    Arg Number: {}", actual.as_str()),
                        Rule::string => println!("    Arg String: {}", actual.as_str()),
                        _ => println!("    Unknown function arg: {:?}", actual.as_rule()),
                    }
                }
            }
        }
    }

    println!("Function Call: {}({})", name, args.join(", "));
}
