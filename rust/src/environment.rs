use crate::lexer::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default, Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<LoxValue, String> {
        if self.values.contains_key(name) {
            return Ok(self.values[name].clone());
        }

        if let Some(enclosing) = &self.enclosing {
            if let Ok(value) = enclosing.borrow().get(name) {
                return Ok(value);
            }
        }
        return Err(format!("Undefined variable '{}'.", name));
    }

    pub fn assign(&mut self, name: String, value: LoxValue) -> Result<(), String> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            if let Ok(_) = enclosing.borrow_mut().assign(name.clone(), value) {
                return Ok(());
            }
        }

        Err(format!("Undefined variable '{}'.", name))
    }
}
