fn main() {
    #[derive(Debug)]
    enum Expr {
        Number(i32),
        BinaryOp {
            left: Box<Expr>,
            op: Op,
            right: Box<Expr>,
        },
    }

    #[derive(Debug)]
    enum Op {
        Add,
        Sub,
        Mul,
        Div,
    }

    fn eval(expr: &Expr) -> i32 {
        match expr {
            Expr::Number(value) => *value,
            Expr::BinaryOp { left, op, right } => {
                let left_val = eval(left);
                let right_val = eval(right);
                match op {
                    Op::Add => left_val + right_val,
                    Op::Sub => left_val - right_val,
                    Op::Mul => left_val * right_val,
                    Op::Div => left_val / right_val,
                }
            }
        }
    }

    let expr = Expr::BinaryOp {
        left: Box::new(Expr::Number(5)),
        op: Op::Add,
        right: Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Number(3)),
            op: Op::Mul,
            right: Box::new(Expr::Number(2)),
        }),
    };

    println!("AST: {:?}", expr);
    println!("Result: {}", eval(&expr));
}
