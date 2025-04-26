use crate::lexer::*;
use core::borrow;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default, Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, LoxValue>,
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

    pub fn get_at(&self, distance: usize, name: &str) -> Result<LoxValue, String> {
        if let Some(enclosing) = self.ancestor(distance) {
            let borrowed_env = enclosing.borrow();
            if borrowed_env.values.contains_key(name) {
                return Ok(borrowed_env.values[name].clone());
            }
        }
        Err(format!("Undefined variable '{}'.", name))
    }

    pub fn assign_at(
        &mut self,
        distance: usize,
        name: String,
        value: LoxValue,
    ) -> Result<(), String> {
        if let Some(enclosing) = self.ancestor(distance) {
            let mut borrowed_env = enclosing.borrow_mut();
            if borrowed_env.values.contains_key(&name) {
                borrowed_env.values.insert(name, value);
                return Ok(());
            }
        }
        Err(format!("Undefined variable '{}'.", name))
    }

    fn ancestor(&self, distance: usize) -> Option<Rc<RefCell<Environment>>> {
        let mut env = self.clone();
        for _ in 0..distance {
            if let Some(enclosing) = env.enclosing.clone() {
                env = enclosing.borrow().clone();
            } else {
                return None;
            }
        }
        Some(Rc::new(RefCell::new(env)))
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
