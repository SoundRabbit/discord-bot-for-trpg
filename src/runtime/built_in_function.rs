use super::Environment;
use super::Value;
use crate::parser::ast;
use async_std::sync::Arc;

macro_rules! func {
    ($help:ident ; $arg:ident -> $($args:ident ->)+ $implement:block) => {{
        Arc::new(Value::BuiltInFunction {
            help: Arc::clone(&$help),
            implement: Box::new({
                let help = Arc::clone(&$help);
                move |$arg| func!($help; $($args ->)+ $implement)
            }),
        })
    }};

    ($help:ident ; $arg:ident -> $implement:block) => {{
        Arc::new(Value::BuiltInFunction {
            help: Arc::clone(&$help),
            implement: Box::new(move |$arg| $implement),
        })
    }};
}

macro_rules! def_func {
    ($name:literal $help:ident in $env:ident; $($args:ident ->)+ $implement:block) => {{
        let val = func!($help; $($args ->)+ $implement);
        $env.insert(
            Arc::new(ast::Ident::Strict(Arc::new(String::from($name)))),
            val,
        )
        .await;
    }};
}

pub async fn set_default(env: &mut Environment) {
    //max
    let help = Arc::new(String::from(""));
    def_func!("max" help in env ; a -> b -> {
        if let (Some(a), Some(b)) = (a.as_integer(), b.as_integer()) {
            Arc::new(Value::Integer(a.max(b)))
        } else {
            Arc::new(Value::None)
        }
    });

    //max_of
    let help = Arc::new(String::from(""));
    def_func!("max_of" help in env ; a -> b -> {
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

    // min
    let help = Arc::new(String::from(""));
    def_func!("min" help in env ; a -> b -> {
        if let (Some(a), Some(b)) = (a.as_integer(), b.as_integer()) {
            Arc::new(Value::Integer(a.min(b)))
        } else {
            Arc::new(Value::None)
        }
    });

    // min_of
    let help = Arc::new(String::from(""));
    def_func!("min_of" help in env ; a -> b -> {
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

    // s
    let help = Arc::new(String::from(
        "\n\
        s : Array -> Array\n\
        \n\
        ［説明］\n\
        引数として渡された配列を昇順でソートします。\n\
        \n\
        ［使用例］\n\
        10B6.s //10B6を並び替えて表示\n\
        20B6.s>=5 //20B6を並び替えて表示して、更に出目が5以上のダイスを数える",
    ));
    def_func!("s" help in env ; a -> {
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

    //at
}
