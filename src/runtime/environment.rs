use crate::parser::ast;
use async_std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct Environment {
    binds: Arc<Mutex<Vec<Option<Bind>>>>,
    head: usize,
    root: usize,
}

struct Bind {
    parent: usize,
    ident: Arc<String>,
    val: Arc<Value>,
    rc: usize,
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
    Err(String),
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
            binds: Arc::new(Mutex::new(vec![])),
            head: 0,
            root: 0,
        };
        async_std::task::block_on(this.append_build_in_function(
            Arc::new(String::from("help")),
            Arc::new(String::from(r#""#)),
            Self::build_in_help,
        ));
        this
    }

    pub async fn capture(&self) -> Self {
        if let Some(bind) = self
            .binds
            .lock()
            .await
            .get_mut(self.head - 1)
            .and_then(Option::as_mut)
        {
            bind.rc += 1;
        }

        Self {
            binds: Arc::clone(&self.binds),
            head: self.head,
            root: self.head,
        }
    }

    pub async fn insert(&mut self, ident: Arc<String>, val: Arc<Value>) {
        let bind = Bind {
            parent: self.head,
            ident: ident,
            val: val,
            rc: 0,
        };

        {
            let mut binds = self.binds.lock().await;

            if let Some(idx) = binds.iter().position(|x| x.is_none()) {
                binds[idx] = Some(bind);
                self.head = idx + 1;
            } else {
                binds.push(Some(bind));
                self.head = binds.len();
            }
        }
    }

    pub async fn get(&self, ident: &String) -> Option<Arc<Value>> {
        let mut idx = self.head;
        while idx > 0 {
            if let Some(bind) = self
                .binds
                .lock()
                .await
                .get(idx - 1)
                .and_then(Option::as_ref)
            {
                if *(bind.ident) == *ident {
                    return Some(Arc::clone(&bind.val));
                } else {
                    idx = bind.parent;
                }
            } else {
                break;
            }
        }
        None
    }

    pub async fn free(&self) {
        let mut idx = self.head;
        let mut binds = self.binds.lock().await;
        while idx >= self.root && idx > 0 {
            let parent = if let Some(bind) = binds.get_mut(idx - 1).and_then(Option::as_mut) {
                if bind.rc > 0 {
                    bind.rc -= 1;
                    break;
                } else {
                    bind.parent
                }
            } else {
                break;
            };

            binds[idx - 1] = None;
            idx = parent;
        }
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
