pub enum Expr0 {
    Expr0 {
        right: Box<Expr0>,
        left: Box<Expr0>,
        operator: String,
    },
    Term(Term),
}

pub enum Term {
    Expr0(Box<Expr0>),
    Array(Vec<Expr0>),
    Literal(Literal),
}

pub enum Literal {
    Integer(i64),
}
