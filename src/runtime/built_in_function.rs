use super::Environment;
use super::Value;
use async_std::sync::Arc;

macro_rules! func {
    ($help:ident ; $arg:ident => $($args:ident =>)+ $implement:block) => {{
        Arc::new(Value::BuiltInFunction {
            help: Arc::clone(&$help),
            implement: Box::new(move |$arg| func!($help; $($args =>)+ $implement)),
        })
    }};

    ($help:ident ; $arg:ident => $implement:block) => {{
        Arc::new(Value::BuiltInFunction {
            help: Arc::clone(&$help),
            implement: Box::new(move |$arg| $implement),
        })
    }};
}

pub async fn set_default(env: &mut Environment) {
    // max
    let help = Arc::new(String::from(""));
    let val = func!(help ; a => b => {
        if let (Some(a), Some(b)) = (a.as_integer(), b.as_integer()) {
            Arc::new(Value::Integer(a.max(b)))
        } else {
            Arc::new(Value::None)
        }
    });
    env.insert(Arc::new(String::from("max")), val).await;

    // max_of
    let help = Arc::new(String::from(""));
    let val = func!(help ; a => b => {
        if let (Some(a), Some(b)) = (a.as_integer(), b.as_array()) {
            let bi: Vec<_> = b.iter().filter_map(|x| x.as_integer()).collect();
            if b.len() == bi.len() && a > 0{
                let mut bi: Vec<_> = bi.into_iter().enumerate().collect();
                bi.sort_by(|x, y| if x.1 != y.1 { y.1.cmp(&x.1) } else { x.0.cmp(&y.0) });
                let mut bi: Vec<_> = bi.drain(0..(a as usize)).collect();
                bi.sort_by(|x, y| x.0.cmp(&y.0));
                let bi = bi.into_iter().map(|(_, x)| Arc::new(Value::Integer(x))).collect();
                return Arc::new(Value::Array(bi));
            }
        }
        Arc::new(Value::None)
    });
    env.insert(Arc::new(String::from("max_of")), val).await;

    // min
    let help = Arc::new(String::from(""));
    let val = func!(help ; a => b => {
        if let (Some(a), Some(b)) = (a.as_integer(), b.as_integer()) {
            Arc::new(Value::Integer(a.min(b)))
        } else {
            Arc::new(Value::None)
        }
    });
    env.insert(Arc::new(String::from("min")), val).await;

    // min_of
    let help = Arc::new(String::from(""));
    let val = func!(help ; a => b => {
        if let (Some(a), Some(b)) = (a.as_integer(), b.as_array()) {
            let bi: Vec<_> = b.iter().filter_map(|x| x.as_integer()).collect();
            if b.len() == bi.len() && a > 0{
                let mut bi: Vec<_> = bi.into_iter().enumerate().collect();
                bi.sort_by(|x, y| if x.1 != y.1 { x.1.cmp(&y.1) } else { x.0.cmp(&y.0) });
                let mut bi: Vec<_> = bi.drain(0..(a as usize)).collect();
                bi.sort_by(|x, y| x.0.cmp(&y.0));
                let bi = bi.into_iter().map(|(_, x)| Arc::new(Value::Integer(x))).collect();
                return Arc::new(Value::Array(bi));
            }
        }
        Arc::new(Value::None)
    });
    env.insert(Arc::new(String::from("min_of")), val).await;

    // s
    let help = Arc::new(String::from(""));
    let val = func!(help ; a => {
        if let Some(a) = a.as_array() {
            let mut ai: Vec<_> = a.iter().filter_map(|x| x.as_integer()).collect();
            if a.len() == ai.len(){
                ai.sort_by(|x, y| x.cmp(&y));
                let ai = ai.into_iter().map(|x| Arc::new(Value::Integer(x))).collect();
                return Arc::new(Value::Array(ai));
            }
        }
        Arc::new(Value::None)
    });
    env.insert(Arc::new(String::from("s")), val).await;
}
