use crate::parser::ast;
use async_std::sync::Arc;
use std::collections::HashMap;

pub mod built_in_function;
mod environment;

pub use environment::Environment;
pub use environment::Value;

const TIME_LIMIT: u128 = 1000;

macro_rules! check_tle {
    ($t:expr) => {
        if (std::time::Instant::now() - *$t).as_millis() > TIME_LIMIT {
            return Arc::new(Value::Err(format!("TLE (Limit :{} ms)", TIME_LIMIT)));
        }
    };
}

pub enum ValueElement {
    Integer(i64),
    Boolean(bool),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Integer(val) => write!(f, "{}", val),
            Self::Boolean(true) => write!(f, "成功"),
            Self::Boolean(false) => write!(f, "失敗"),
            Self::String(val) => write!(f, "{}", val.as_str()),
            Self::Array(vals) => {
                write!(f, "{}", Self::fmt_array(vals))
            }
            Self::Record(vals) => {
                write!(f, "{:?}", vals)
            }
            Self::Fn { arg, .. } => {
                write!(f, "fn {}", arg.as_str())
            }
            Self::BuiltInFunction { .. } => {
                write!(f, "fn")
            }
            Self::Lazy { .. } => {
                write!(f, "Lazy")
            }
            Self::Err(err) => {
                write!(f, "エラー：{}", err.as_str())
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ast::Proc {
    pub fn evalute(
        &self,
        env: &mut Environment,
        rng: &mut impl rand::Rng,
        log: &mut Vec<String>,
        begin_time: &std::time::Instant,
    ) -> Arc<Value> {
        let mut res = Arc::new(Value::None);
        for expr in self.iter() {
            res = expr.evalute(env, rng, log, &begin_time);
        }
        res
    }
}

impl ast::Expr0 {
    fn evalute(
        &self,
        env: &mut Environment,
        rng: &mut impl rand::Rng,
        log: &mut Vec<String>,
        begin_time: &std::time::Instant,
    ) -> Arc<Value> {
        match self {
            Self::Def { ident, value } => {
                if ident.is_strict() {
                    let value = value.evalute(env, rng, log, begin_time);
                    async_std::task::block_on(env.insert(Arc::clone(ident), value));
                } else {
                    async_std::task::block_on(
                        env.insert(Arc::clone(ident), Arc::new(Value::Lazy(Arc::clone(value)))),
                    );
                }
                Arc::new(Value::None)
            }
            Self::Fn { arg, value } => {
                let env = async_std::task::block_on(env.capture());
                let arg = Arc::clone(arg);
                let value = Arc::clone(value);
                Arc::new(Value::Fn { env, arg, value })
            }
            Self::Expr0 {
                left,
                right,
                operator,
            } => match operator.as_str() {
                "#" => {
                    let right = right.evalute(env, rng, log, begin_time);
                    if let Some(mut right) = right.as_integer() {
                        let mut a = vec![];
                        while right > 0 {
                            check_tle!(begin_time);
                            a.push(left.evalute(env, rng, log, begin_time));
                            right -= 1;
                        }
                        Arc::new(Value::Array(a))
                    } else {
                        Arc::new(Value::None)
                    }
                }
                "@==" => Self::rep_with_op("==", left, right, env, rng, log, begin_time),
                "@!=" => Self::rep_with_op("!=", left, right, env, rng, log, begin_time),
                "@<=" => Self::rep_with_op("<=", left, right, env, rng, log, begin_time),
                "@>=" => Self::rep_with_op(">=", left, right, env, rng, log, begin_time),
                "@<" => Self::rep_with_op("<", left, right, env, rng, log, begin_time),
                "@>" => Self::rep_with_op(">", left, right, env, rng, log, begin_time),
                "@" => {
                    let mut rep = left.evalute(env, rng, log, begin_time);
                    let mut cmp = right.evalute(env, rng, log, begin_time);
                    let mut res = vec![Arc::clone(&rep)];

                    loop {
                        if !Self::operate(
                            " ",
                            Arc::clone(&cmp),
                            Arc::clone(&rep),
                            rng,
                            log,
                            begin_time,
                        )
                        .as_boolean()
                        .unwrap_or(false)
                        {
                            break;
                        }

                        check_tle!(begin_time);

                        rep = left.evalute(env, rng, log, begin_time);
                        cmp = right.evalute(env, rng, log, begin_time);
                        res.push(Arc::clone(&rep));
                    }

                    Arc::new(Value::Array(res))
                }
                op => {
                    let left = left.evalute(env, rng, log, begin_time);
                    let right = right.evalute(env, rng, log, begin_time);
                    Self::operate(op, left, right, rng, log, begin_time)
                }
            },
            Self::Term(term) => term.evalute(env, rng, log, begin_time),
        }
    }

    fn rep_with_op(
        op: &str,
        left: &Self,
        right: &Self,
        env: &mut Environment,
        rng: &mut impl rand::Rng,
        log: &mut Vec<String>,
        begin_time: &std::time::Instant,
    ) -> Arc<Value> {
        let mut rep = left.evalute(env, rng, log, begin_time);
        let mut cmp = right.evalute(env, rng, log, begin_time);
        let mut res = vec![Arc::clone(&rep)];

        while Self::operate(op, Arc::clone(&rep), Arc::clone(&cmp), rng, log, begin_time)
            .as_boolean()
            .unwrap_or(false)
        {
            check_tle!(begin_time);

            rep = left.evalute(env, rng, log, begin_time);
            cmp = right.evalute(env, rng, log, begin_time);
            res.push(Arc::clone(&rep));
        }

        Arc::new(Value::Array(res))
    }

    fn operate(
        op: &str,
        left: Arc<Value>,
        right: Arc<Value>,
        rng: &mut impl rand::Rng,
        log: &mut Vec<String>,
        begin_time: &std::time::Instant,
    ) -> Arc<Value> {
        match op {
            " " => match left.as_ref() {
                Value::Fn { env, arg, value } => {
                    Self::call_fn(right, Arc::clone(arg), value, env, rng, log, begin_time)
                }
                Value::BuiltInFunction { implement, .. } => implement(right),
                _ => Arc::new(Value::None),
            },
            "==" => Self::compare(log, &left, &right, &mut |left, right| {
                if let (Some(left), Some(right)) = (left.as_boolean(), right.as_boolean()) {
                    Value::Boolean(left == right)
                } else if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    Value::Boolean(left == right)
                } else {
                    Value::None
                }
            }),
            "!=" => Self::compare(log, &left, &right, &mut |left, right| {
                if let (Some(left), Some(right)) = (left.as_boolean(), right.as_boolean()) {
                    Value::Boolean(left != right)
                } else if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    Value::Boolean(left != right)
                } else {
                    Value::None
                }
            }),
            "<=" => Self::compare(log, &left, &right, &mut |left, right| {
                if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    Value::Boolean(left <= right)
                } else {
                    Value::None
                }
            }),
            ">=" => Self::compare(log, &left, &right, &mut |left, right| {
                if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    Value::Boolean(left >= right)
                } else {
                    Value::None
                }
            }),
            "<" => Self::compare(log, &left, &right, &mut |left, right| {
                if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    Value::Boolean(left < right)
                } else {
                    Value::None
                }
            }),
            ">" => Self::compare(log, &left, &right, &mut |left, right| {
                if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    Value::Boolean(left > right)
                } else {
                    Value::None
                }
            }),
            "+" => {
                if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    Arc::new(Value::Integer(left + right))
                } else {
                    Arc::new(Value::None)
                }
            }
            "-" => {
                if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    Arc::new(Value::Integer(left - right))
                } else {
                    Arc::new(Value::None)
                }
            }
            "*" => {
                if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    Arc::new(Value::Integer(left * right))
                } else {
                    Arc::new(Value::None)
                }
            }
            "/" => {
                if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    Arc::new(Value::Integer(left / right))
                } else {
                    Arc::new(Value::None)
                }
            }
            "." => match right.as_ref() {
                Value::Fn { env, arg, value } => {
                    Self::call_fn(left, Arc::clone(arg), value, env, rng, log, begin_time)
                }
                Value::BuiltInFunction { implement, .. } => implement(left),
                _ => Arc::new(Value::None),
            },
            "b" => {
                if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    let mut res = vec![];
                    for _ in 0..left {
                        let d: f64 = rng.sample(rand::distributions::OpenClosed01);
                        res.push(Arc::new(Value::Integer((d * right as f64).ceil() as i64)));
                    }

                    log.push(format!("{:?}", &res));

                    Arc::new(Value::Array(res))
                } else {
                    Arc::new(Value::None)
                }
            }
            "d" => {
                if let (Some(left), Some(right)) = (left.as_integer(), right.as_integer()) {
                    let mut res = vec![];
                    for _ in 0..left {
                        let d: f64 = rng.sample(rand::distributions::OpenClosed01);
                        res.push((d * right as f64).ceil() as i64);
                    }

                    let mut sum = 0;
                    for d in &res {
                        sum += *d;
                    }

                    log.push(format!("{} {:?}", sum, &res));

                    Arc::new(Value::Integer(sum))
                } else {
                    Arc::new(Value::None)
                }
            }
            _ => Arc::new(Value::None),
        }
    }

    fn compare(
        log: &mut Vec<String>,
        left: &Value,
        right: &Value,
        operator: &mut impl FnMut(&ValueElement, &ValueElement) -> Value,
    ) -> Arc<Value> {
        if let (Some(left), Some(right)) = (left.as_element(), right.as_element()) {
            Arc::new(operator(&left, &right))
        } else if let Some(left) = left.as_array() {
            log.push(Value::fmt_array(left));
            let value: Vec<Arc<Value>> = left
                .iter()
                .map(|item| Self::compare(log, item, right, operator))
                .collect();
            Arc::new(Value::Array(value))
        } else if let Some(left) = left.as_record() {
            let value: HashMap<Arc<String>, Arc<Value>> = left
                .iter()
                .map(|(key, item)| (Arc::clone(key), Self::compare(log, item, right, operator)))
                .collect();
            Arc::new(Value::Record(value))
        } else {
            Arc::new(Value::None)
        }
    }

    fn call_fn(
        argv: Arc<Value>,
        arg: Arc<String>,
        value: &ast::Expr0,
        scoped_env: &Environment,
        rng: &mut impl rand::Rng,
        log: &mut Vec<String>,
        begin_time: &std::time::Instant,
    ) -> Arc<Value> {
        check_tle!(begin_time);
        let mut scoped_env = async_std::task::block_on(scoped_env.capture());
        async_std::task::block_on(scoped_env.insert(Arc::new(ast::Ident::Strict(arg)), argv));
        let val = value.evalute(&mut scoped_env, rng, log, begin_time);
        async_std::task::block_on(scoped_env.free());
        val
    }
}

