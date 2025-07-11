//use crate::ast::;
use pest::iterators::Pair;
use crate::Rule;

pub fn parse_pairs(pair: Pair<Rule>) {
    let command = pair.into_inner().next().unwrap();
    println!("Command: {:?}", command.as_str());

    for inner_pair in command.into_inner() {
        match inner_pair.as_rule() {
            Rule::plot_cmd => {
                parse_plot(inner_pair);
            }
            Rule::filter_cmd => {
                parse_filter(inner_pair);
            }
            Rule::sort_cmd => {
                parse_sort(inner_pair);
            }
            Rule::backtest_cmd => {
                parse_backtest(inner_pair);
            }
            _ => {
                println!("Other command: {:?}", inner_pair.as_str());
            }
        }
    }
}

fn parse_plot(pair: Pair<Rule>) {
    println!("Plot: {:?}", pair.as_str());
}

fn parse_filter(pair: Pair<Rule>) {
    println!("Filter: {:?}", pair.as_str());
}

fn parse_sort(pair: Pair<Rule>) {
    println!("Sort: {:?}", pair.as_str());
}

fn parse_backtest(pair: Pair<Rule>) {
    println!("Backtest: {:?}", pair.as_str());
}