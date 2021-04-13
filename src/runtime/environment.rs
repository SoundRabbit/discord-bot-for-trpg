use super::Value;
use async_std::sync::{Arc, Mutex};

struct Var {
    parent: usize,
    ident: Arc<String>,
    val: Arc<Value>,
}

pub struct Environment {
    vars: Arc<Mutex<Vec<Var>>>,
    head: usize,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            vars: Arc::new(Mutex::new(vec![])),
            head: 0,
        }
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
}
