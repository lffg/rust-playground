use Expr::*;

#[derive(Clone, Debug)]
enum Expr {
    Var(String),
    Abs(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

fn alpha(param: &str, arg: Expr, expr: Expr) -> Expr {
    match expr {
        Var(x) if x == param => arg,
        Var(x) => Var(x),

        Abs(x, y) if x == param => Abs(x, alpha(param, arg, *y).into()),
        Abs(x, y) => Abs(x, y),

        App(x, y) => App(
            alpha(param, arg.clone(), *x).into(),
            alpha(param, arg, *y).into(),
        ),
    }
}

fn beta(expr: Expr) -> Expr {
    match expr {
        Var(x) => Var(x),
        Abs(x, y) => Abs(x, beta(*y).into()),
        App(x, y) => match *x {
            Abs(param, body) => alpha(&param, beta(*y).into(), beta(*body).into()),
            _ => App(beta(*x).into(), beta(*y).into()),
        },
    }
}

fn stringify(expr: &Expr) -> String {
    match expr {
        Var(name) => name.clone(),
        Abs(name, x) => format!("(λ{name}. {})", stringify(&x)),
        App(a, b) => format!("{} {}", stringify(&a), stringify(&b)),
    }
}

fn main() {
    // (λx. λy. y) 3 (λx. x) 4
    let expr = App(
        App(
            App(
                Abs("x".into(), Abs("y".into(), Var("y".into()).into()).into()).into(),
                Var("3".into()).into(),
            )
            .into(),
            Abs("x".into(), Var("x".into()).into()).into(),
        )
        .into(),
        Var("4".into()).into(),
    );

    println!("expr:\n{expr:#?}\n");
    println!("string:\n{}", stringify(&expr));
    println!("---");
    println!("{}", stringify(&beta(beta(beta(expr)))));
}