impl ast::Term {
    fn evalute(
        &self,
        env: &mut Environment,
        rng: &mut impl rand::Rng,
        log: &mut Vec<String>,
        begin_time: &std::time::Instant,
    ) -> Arc<Value> {
        match self {
            Self::Expr0(expr) => expr.evalute(env, rng, log, begin_time),
            Self::Proc(proc) => {
                let mut scoped_env = async_std::task::block_on(env.capture());
                let val = proc.evalute(&mut scoped_env, rng, log, begin_time);
                async_std::task::block_on(scoped_env.free());
                val
            }
            Self::Array(vals) => Arc::new(Value::Array(
                vals.iter()
                    .map(|v| v.evalute(env, rng, log, begin_time))
                    .collect(),
            )),
            Self::Record(vals) => Arc::new(Value::Record(
                vals.iter()
                    .map(|(i, v)| (Arc::clone(i), v.evalute(env, rng, log, begin_time)))
                    .collect(),
            )),
            Self::Literal(literal) => literal.evalute(env, rng, log, begin_time),
        }
    }
}

impl ast::Literal {
    fn evalute(
        &self,
        env: &mut Environment,
        rng: &mut impl rand::Rng,
        log: &mut Vec<String>,
        begin_time: &std::time::Instant,
    ) -> Arc<Value> {
        match self {
            Self::Integer(val) => Arc::new(Value::Integer(*val)),
            Self::Ident(ident) => {
                let val =
                    async_std::task::block_on(env.get(&ident)).unwrap_or(Arc::new(Value::None));
                if let Value::Lazy(expr) = val.as_ref() {
                    expr.evalute(env, rng, log, begin_time)
                } else {
                    val
                }
            }
        }
    }
}

impl Value {
    fn is_boolean(&self) -> bool {
        match self {
            Self::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            Self::Err(_) => true,
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

    fn as_array(&self) -> Option<&Vec<Arc<Value>>> {
        match self {
            Self::Array(val) => Some(val),
            _ => None,
        }
    }

    fn as_record(&self) -> Option<&HashMap<Arc<String>, Arc<Value>>> {
        match self {
            Self::Record(val) => Some(&val),
            _ => None,
        }
    }

    fn as_element(&self) -> Option<ValueElement> {
        match self {
            Self::Integer(val) => Some(ValueElement::Integer(*val)),
            Self::Boolean(val) => Some(ValueElement::Boolean(*val)),
            _ => None,
        }
    }

    fn fmt_array(vals: &Vec<Arc<Value>>) -> String {
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

impl ValueElement {
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
