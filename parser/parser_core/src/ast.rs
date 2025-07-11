// Abstract Syntax Tree (AST), layout:
/*
Program
└── Command (Filter | Sort | Plot | Backtest)
    └── NamedArg
        └── Value
            ├── Literal (Number, String, Ident, etc.)
            ├── List / Tuple
            ├── ArithmeticExpr
            ├── LogicalExpr
            └── FunctionCall
*/

#[derive(Debug, Clone)]
pub struct Program {
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone)]
pub enum Command {
    Filter(Vec<NamedArg>),
    Sort(Vec<NamedArg>),
    Backtest(Vec<NamedArg>),
    Plot(Vec<NamedArg>)
}

#[derive(Debug, Clone)]
pub struct NamedArg {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Ident(String),
    Keyword(Keyword),
    Date(String),
    Duration(String),
    ArithmeticExpr(Expr),
    LogicalExpr(LogicalExpr),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Ident(String),
    FunctionCall(FunctionCall),
    Tuple(Vec<Value>),
    BinaryOp {
        left: Box<Expr>,
        op: ArithmeticOp,
        right: Box<Expr>,
    },
    Group(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum ArithmeticOp {
    Add,
    Div,
}

#[derive(Debug, Clone)]
pub enum LogicalExpr {
    Comparison {
        left: Operand,
        op: Comparator,
        right: Operand,
    },
    BinaryOp {
        left: Box<LogicalExpr>,
        op: LogicalOp,
        right: Box<LogicalExpr>,
    },
    Group(Box<LogicalExpr>),
}

#[derive(Debug, Clone)]
pub enum Operand {
    Number(f64),
    Ident(String),
    FunctionCall(FunctionCall),
    LogicalExpr(Box<LogicalExpr>),
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum Comparator {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<FunctionArg>,
}

#[derive(Debug, Clone)]
pub enum FunctionArg {
    Ident(String),
    Number(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Today,
    Stocks,
    Indexes,
}
