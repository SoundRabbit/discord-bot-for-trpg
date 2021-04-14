use async_std::sync::Arc;
use std::collections::HashMap;

pub struct Proc(Vec<Arc<Expr0>>);

pub enum Expr0 {
    Expr0 {
        right: Arc<Expr0>,
        left: Arc<Expr0>,
        operator: String,
    },
    Fn {
        arg: Arc<String>,
        value: Arc<Expr0>,
    },
    Def {
        ident: Arc<Ident>,
        value: Arc<Expr0>,
    },
    Term(Term),
}

pub enum Term {
    Expr0(Arc<Expr0>),
    Proc(Proc),
    Array(Vec<Arc<Expr0>>),
    Record(HashMap<Arc<String>, Arc<Expr0>>),
    Literal(Literal),
}

pub enum Literal {
    Integer(i64),
    Ident(Ident),
}

#[derive(PartialEq)]
pub enum Ident {
    Strict(Arc<String>),
    Lazy(Arc<String>),
}

impl Proc {
    pub fn new(proc: Vec<Arc<Expr0>>) -> Self {
        Self(proc)
    }
}

impl std::ops::Deref for Proc {
    type Target = Vec<Arc<Expr0>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Proc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Ident {
    pub fn name(&self) -> Arc<String> {
        match self {
            Self::Strict(x) => Arc::clone(x),
            Self::Lazy(x) => Arc::clone(x),
        }
    }

    pub fn is_strict(&self) -> bool {
        match self {
            Self::Strict(_) => true,
            _ => false,
        }
    }
}
