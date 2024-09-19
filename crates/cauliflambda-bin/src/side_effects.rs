use cauliflambda::evaluation::{abs, app, ChurchNumeral, SideEffect, Term, Tuple, Variable};
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
        _ => Some(error()),
    }
}

fn beep() {
    print!("\x07");
}

fn rand(term: Term<'_>) -> Result<Term<'static>, ()> {
    let (start, end): (ChurchNumeral, ChurchNumeral) = Tuple::try_from(term)?.0;
    if start == end {
        return Err(());
    }
    let n = thread_rng().gen_range(start.0..end.0);
    Ok(ChurchNumeral(n).into())
}

fn read() -> Result<Term<'static>, ()> {
    let mut s = String::new();
    stdin().read_line(&mut s).map_err(|_| ())?;
    Ok(ChurchNumeral(s.trim().parse().map_err(|_| ())?).into())
}

fn write(n: Term<'_>) -> Result<(), ()> {
    let n = ChurchNumeral::try_from(n)?.0;
    println!("{n}");
    Ok(())
}

fn id() -> Term<'static> {
    const X: Variable<'_> = Variable::new("x");
    abs(X, X)
}

fn error() -> Term<'static> {
    Variable::new("error").into()
}
