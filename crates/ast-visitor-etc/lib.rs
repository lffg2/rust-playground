pub mod expr_ast {
    #[derive(Clone, Copy)]
    pub enum BinaryOp {
        Add,
        Sub,
        Mul,
        Div,
    }

    #[derive(Clone, Copy)]
    pub enum UnaryOp {
        Neg,
    }

    #[derive(Clone)]
    pub struct Binary {
        pub op: BinaryOp,
        pub lhs: Box<Expr>,
        pub rhs: Box<Expr>,
    }

    #[derive(Clone)]
    pub struct Unary {
        pub op: UnaryOp,
        pub expr: Box<Expr>,
    }

    #[derive(Clone)]
    pub struct Group {
        pub expr: Box<Expr>,
    }

    #[derive(Clone)]
    pub struct Num {
        pub lit: f64,
    }

    #[derive(Clone)]
    pub enum Expr {
        Binary(Binary),
        Unary(Unary),
        Group(Group),
        Num(Num),
    }
}

use expr_ast::*;

pub fn eval_expr(expr: &Expr) -> f64 {
    use Expr::*;
    match expr {
        Binary(e) => {
            let a = eval_expr(&e.lhs);
            let b = eval_expr(&e.rhs);
            match e.op {
                BinaryOp::Add => a + b,
                BinaryOp::Sub => a - b,
                BinaryOp::Mul => a * b,
                BinaryOp::Div => a / b,
            }
        }
        Unary(e) => {
            let a = eval_expr(&e.expr);
            match e.op {
                UnaryOp::Neg => -a,
            }
        }
        Group(e) => eval_expr(&e.expr),
        Num(e) => e.lit,
    }
}

pub fn lispify_expr(expr: &Expr) -> String {
    fn paren<const N: usize>(label: &str, exprs: [&Expr; N]) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(label);
        for expr in exprs {
            out.push(' ');
            out.push_str(&lispify_expr(expr));
        }
        out.push(')');
        out
    }

    use Expr::*;
    match expr {
        Binary(e) => {
            let op_repr = match e.op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
            };
            paren(op_repr, [&e.lhs, &e.rhs])
        }
        Unary(e) => match e.op {
            UnaryOp::Neg => paren("-", [&e.expr]),
        },
        Group(e) => paren("group", [&e.expr]),
        Num(e) => e.lit.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_expr() {
        let expr = mock_expr();
        assert_eq!(eval_expr(&expr), 18.);
    }

    #[test]
    fn test_lispify_expr() {
        let expr = mock_expr();
        assert_eq!(lispify_expr(&expr), "(+ 4 (* (group (+ 1 (* 2 3))) 2))");
    }

    fn mock_expr() -> Expr {
        // 4 + (1 + 2 * 3) * 2
        Expr::Binary(Binary {
            op: BinaryOp::Add,
            lhs: Expr::Num(Num { lit: 4. }).into(),
            rhs: Expr::Binary(Binary {
                op: BinaryOp::Mul,
                lhs: Expr::Group(Group {
                    expr: Expr::Binary(Binary {
                        op: BinaryOp::Add,
                        lhs: Expr::Num(Num { lit: 1. }).into(),
                        rhs: Expr::Binary(Binary {
                            op: BinaryOp::Mul,
                            lhs: Expr::Num(Num { lit: 2. }).into(),
                            rhs: Expr::Num(Num { lit: 3. }).into(),
                        })
                        .into(),
                    })
                    .into(),
                })
                .into(),
                rhs: Expr::Num(Num { lit: 2. }).into(),
            })
            .into(),
        })
    }
}
