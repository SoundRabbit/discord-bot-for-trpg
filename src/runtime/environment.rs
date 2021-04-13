use crate::parser::ast;
use async_std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct Environment {
    vars: Arc<Mutex<Vec<Var>>>,
    head: usize,
}

struct Var {
    parent: usize,
    ident: Arc<String>,
    val: Arc<Value>,
}

pub enum Value {
    None,
    Integer(i64),
    Boolean(bool),
    String(Arc<String>),
    Array(Vec<Arc<Value>>),
    Record(HashMap<Arc<String>, Arc<Value>>),
    Fn {
        env: Environment,
        arg: Arc<String>,
        value: Arc<ast::Expr0>,
    },
    BuiltInFunction {
        help: Arc<String>,
        implement: Box<dyn Fn(Arc<Value>) -> Arc<Value>>,
    },
}

impl Environment {
    fn build_in_help(val: Arc<Value>) -> Arc<Value> {
        if let Value::BuiltInFunction { help, .. } = val.as_ref() {
            Arc::new(Value::String(Arc::clone(help)))
        } else {
            Arc::new(Value::None)
        }
    }

    pub fn new() -> Self {
        let mut this = Self {
            vars: Arc::new(Mutex::new(vec![])),
            head: 0,
        };
        async_std::task::block_on(this.append_build_in_function(
            Arc::new(String::from("help")),
            Arc::new(String::from(r#""#)),
            Self::build_in_help,
        ));
        this
    }

    pub fn capture(&self) -> Self {
        Self {
            vars: Arc::clone(&self.vars),
            head: self.head,
        }
    }

    pub async fn insert(&mut self, ident: Arc<String>, val: Arc<Value>) {
        let var = Var {
            parent: self.head,
            ident: ident,
            val: val,
        };

        {
            let mut vars = self.vars.lock().await;
            vars.push(var);
            self.head = vars.len();
        }
    }

    pub async fn get(&self, ident: &String) -> Option<Arc<Value>> {
        let mut idx = self.head;
        while idx > 0 {
            if let Some(var) = self.vars.lock().await.get(idx - 1) {
                if *(var.ident) == *ident {
                    return Some(Arc::clone(&var.val));
                } else {
                    idx = var.parent;
                }
            } else {
                break;
            }
        }
        None
    }

    pub async fn append_build_in_function(
        &mut self,
        name: Arc<String>,
        help: Arc<String>,
        implement: impl Fn(Arc<Value>) -> Arc<Value> + 'static,
    ) {
        let val = Value::BuiltInFunction {
            help,
            implement: Box::new(implement),
        };

        self.insert(name, Arc::new(val)).await
    }
}
