use std::io::stdin;

use cauliflambda::evaluation::{abs, app, ChurchNumeral, Term, Tuple, Variable};
use rand::{thread_rng, Rng};

pub(crate) fn perform_side_effect<'a>(name: Variable<'a>, term: Term<'a>) -> Option<Term<'a>> {
    const BEEP: Variable<'_> = Variable::new("beep");
    const RAND: Variable<'_> = Variable::new("rand");
    const READ: Variable<'_> = Variable::new("read");
    const WRITE: Variable<'_> = Variable::new("write");
    const F: Variable<'_> = Variable::new("f");
    if name == BEEP {
        print!("\x07");
        Some(term)
    } else if name == RAND {
        Some(abs(F, app(F, rand(term).unwrap_or(error()))))
    } else if name == READ {
        Some(app(abs(F, app(F, read().unwrap_or(error()))), term))
    } else if name == WRITE {
        Some(write(term).map(|_| id()).unwrap_or(error()))
    } else {
        None
    }
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
