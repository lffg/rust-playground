use Expr::*;

#[derive(Clone, Debug)]
enum Expr {
    Var(String),
    Abs(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

fn alpha_conversion(param: &str, arg: &Expr, expr: &Expr) -> Expr {
    match expr {
       Var(x) if x == param => Var(x.clone()),
       Var(_x) => arg.clone(),
       Abs(x, y) if x == param => Abs(x.clone(), y.clone()),
       Abs(x, y) => Abs(x.clone(), Box::new(alpha_conversion(param, arg, y))),
       App(x, y) => App(Box::new(alpha_conversion(param, arg, x)), Box::new(alpha_conversion(param, arg, y))),
    }
}

fn beta_reduction(expr: Expr) -> Expr {
    match expr {
        Var(x) => Var(x),
        Abs(x, y) => Abs(x, Box::new(beta_reduction(*y))),
        App(x, y) => {
            match *x {
                Abs(a, b) => alpha_conversion(&a, &y, &b),
                _ => App(Box::new(beta_reduction(*x)), Box::new(beta_reduction(*y))),
            }
        }
    }
}

fn main() {
    let expression = App(Box::new(Abs("x".into(), Box::new(Var("x".into())))), Box::new(Var("3".into())));

    println!("{:?}", beta_reduction(expression));
}