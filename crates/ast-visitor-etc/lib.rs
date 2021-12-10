pub mod expr_ast {
    pub enum BinaryOp {
        Add,
        Sub,
        Mul,
        Div,
    }

    pub enum UnaryOp {
        Neg,
    }

    pub struct Binary {
        pub op: BinaryOp,
        pub lhs: Box<Expr>,
        pub rhs: Box<Expr>,
    }

    pub struct Unary {
        pub op: UnaryOp,
        pub expr: Box<Expr>,
    }

    pub struct Group {
        pub expr: Box<Expr>,
    }

    pub struct Num {
        pub lit: f64,
    }

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
        Unary(e) => match e.op {
            UnaryOp::Neg => -eval_expr(&e.expr),
        },
        Group(e) => eval_expr(&e.expr),
        Num(e) => e.lit,
    }
}

pub fn lisp_printer(expr: &Expr) -> String {
    fn paren<const N: usize>(label: &str, exprs: [&Expr; N]) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(label);
        for expr in exprs {
            out.push(' ');
            out.push_str(&lisp_printer(expr));
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
        Group(e) => lisp_printer(&e.expr),
        Num(e) => e.lit.to_string(),
    }
}

// See: https://wiki.c2.com/?PostfixNotation
pub fn rpn_printer(expr: &Expr) -> String {
    use Expr::*;
    match expr {
        Binary(e) => {
            let a = rpn_printer(&e.lhs);
            let b = rpn_printer(&e.rhs);
            let op_repr = match e.op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
            };
            format!("{} {} {}", a, b, op_repr)
        }
        Unary(e) => match e.op {
            UnaryOp::Neg => format!("- {}", rpn_printer(&e.expr)),
        },
        Group(e) => rpn_printer(&e.expr),
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
    fn test_lisp_printer() {
        let expr = mock_expr();
        assert_eq!(lisp_printer(&expr), "(+ 4 (* (group (+ 1 (* 2 3))) 2))");
    }

    #[test]
    fn test_rpn_printer() {
        let expr = mock_expr();
        assert_eq!(rpn_printer(&expr), "4 1 2 3 * + 2 * +");
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
