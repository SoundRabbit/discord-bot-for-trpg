use crate::parser::ast;
use std::collections::HashMap;

pub enum Value {
    Evaluted(Evaluted),
    Unevaluted(ast::Expr0),
}

pub enum Evaluted {
    None,
    Integer(i64),
    Boolean(bool),
    Array(Vec<Evaluted>),
    Record(HashMap<String, Evaluted>),
}

impl Value {
    pub fn evalute(&mut self, rng: &mut impl rand::Rng, log: &mut Vec<String>) -> &Evaluted {
        match self {
            Self::Evaluted(value) => value,
            Self::Unevaluted(expr) => {
                *self = Self::Evaluted(expr.evalute(rng, log));
                self.evalute(rng, log)
            }
        }
    }
}

impl std::fmt::Debug for Evaluted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Integer(val) => write!(f, "{}", val),
            Self::Boolean(val) => write!(f, "{}", val),
            Self::Array(vals) => {
                write!(f, "{:?}", vals)
            }
            Self::Record(vals) => {
                write!(f, "{:?}", vals)
            }
        }
    }
}

impl std::fmt::Display for Evaluted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ast::Expr0 {
    fn evalute(&self, rng: &mut impl rand::Rng, log: &mut Vec<String>) -> Evaluted {
        match self {
            Self::Expr0 {
                left,
                right,
                operator,
            } => match operator.as_str() {
                "==" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        Evaluted::Boolean(left == right)
                    } else {
                        Evaluted::None
                    }
                }
                "!=" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        Evaluted::Boolean(left != right)
                    } else {
                        Evaluted::None
                    }
                }
                "<=" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        Evaluted::Boolean(left <= right)
                    } else {
                        Evaluted::None
                    }
                }
                ">=" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        Evaluted::Boolean(left >= right)
                    } else {
                        Evaluted::None
                    }
                }
                "<" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        Evaluted::Boolean(left < right)
                    } else {
                        Evaluted::None
                    }
                }
                ">" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        Evaluted::Boolean(left > right)
                    } else {
                        Evaluted::None
                    }
                }
                "+" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        Evaluted::Integer(left + right)
                    } else {
                        Evaluted::None
                    }
                }
                "-" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        Evaluted::Integer(left - right)
                    } else {
                        Evaluted::None
                    }
                }
                "*" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        Evaluted::Integer(left * right)
                    } else {
                        Evaluted::None
                    }
                }
                "/" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        Evaluted::Integer(left / right)
                    } else {
                        Evaluted::None
                    }
                }
                "b" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        let mut res = vec![];
                        for _ in 0..left {
                            let d: f64 = rng.sample(rand::distributions::OpenClosed01);
                            res.push(Evaluted::Integer((d * right as f64).ceil() as i64));
                        }
                        Evaluted::Array(res)
                    } else {
                        Evaluted::None
                    }
                }
                "d" => {
                    if let (Some(left), Some(right)) = (
                        left.evalute(rng, log).as_integer(),
                        right.evalute(rng, log).as_integer(),
                    ) {
                        let mut res = vec![];
                        for _ in 0..left {
                            let d: f64 = rng.sample(rand::distributions::OpenClosed01);
                            res.push((d * right as f64).ceil() as i64);
                        }
                        log.push(format!("{:?}", &res));

                        let mut sum = 0;
                        for d in res {
                            sum += d;
                        }

                        Evaluted::Integer(sum)
                    } else {
                        Evaluted::None
                    }
                }
                _ => Evaluted::None,
            },
            Self::Term(term) => term.evalute(rng, log),
        }
    }
}

impl ast::Term {
    fn evalute(&self, rng: &mut impl rand::Rng, log: &mut Vec<String>) -> Evaluted {
        match self {
            Self::Expr0(expr) => expr.evalute(rng, log),
            Self::Array(vals) => {
                Evaluted::Array(vals.iter().map(|v| v.evalute(rng, log)).collect())
            }
            Self::Record(vals) => Evaluted::Record(
                vals.iter()
                    .map(|(i, v)| (i.clone(), v.evalute(rng, log)))
                    .collect(),
            ),
            Self::Literal(literal) => literal.evalute(),
        }
    }
}

impl ast::Literal {
    fn evalute(&self) -> Evaluted {
        match self {
            Self::Integer(val) => Evaluted::Integer(*val),
        }
    }
}

impl Evaluted {
    fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(val) => Some(*val),
            _ => None,
        }
    }
}
