use cauliflambda::evaluation::{abs, app, Encode as _, SideEffect, Term, Variable};
use rand::{thread_rng, Rng};
use std::io::stdin;

pub(crate) fn perform_side_effect<'a>(s: SideEffect<'a>, term: Term<'a>) -> Option<Term<'a>> {
    const F: Variable<'_> = Variable::new("f");

    match s.name {
        "beep" => {
            beep();
            Some(term)
        }
        "rand" => Some(abs(F, app(F, rand(term).unwrap_or(error())))),
        "read" => Some(app(abs(F, app(F, read().unwrap_or(error()))), term)),
        "write" => Some(write(term).map(|_| id()).unwrap_or(error())),
        "error" => None,
        _ => Some(Term::SideEffect(SideEffect::new("error"))),
    }
}

fn beep() {
    print!("\x07");
}

fn rand(term: Term<'_>) -> Option<Term<'static>> {
    let (start, end): (u64, u64) = term.decode()?;
    if start == end {
        return None;
    }
    let n = thread_rng().gen_range(start..end);
    Some(n.encode())
}

fn read() -> Option<Term<'static>> {
    let mut s = String::new();
    stdin().read_line(&mut s).ok()?;
    Some(s.trim().parse::<u64>().ok()?.encode())
}

fn write(n: Term<'_>) -> Option<()> {
    let n: u64 = n.decode()?;
    println!("{n}");
    Some(())
}

fn id() -> Term<'static> {
    const X: Variable<'_> = Variable::new("x");
    abs(X, X)
}

fn error() -> Term<'static> {
    Variable::new("error").into()
}
