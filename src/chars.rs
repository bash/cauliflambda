use unicode_xid::UnicodeXID;

pub const LAMBDA: char = 'λ';

pub fn is_xid_start(c: char) -> bool {
    c.is_xid_start() && c != LAMBDA || matches!(c, '0'..='9')
}

pub fn is_xid_continue(c: char) -> bool {
    c.is_xid_continue() && c != LAMBDA
}

// ascSymbol from https://www.haskell.org/onlinereport/haskell2010/haskellch2.html#x7-160002.2
// with the exception of & and . as we use these characters in our grammar.
// We don't use , or ; in our grammar, so these are available additionally.
pub fn is_ascii_symbol(c: char) -> bool {
    matches!(
        c,
        '!' | '#'
            | '$'
            | '%'
            | '⋆'
            | '+'
            | '/'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '\\'
            | '^'
            | '|'
            | '-'
            | '~'
            | ':'
            | ';'
            | ','
    )
}
