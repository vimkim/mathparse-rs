pub enum Expr {
    Number(f64),
    BinaryOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    // Optionally, you can handle Unary operations:
    // UnaryOp {
    //     op: Op,
    //     expr: Box<Expr>,
    // },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
