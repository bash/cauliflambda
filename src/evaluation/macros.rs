macro_rules! App {
    ($($t:tt)*) => {
        Term::App(box Application { $($t)* })
    }
}

macro_rules! Abs {
    ($($t:tt)*) => {
        Term::Abs(box Abstraction { $($t)* })
    }
}
