use crate::callable::*;
use crate::environment::Environment;
use crate::lexer::*;
use crate::interpreter::*;
use crate::ast::*;
use crate::oop::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

///////////// The Lox STL ///////////////
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct ClockCallable {}

impl ClockCallable {
  pub fn new() -> Self {
    Self {}
  }
}
impl LoxCallable for ClockCallable {
  fn call(&mut self, _interpreter: &mut Interpreter, _arguments: Vec<LoxValue>) -> Box<LoxValue> {
    let time = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .expect("Time went backwards")
      .as_secs_f64();
    Box::new(LoxValue::Number(time))
  }

  fn arity(&self) -> usize {
    0
  }

  fn box_clone(&self) -> Box<dyn LoxCallable> {
    Box::new(ClockCallable::new())
  }
}

#[derive(Clone)]
pub struct LoxFunction {
  pub declaration: Rc<FunStmt>,
  pub closure: Rc<RefCell<Environment>>,
  pub is_initializer: bool,
}

impl fmt::Debug for LoxFunction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Loxfunction {{ name: {:?} }}", self.declaration.name)
  }
}

impl LoxFunction {
  pub fn new(declaration: Rc<FunStmt>, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> Self {
    Self { declaration, closure, is_initializer }
  }

  pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> LoxFunction {
    let mut environment = Environment::new_enclosed(self.closure.clone());
    environment.define("this".to_string(), LoxValue::Instance(instance));
    LoxFunction::new(self.declaration.clone(), Rc::new(RefCell::new(environment)), self.is_initializer)
  }
}

impl LoxCallable for LoxFunction {
  fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Box<LoxValue> {
    let mut environment = Environment::new_enclosed(self.closure.clone());
    for (i, param) in self.declaration.params.iter().enumerate() {
      environment.define(param.token.clone(), arguments[i].clone());
    }
    let result = interpreter.execute_block(&self.declaration.body, Rc::new(RefCell::new(environment)));
    if let Err(e) = result {
      match e.error_type {
        InterpreterErrorType::ReturnValue(value) => {
          if self.is_initializer {
            let this = self.closure.borrow_mut().get_at(0, "this");
            match this {
              Ok(value) => return Box::new(value),
              Err(_) => panic!("Error: 'this' not found in closure."),
            }
          }
          return value;
        }
        _ => panic!("Error in function call: {:?}", e),
      }
    }
    if self.is_initializer {
      let this = self.closure.borrow_mut().get_at(0, "this");
      match this {
        Ok(value) => {
          return Box::new(value);
        }
        Err(_) => panic!("Error: 'this' not found in closure."),
      }
    }
    Box::new(LoxValue::Nil)
  }

  fn arity(&self) -> usize {
    self.declaration.params.len()
  }

  fn box_clone(&self) -> Box<dyn LoxCallable> {
    Box::new(LoxFunction::new(self.declaration.clone(), self.closure.clone(), self.is_initializer))
  }
}
