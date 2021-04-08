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

pub enum EvalutedElement {
    Integer(i64),
    Boolean(bool),
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
            Self::Boolean(true) => write!(f, "成功"),
            Self::Boolean(false) => write!(f, "失敗"),
            Self::Array(vals) => {
                write!(f, "{}", Self::fmt_array(vals))
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
                    let left = left.evalute(rng, log);
                    let right = right.evalute(rng, log);
                    Self::compare(log, &left, &right, &mut |left, right| {
                        if let (Some(left), Some(right)) = (left.as_boolean(), right.as_boolean()) {
                            Evaluted::Boolean(left == right)
                        } else if let (Some(left), Some(right)) =
                            (left.as_integer(), right.as_integer())
                        {
                            Evaluted::Boolean(left == right)
                        } else {
                            Evaluted::None
                        }
                    })
                }
                "!=" => {
                    let left = left.evalute(rng, log);
                    let right = right.evalute(rng, log);
                    Self::compare(log, &left, &right, &mut |left, right| {
                        if let (Some(left), Some(right)) = (left.as_boolean(), right.as_boolean()) {
                            Evaluted::Boolean(left != right)
                        } else if let (Some(left), Some(right)) =
                            (left.as_integer(), right.as_integer())
                        {
                            Evaluted::Boolean(left != right)
                        } else {
                            Evaluted::None
                        }
                    })
                }
                "<=" => {
                    let left = left.evalute(rng, log);
                    let right = right.evalute(rng, log);
                    Self::compare(log, &left, &right, &mut |left, right| {
                        if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                            Evaluted::Boolean(left <= right)
                        } else {
                            Evaluted::None
                        }
                    })
                }
                ">=" => {
                    let left = left.evalute(rng, log);
                    let right = right.evalute(rng, log);
                    Self::compare(log, &left, &right, &mut |left, right| {
                        if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                            Evaluted::Boolean(left >= right)
                        } else {
                            Evaluted::None
                        }
                    })
                }
                "<" => {
                    let left = left.evalute(rng, log);
                    let right = right.evalute(rng, log);
                    Self::compare(log, &left, &right, &mut |left, right| {
                        if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                            Evaluted::Boolean(left < right)
                        } else {
                            Evaluted::None
                        }
                    })
                }
                ">" => {
                    let left = left.evalute(rng, log);
                    let right = right.evalute(rng, log);
                    Self::compare(log, &left, &right, &mut |left, right| {
                        if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                            Evaluted::Boolean(left > right)
                        } else {
                            Evaluted::None
                        }
                    })
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

    fn compare(
        log: &mut Vec<String>,
        left: &Evaluted,
        right: &Evaluted,
        operator: &mut impl FnMut(&EvalutedElement, &EvalutedElement) -> Evaluted,
    ) -> Evaluted {
        if let (Some(left), Some(right)) = (left.as_element(), right.as_element()) {
            operator(&left, &right)
        } else if let Some(left) = left.as_array() {
            log.push(Evaluted::fmt_array(left));
            let evaluted: Vec<Evaluted> = left
                .iter()
                .map(|item| Self::compare(log, item, right, operator))
                .collect();
            Evaluted::Array(evaluted)
        } else if let Some(left) = left.as_record() {
            let evaluted: HashMap<String, Evaluted> = left
                .iter()
                .map(|(key, item)| (key.clone(), Self::compare(log, item, right, operator)))
                .collect();
            Evaluted::Record(evaluted)
        } else {
            Evaluted::None
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
    fn is_boolean(&self) -> bool {
        match self {
            Self::Boolean(_) => true,
            _ => false,
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(val) => Some(*val),
            _ => None,
        }
    }

    fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(val) => Some(*val),
            _ => None,
        }
    }

    fn as_array(&self) -> Option<&Vec<Evaluted>> {
        match self {
            Self::Array(val) => Some(val),
            _ => None,
        }
    }

    fn as_record(&self) -> Option<&HashMap<String, Evaluted>> {
        match self {
            Self::Record(val) => Some(&val),
            _ => None,
        }
    }

    fn as_element(&self) -> Option<EvalutedElement> {
        match self {
            Self::Integer(val) => Some(EvalutedElement::Integer(*val)),
            Self::Boolean(val) => Some(EvalutedElement::Boolean(*val)),
            _ => None,
        }
    }

    fn fmt_array(vals: &Vec<Evaluted>) -> String {
        if vals.iter().all(|val| val.is_boolean()) {
            let hit_num = vals
                .iter()
                .filter(|val| val.as_boolean().unwrap_or(false))
                .count();
            format!("{}成功", hit_num)
        } else {
            format!("{:?}", vals)
        }
    }
}

impl EvalutedElement {
    fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(val) => Some(*val),
            _ => None,
        }
    }

    fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(val) => Some(*val),
            _ => None,
        }
    }
}
