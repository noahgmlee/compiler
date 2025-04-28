use crate::callable::*;
use crate::interpreter::*;
use crate::lexer::*;
use crate::stl::LoxFunction;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct LoxClass {
  pub name: String,
  pub methods: std::collections::HashMap<String, LoxFunction>,
}

impl PartialEq for LoxClass {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

impl fmt::Display for LoxClass {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl LoxClass {
  pub fn new(name: String, methods: HashMap<String, LoxFunction> ) -> Self {
    Self { name, methods }
  }
}

#[derive(Clone, PartialEq)]
pub struct LoxInstance {
  pub class: LoxClass,
  pub properties: std::collections::HashMap<String, LoxValue>,
}

impl fmt::Debug for LoxInstance {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "LoxInstance {{ class: {:?} }}", self.class.name)
  }
}

impl fmt::Display for LoxInstance {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.class.name.clone() + " instance")
  }
}

impl LoxInstance {
  pub fn new(class: LoxClass) -> Self {
    Self {
      class: class.clone(),
      properties: std::collections::HashMap::new(),
    }
  }

  // Ew
  pub fn get(this: Rc<RefCell<LoxInstance>>, name: &str) -> Result<LoxValue, String> {
    if let Some(value) = this.borrow().properties.get(name) {
      return Ok(value.clone());
    }

    if let Some(method) = this.borrow().class.methods.get(name) {
      return Ok(LoxValue::Callable(Rc::new(RefCell::new(Box::new(method.clone().bind(this.clone()))))));
    }

    Err(format!("Undefined property '{}'.", name))
  }

  pub fn set(&mut self, name: String, value: LoxValue) {
    self.properties.insert(name, value);
  }
}

impl LoxCallable for LoxClass {
  fn call(&mut self, _interpreter: &mut Interpreter, _arguments: Vec<LoxValue>) -> Box<LoxValue> {
    let instance = Rc::new(RefCell::new(LoxInstance::new(self.clone())));
    if let Some(method) = self.methods.get("init") {
      method.bind(instance.clone()).call(_interpreter, _arguments);
    }
    Box::new(LoxValue::Instance(instance))
  }

  fn arity(&self) -> usize {
    if let Some(method) = self.methods.get("init") {
      return method.arity();
    }
    0
  }

  fn box_clone(&self) -> Box<dyn LoxCallable> {
    Box::new(self.clone())
  }
}
